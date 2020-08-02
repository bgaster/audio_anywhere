//! 
//! Wasmtime implementation of AAUnit
//! Copyright: Benedict R. Gaster
#![allow(dead_code)]

use std::sync::mpsc::{Sender};
use std::iter;
use wasmtime::*;

use vst::editor::Editor;
use vst::plugin::{Category, Info};

use crate::messages::*;
use crate::utils::*;

//------------------------------------------------------------------------------

/// symbol for input buffer 0 in WASM module
const IN_BUFFER0: &str = "IN_BUFFER0";
/// symbol for input buffer 1 in WASM module
const IN_BUFFER1: &str = "IN_BUFFER1";
/// symbol for output buffer 0 in WASM module
const OUT_BUFFER0: &str = "OUT_BUFFER0";
/// symbol for output buffer 1 in WASM module
const OUT_BUFFER1: &str = "OUT_BUFFER1";

//------------------------------------------------------------------------------

/// Representation of an Audio Anywhere Module or Unit
pub struct AAUnit {
    pub info: Info,
    /// wasmtime instance for single module
    #[allow(dead_code)]
    instance: Instance,
    /// wasmtime memory segment
    memory: Memory,
    /// module init function
    init: Func,
    /// get current sample rate from WASM
    get_sample_rate: Func,
    /// get number of inputs channels for current module
    get_num_inputs: Func,
    /// get number of outputs channels for current module
    get_num_outputs: Func,
    /// set float parameter in module
    set_param_float: Func,
    /// set int parameter in module
    set_param_int: Func,
    /// get float parameter in module
    get_param_float: Func,
    /// get int parameter in module
    get_param_int: Func,
    /// compute function for module module
    compute: Func,
    /// offset in WASM linear memory to input buffer 0 (max buffer size 1024)
    in_buffer0_offset: usize,
    /// offset in WASM linear memory to input buffer 1 (max buffer size 1024)
    in_buffer1_offset: usize,
    /// offset in WASM linear memory to output buffer 0 (max buffer size 1024)
    out_buffer0_offset: usize,
    /// offset in WASM linear memory to output buffer 1 (max buffer size 1024)
    out_buffer1_offset: usize,
    //out_buffer1_offset: usize,
    pub send: Sender<(Index, Value)>,
    //pub outgoing: Sender<Value>,
}

impl AAUnit {
    pub fn new(
        send: Sender<(Index, Value)>, 
        wasm_bytes: &[u8]) -> Result<Self> {

        let engine = Engine::new(Config::new().wasm_simd(true));
        let store = Store::new(&engine);

        // add error handling
        let module = Module::new(store.engine(), wasm_bytes).unwrap();
        let instance = Instance::new(&store, &module, &[]).unwrap();
        let init = instance.get_func("init").unwrap();
        let get_sample_rate: Func = instance.get_func("get_sample_rate").unwrap();
        let get_num_inputs: Func = instance.get_func("get_num_input_channels").unwrap();
        let get_num_outputs: Func = instance.get_func("get_num_output_channels").unwrap();
        let set_param_float: Func = instance.get_func("set_param_float").unwrap();
        let set_param_int: Func = instance.get_func("set_param_int").unwrap();
        let get_param_float: Func = instance.get_func("get_param_float").unwrap();
        let get_param_int: Func = instance.get_func("get_param_int").unwrap();
        let compute = instance.get_func("compute").unwrap();
    
        // configure offsets for global audio channel symbols
        let in_buffer0_offset = Self::get_global_symbol_offset(IN_BUFFER0, &instance);
        let in_buffer1_offset = Self::get_global_symbol_offset(IN_BUFFER1, &instance);
        
        let out_buffer0_offset = Self::get_global_symbol_offset(OUT_BUFFER0, &instance);
        let out_buffer1_offset = Self::get_global_symbol_offset(OUT_BUFFER1, &instance);

        // handle to WASM linear memory
        let memory = instance.get_memory("memory").unwrap();

        let info = Info {
            name: "GAIN".to_string(),
            vendor: "_cuberoo".to_string(),
            unique_id: 9614,
            category: Category::Synth,
            inputs: 1,
            outputs: 1,
            parameters: 0,
            initial_delay: 0,
            f64_precision: false,
            ..Info::default()
        };

        Ok(Self {
            info,
            instance,
            memory,
            init,
            get_sample_rate,
            get_num_inputs,
            get_num_outputs,
            set_param_float,
            set_param_int,
            get_param_float,
            get_param_int,
            compute,
            in_buffer0_offset,
            in_buffer1_offset,
            out_buffer0_offset,
            out_buffer1_offset,
            send,
        })
    }

