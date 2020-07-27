//use wasm_bindgen::prelude::*;

static mut PARAMS_FLOAT: [f32;1] = [0.;1];
static mut SAMPLE_RATE: f64 = 0.;
static mut FREC0: [f32;2] = [0.;2];

extern "C" {
    fn write_left_audio(offset: i32, v: f32);
    fn get_left_audio(offset: i32) -> f32;
}

#[inline]
fn get_frec0(index: usize) -> f32 {
    // avoid bounds checking (NOTE: WASM will do this for us)
    unsafe { *(FREC0.as_mut_ptr().add(index as usize)) }
}

fn get_mut_frec0(index: usize) -> *mut f32 {
    // avoid bounds checking (NOTE: WASM will do this for us)
    unsafe { FREC0.as_mut_ptr().add(index as usize) }
}

#[inline]
fn set_sample_rate(sample_rate: f64) {
    unsafe { SAMPLE_RATE = sample_rate; }
}

#[no_mangle]
pub fn get_sample_rate() -> f64 {
    unsafe { SAMPLE_RATE }
}

#[no_mangle]
pub fn get_num_input_channels() -> u32 {
    1
}

// number of output channels (currently max 2)
#[no_mangle]
pub fn get_num_output_channels() -> u32 {
    1
}

#[no_mangle]
pub fn init(sample_rate: f64) -> () {
    set_sample_rate(sample_rate);
    set_param_float(0, 0.);
}

#[no_mangle]
pub fn set_param_float(index: u32, v: f32) {
    // avoid bounds checking (NOTE: WASM will do this for us)
    unsafe { *(PARAMS_FLOAT.as_mut_ptr().add(index as usize)) = v; } 
}

#[no_mangle]
pub fn set_param_int(index: u32, v: i32) {
    unsafe { *(PARAMS_FLOAT.as_mut_ptr().add(index as usize)) = v as f32; } 
}

/// set bool parameter 
/// panics if param not defined
#[no_mangle]
pub fn set_param_bool(_index: u32, _v: u8) {
    unsafe { core::arch::wasm32::unreachable(); }
}

// float parameter at index
// panics if param not defined
#[no_mangle]
pub fn get_param_float(index: u32) -> f32 {
    unsafe { *(PARAMS_FLOAT.as_ptr().add(index as usize)) }
}

// int parameter at index
// panics if param not defined
#[no_mangle]
pub fn get_param_int(_index: u32) -> i32 {
    unsafe { core::arch::wasm32::unreachable(); }
}

// bool parameter at index
// panics if param not defined
#[no_mangle]
pub fn get_param_bool(_index: u32) -> bool {
    unsafe { core::arch::wasm32::unreachable(); }
}

#[no_mangle]
pub fn compute(frames: u32) -> () {
    let param = get_param_float(0);
    let f_slow0: f32 = 0.00100000005 * f32::powf(10.0, 0.0500000007 * (param));
    for i in 0..frames {
        unsafe {
            *get_mut_frec0(0) = f_slow0 + (0.999000013 * get_frec0(1));
            write_left_audio(i as i32, get_frec0(0) * get_left_audio(i as i32)); 
            *get_mut_frec0(1) = get_frec0(0);
        }
    }
}

