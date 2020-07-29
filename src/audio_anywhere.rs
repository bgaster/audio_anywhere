//! 
//! Wasmer implementation of AAUnit
//!  TODO: update to use the new API, which supports SIMD and internal buffers
//! Copyright: Benedict R. Gaster
#![allow(dead_code)]

use std::sync::mpsc::{channel, Sender, Receiver};

use wasmer_runtime::{imports,  Instance, compile, Features, compile_with_config, compile_with_config_with, func, Func, CompilerConfig };
use wasmer_llvm_backend::LLVMCompiler;
use wasmer_runtime_core::{compile_with, backend::MemoryBoundCheckMode};

use vst::editor::Editor;
use vst::plugin::{Category, Plugin, Info};

use crate::gui::*;
use crate::messages::*;

pub static mut LEFT_MEMORY_BUFFER: *mut f32 = 0x0 as *mut f32;
pub static mut RIGHT_MEMORY_BUFFER: *mut f32 = 0x0 as *mut f32;

pub static mut LEFT_INPUT_MEMORY_BUFFER: *const f32 = 0x0 as *mut f32;
pub static mut RIGHT_INPUT_MEMORY_BUFFER: *const f32 = 0x0 as *mut f32;

#[no_mangle]
fn get_left_audio(offset: i32) -> f32 {
    unsafe {
        *LEFT_INPUT_MEMORY_BUFFER.offset(offset as isize)
    }
}

#[no_mangle]
fn get_right_audio(offset: i32) -> f32 {
    unsafe {
        *RIGHT_INPUT_MEMORY_BUFFER.offset(offset as isize)
    }
}

#[no_mangle]
fn write_left_audio(offset: i32, v: f32) {
    unsafe {
        *LEFT_MEMORY_BUFFER.offset(offset as isize) = v;
    }
}

#[no_mangle]
fn write_right_audio(offset: i32, v: f32) {
    unsafe {
        *RIGHT_MEMORY_BUFFER.offset(offset as isize) = v;
    }
}

//------------------------------------------------------------------------------

pub struct AAUnit {
    pub info: Info,
    #[allow(dead_code)]
    instance: Instance,
    pub init: Func<'static, f64, ()>,
    pub get_sample_rate: Func<'static, (), f64>,
    pub get_num_input_channels: Func<'static, (), u32>,
    pub get_num_output_channels: Func<'static, (), u32>,
    pub set_param_float: Func<'static, (u32, f32), ()>,
    pub set_param_int: Func<'static, (u32, i32), ()>,
    pub set_param_bool: Func<'static, (u32, u8), ()>,
    pub get_param_float: Func<'static, u32, f32>,
    pub get_param_int: Func<'static, u32, i32>,
    pub get_param_bool: Func<'static, u32, u8>,
    pub compute: Func<'static, u32, ()>,
    pub send: Sender<(Index, Value)>,
    //pub outgoing: Sender<Value>,
}

impl AAUnit {
    pub fn new(
        send: Sender<(Index, Value)>, 
        wasm_bytes: &[u8]) -> Result<Self, ()> {

        let mut compiler_config = CompilerConfig::default();
        compiler_config.memory_bound_check_mode = MemoryBoundCheckMode::Disable;
        compiler_config.features = Features { simd: true, threads: false };

        let import_object = imports! {
            "env" => {
                "write_left_audio" => func!(write_left_audio),
                "get_left_audio" => func!(get_left_audio),
            }
        };

        let instance = compile_with_config(
            wasm_bytes, 
            compiler_config)
        // let instance = compile_with_config_with(
        //     wasm_bytes, 
        //     compiler_config,
        //     &LLVMCompiler::new())
        //let instance = compile(wasm_bytes)
            .unwrap()
            .instantiate(&import_object)
            .unwrap();

        unsafe {
            let get_sample_rate: Func<'_, (), f64> = instance.exports.get("get_sample_rate").unwrap();
            let get_sample_rate = std::mem::transmute::<Func<'_, (), f64>, Func<'static, (), f64>>(get_sample_rate);

            let get_num_input_channels: Func<'_, (), u32> = instance.exports.get("get_num_input_channels").unwrap();
            let get_num_input_channels = std::mem::transmute::<Func<'_, (), u32>, Func<'static, (), u32>>(get_num_input_channels);

            let get_num_output_channels: Func<'_, (), u32> = instance.exports.get("get_num_output_channels").unwrap();
            let get_num_output_channels = std::mem::transmute::<Func<'_, (), u32>, Func<'static, (), u32>>(get_num_output_channels);
            
            let set_param_float: Func<'_, (u32,f32), ()> = instance.exports.get("set_param_float").unwrap();
            let set_param_float = std::mem::transmute::<Func<'_, (u32, f32), ()>, Func<'static, (u32,f32), ()>>(set_param_float);

            let set_param_int: Func<'_, (u32,i32), ()> = instance.exports.get("set_param_int").unwrap();
            let set_param_int = std::mem::transmute::<Func<'_, (u32, i32), ()>, Func<'static, (u32,i32), ()>>(set_param_int);

            let set_param_bool: Func<'_, (u32,u8), ()> = instance.exports.get("set_param_bool").unwrap();
            let set_param_bool = std::mem::transmute::<Func<'_, (u32, u8), ()>, Func<'static, (u32,u8), ()>>(set_param_bool);

            let get_param_float: Func<'_, u32, f32> = instance.exports.get("get_param_float").unwrap();
            let get_param_float = std::mem::transmute::<Func<'_, u32, f32>, Func<'static, u32, f32>>(get_param_float);

            let get_param_int: Func<'_, u32, i32> = instance.exports.get("get_param_int").unwrap();
            let get_param_int = std::mem::transmute::<Func<'_, u32, i32>, Func<'static, u32, i32>>(get_param_int);

            let get_param_bool: Func<'_, u32, u8> = instance.exports.get("get_param_bool").unwrap();
            let get_param_bool = std::mem::transmute::<Func<'_, u32, u8>, Func<'static, u32, u8>>(get_param_bool);

            let init: Func<'_, f64, ()> = instance.exports.get("init").unwrap();
            let init = std::mem::transmute::<Func<'_, f64, ()>, Func<'static, f64, ()>>(init);
            init.call(44_100.0).unwrap();

            let compute = instance.exports.get::<Func<'_, u32, ()>>("compute").unwrap();
            let compute = std::mem::transmute::<Func<'_, u32, ()>, Func<'static, u32, ()>>(compute);

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
                send,
            })
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