    /// get offest for symbol in WASM linear memory, if not defined returns 0
    /// of course, in theory 0 is a valid offset, so it is important to check this elsewhere
    fn get_global_symbol_offset(symbol: &str, instance: &Instance) -> usize {
        instance.get_global(symbol).map_or(0, |offset| {
            if let Val::I32(o) = offset.get() {
                o as usize
            }
            else {
                0
            }
        })
    }

    /// set a float parameter
    #[inline]
    pub fn set_param_float(&self, index: u32, param: f32) {
        let f = self.set_param_float.get2::<u32, f32, ()>().unwrap();
        f(index, param).unwrap();
    }

    /// set an int parameter
    #[inline]
    pub fn set_param_int(&self, index: u32, param: i32) {
        let f = self.set_param_int.get2::<u32, i32, ()>().unwrap();
        f(index, param).unwrap();
    }

    #[inline]
    pub fn get_param_float(&self, index: u32) -> f32 {
        let f = self.get_param_float.get1::<u32, f32>().unwrap();
        f(index).unwrap()
    }

    #[inline]
    pub fn get_param_int(&self, index: u32) -> i32 {
        let f = self.get_param_int.get1::<u32, i32>().unwrap();
        f(index).unwrap()
    }

    #[inline]
    pub fn get_number_inputs(&self) -> i32 {
        let f = self.get_num_inputs.get0::<i32>().unwrap();
        f().unwrap()
    }

    #[inline]
    pub fn get_number_outputs(&self) -> i32 {
        let f = self.get_num_outputs.get0::<i32>().unwrap();
        f().unwrap()
    }

    /// compute audio for 1 input and 1 output channels
    #[inline]
    pub fn compute_one_one(&self, frames: usize, inputs: &[f32], outputs: &mut [f32]) {
        // setup and copy input audio
        let inputs0 = inputs[0..frames as usize].iter();
        let wasm_inputs0: &mut [f32] = unsafe { 
            let bytes = 
                &mut self.memory.data_unchecked_mut()[self.in_buffer0_offset..self.in_buffer0_offset 
                                                      + (frames*std::mem::size_of::<f32>())];
            std::mem::transmute(bytes)
        };

        let zipped_inputs = inputs0.zip(wasm_inputs0);
        for (input, wasm_input) in zipped_inputs {
            *wasm_input = *input;
        }

        // now call compute
        let compute = self.compute.get1::<u32, ()>().unwrap();
        compute(frames as u32).unwrap();

        // setup and copy audio out of WASM
        let outputs0 = outputs[0..frames as usize].iter_mut();
        let wasm_outputs0: &[f32] = unsafe { 
            let bytes = 
                &self.memory.data_unchecked()[self.out_buffer0_offset..self.out_buffer0_offset + 
                                              (frames*std::mem::size_of::<f32>())];
            std::mem::transmute(bytes)
        };

        let zipper_outputs = outputs0.zip(wasm_outputs0);
        for (output, wasm_output) in zipper_outputs {
            *output = *wasm_output;
        }
    }

