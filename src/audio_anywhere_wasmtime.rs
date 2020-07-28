use std::sync::mpsc::{channel, Sender, Receiver};

use wasmtime::*;

use vst::editor::Editor;
use vst::plugin::{Category, Plugin, Info};

use crate::gui::*;
use crate::messages::*;

use crate::utils::*;

//------------------------------------------------------------------------------

pub struct AAUnit {
    pub info: Info,
    #[allow(dead_code)]
    instance: Instance,
    memory: Memory,
    init: Func,
    get_sample_rate: Func,
    get_num_input_channels: Func,
    get_num_output_channels: Func,
    set_param_float: Func,
    set_param_int: Func,
    set_param_bool: Func,
    get_param_float: Func,
    get_param_int: Func,
    get_param_bool: Func,
    compute: Func,
    /// offset in WASM linear memory to input buffer (max buffer size 1024)
    in_buffer_offset: usize,
    /// offset in WASM linear memory to output buffer (max buffer size 1024)
    out_buffer_offset: usize,
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
        let get_num_input_channels: Func = instance.get_func("get_num_input_channels").unwrap();
        let get_num_output_channels: Func = instance.get_func("get_num_output_channels").unwrap();
        let set_param_float: Func = instance.get_func("set_param_float").unwrap();
        let set_param_int: Func = instance.get_func("set_param_int").unwrap();
        let set_param_bool: Func = instance.get_func("set_param_bool").unwrap();
        let get_param_float: Func = instance.get_func("get_param_float").unwrap();
        let get_param_int: Func = instance.get_func("get_param_int").unwrap();
        let get_param_bool: Func = instance.get_func("get_param_bool").unwrap();
        let compute = instance.get_func("compute").unwrap();
    
        let in_buffer_offset = if let Val::I32(o) = instance.get_global("IN_BUFFER").unwrap().get() {
            o as usize
        }
        else {
            return err();
        };
    
        let out_buffer_offset = if let Val::I32(o) = instance.get_global("OUT_BUFFER").unwrap().get() {
            o as usize
        }
        else {
            return err();
        };
    
        let memory = instance
            .get_memory("memory").unwrap();

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
            get_num_input_channels,
            get_num_output_channels,
            set_param_float,
            set_param_int,
            set_param_bool,
            get_param_float,
            get_param_int,
            get_param_bool,
            compute,
            in_buffer_offset,
            out_buffer_offset,
            send,
        })
    }

    #[inline]
    pub fn set_param_float(&self, index: u32, param: f32) {
        let f = self.set_param_float.get2::<u32, f32, ()>().unwrap();
        f(index, param).unwrap();
    }

    #[inline]
    pub fn set_param_int(&self, index: u32, param: i32) {
        let f = self.set_param_int.get2::<u32, i32, ()>().unwrap();
        f(index, param).unwrap();
    }

    #[inline]
    pub fn compute_duplex_one_one(&self, frames: usize, inputs: &[f32], outputs: &mut [f32]) {
        // setup and copy input audio
        let inputs0 = inputs[0..frames as usize].iter();
        let wasm_inputs0: &mut [f32] = unsafe { 
            let bytes = &mut self.memory.data_unchecked_mut()[self.in_buffer_offset..self.in_buffer_offset + (frames*4)];
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
            let bytes = &self.memory.data_unchecked()[self.out_buffer_offset..self.out_buffer_offset + (frames*4)];
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