    /// compute audio for 1 input and 2 outputs channels
    /// assume that output channels are interlaced
    #[inline]
    pub fn compute_one_two(&self, frames: usize, inputs: &[f32], outputs: &mut [f32]) {
        // setup and copy input audio
        let inputs0 = inputs[0..frames as usize].iter();
        let wasm_inputs0: &mut [f32] = unsafe { 
            let bytes = 
                &mut self.memory.data_unchecked_mut()[self.in_buffer0_offset..self.in_buffer0_offset 
                                                      + (frames*std::mem::size_of::<f32>())];
            std::mem::transmute(bytes)
        };

        let zipped_inputs = inputs0.zip(wasm_inputs0);
        for (input, wasm_input) in zipped_inputs {
            *wasm_input = *input;
        }

        // now call compute
        let compute = self.compute.get1::<u32, ()>().unwrap();
        compute(frames as u32).unwrap();

        // setup and copy audio out of WASM
        // output is assumed to be interlaced
        let outputs0 = outputs[0..2 * frames as usize].iter_mut();
        let (wasm_outputs0, wasm_outputs1): (&[f32],&[f32]) = unsafe { 
            let bytes0 = 
                &self.memory.data_unchecked()[self.out_buffer0_offset..self.out_buffer0_offset + 
                                              (frames*std::mem::size_of::<f32>())];
            let bytes1 = 
                &self.memory.data_unchecked()[self.out_buffer1_offset..self.out_buffer1_offset + 
                                              (frames*std::mem::size_of::<f32>())];
            (std::mem::transmute(bytes0), std::mem::transmute(bytes1))
        };

        // collect outputs from WASM so they are interlaced
        let collected_wasm_outputs = wasm_outputs0
            .iter()
            .zip(wasm_outputs1)
            .flat_map(|(x, y)| iter::once(x).chain(iter::once(y))); 

        let zipper_outputs = outputs0.zip(collected_wasm_outputs);
        for (output, wasm_output) in zipper_outputs {
            *output = *wasm_output;
        }
    }

    /// compute audio for 1 input and 1 output channels
    #[inline]
    pub fn compute_two_one(&self, frames: usize, inputs: &[f32], outputs: &mut [f32]) {
        // setup and copy input audio
        let inputs0 = inputs[0..frames as usize].iter();
        let (wasm_inputs0, wasm_inputs1): (&mut [f32],&mut [f32]) = unsafe { 
            let bytes0 = 
                &mut self.memory.data_unchecked_mut()[self.in_buffer0_offset..self.in_buffer0_offset 
                                                      + (frames*std::mem::size_of::<f32>())];
            let bytes1 = 
                &mut self.memory.data_unchecked_mut()[self.in_buffer0_offset..self.in_buffer0_offset 
                                                      + (frames*std::mem::size_of::<f32>())];
            (std::mem::transmute(bytes0), std::mem::transmute(bytes1))
        };

        let collected_wasm_outputs = wasm_inputs0
            .iter_mut()
            .zip(wasm_inputs1)
            .flat_map(|(x, y)| iter::once(x).chain(iter::once(y)));

        let zipped_inputs = inputs0.zip(collected_wasm_outputs);
        for (input, wasm_input) in zipped_inputs {
            *wasm_input = *input;
        }

        // now call compute
        let compute = self.compute.get1::<u32, ()>().unwrap();
        compute(frames as u32).unwrap();

        // setup and copy audio out of WASM
        let outputs0 = outputs[0..frames as usize].iter_mut();
        let wasm_outputs0: &[f32] = unsafe { 
            let bytes = 
                &self.memory.data_unchecked()[self.out_buffer0_offset..self.out_buffer0_offset + 
                                              (frames*std::mem::size_of::<f32>())];
            std::mem::transmute(bytes)
        };

        let zipper_outputs = outputs0.zip(wasm_outputs0);
        for (output, wasm_output) in zipper_outputs {
            *output = *wasm_output;
        }
    }


    /// compute audio for 2 input and 2 outputs channels
    /// assume that output channels are interlaced
    #[inline]
    pub fn compute_two_two(&self, frames: usize, inputs: &[f32], outputs: &mut [f32]) {
        // setup and copy input audio
        let inputs0 = inputs[0..frames as usize].iter();
        let (wasm_inputs0, wasm_inputs1): (&mut [f32],&mut [f32]) = unsafe { 
            let bytes0 = 
                &mut self.memory.data_unchecked_mut()[self.in_buffer0_offset..self.in_buffer0_offset 
                                                      + (frames*std::mem::size_of::<f32>())];
            let bytes1 = 
                &mut self.memory.data_unchecked_mut()[self.in_buffer0_offset..self.in_buffer0_offset 
                                                      + (frames*std::mem::size_of::<f32>())];
            (std::mem::transmute(bytes0), std::mem::transmute(bytes1))
        };

        let collected_wasm_outputs = wasm_inputs0
            .iter_mut()
            .zip(wasm_inputs1)
            .flat_map(|(x, y)| iter::once(x).chain(iter::once(y)));

        let zipped_inputs = inputs0.zip(collected_wasm_outputs);
        for (input, wasm_input) in zipped_inputs {
            *wasm_input = *input;
        }

        // now call compute
        let compute = self.compute.get1::<u32, ()>().unwrap();
        compute(frames as u32).unwrap();

        // setup and copy audio out of WASM
        // output is assumed to be interlaced
        let outputs0 = outputs[0..2 * frames as usize].iter_mut();
        let (wasm_outputs0, wasm_outputs1): (&[f32],&[f32]) = unsafe { 
            let bytes0 = 
                &self.memory.data_unchecked()[self.out_buffer0_offset..self.out_buffer0_offset + 
                                              (frames*std::mem::size_of::<f32>())];
            let bytes1 = 
                &self.memory.data_unchecked()[self.out_buffer1_offset..self.out_buffer1_offset + 
                                              (frames*std::mem::size_of::<f32>())];
            (std::mem::transmute(bytes0), std::mem::transmute(bytes1))
        };

        // collect outputs from WASM so they are interlaced
        let collected_wasm_outputs = wasm_outputs0
            .iter()
            .zip(wasm_outputs1)
            .flat_map(|(x, y)| iter::once(x).chain(iter::once(y))); 

        let zipper_outputs = outputs0.zip(collected_wasm_outputs);
        for (output, wasm_output) in zipper_outputs {
            *output = *wasm_output;
        }
    }

    /// compute audio for 1 input and 2 outputs channels
    /// assume that output channels are interlaced
    #[inline]
    pub fn compute_zero_two(&self, frames: usize, outputs: &mut [f32]) {
    
        // now call compute
        let compute = self.compute.get1::<u32, ()>().unwrap();
        compute(frames as u32).unwrap();

        // setup and copy audio out of WASM
        // output is assumed to be interlaced
        let outputs0 = outputs[0..2 * frames as usize].iter_mut();
        let (wasm_outputs0, wasm_outputs1): (&[f32],&[f32]) = unsafe { 
            let bytes0 = 
                &self.memory.data_unchecked()[self.out_buffer0_offset..self.out_buffer0_offset + 
                                              (frames*std::mem::size_of::<f32>())];
            let bytes1 = 
                &self.memory.data_unchecked()[self.out_buffer1_offset..self.out_buffer1_offset + 
                                              (frames*std::mem::size_of::<f32>())];
            (std::mem::transmute(bytes0), std::mem::transmute(bytes1))
        };

        // collect outputs from WASM so they are interlaced
        let collected_wasm_outputs = wasm_outputs0
            .iter()
            .zip(wasm_outputs1)
            .flat_map(|(x, y)| iter::once(x).chain(iter::once(y))); 

        let zipper_outputs = outputs0.zip(collected_wasm_outputs);
        for (output, wasm_output) in zipper_outputs {
            *output = *wasm_output;
        }
    }

    /// compute audio for 0 input and 1 output channels
    #[inline]
    pub fn compute_zero_one(&self, frames: usize, outputs: &mut [f32]) {
        // now call compute
        let compute = self.compute.get1::<u32, ()>().unwrap();
        compute(frames as u32).unwrap();

        // setup and copy audio out of WASM
        let outputs0 = outputs[0..frames as usize].iter_mut();
        let wasm_outputs0: &[f32] = unsafe { 
            let bytes = 
                &self.memory.data_unchecked()[self.out_buffer0_offset..self.out_buffer0_offset 
                                              + (frames*std::mem::size_of::<f32>())];
            std::mem::transmute(bytes)
        };

        let zipper_outputs = outputs0.zip(wasm_outputs0);
        for (output, wasm_output) in zipper_outputs {
            *output = *wasm_output;
        }
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        // let gui_html = Asset::get("gui.html").unwrap();

        // let gui = vst_gui::new_plugin_gui(
        //     String::from_utf8(gui_html.as_ref().to_vec()).unwrap(), //String::from(gui_html),
        //     create_sine_javascript_callback(self.outgoing.clone()),
        //     Some((480, 320)),
        //     ());
        // Some(Box::new(gui))
        None
    }
}