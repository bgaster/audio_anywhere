#![feature(wasm_target_feature)]

const MAX_BUFFER_SIZE: usize = 1024;

#[no_mangle]
pub static mut IN_BUFFER: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];

#[no_mangle]
pub static mut OUT_BUFFER: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];

pub struct MyDSP {
	pub fVslider0: f32,
	pub fRec0: [f32;2],
    pub fSampleRate: i32,
}

static mut engine: MyDSP =  MyDSP {
    fVslider0: 0.,
    fRec0: [0.,0.],
    fSampleRate: 0,
};

impl MyDSP {
		
	fn new() -> MyDSP { 
		MyDSP {
			fVslider0: 0.0,
			fRec0: [0.0;2],
            fSampleRate: 0,
		}
	}

    #[inline]
	pub fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
    }

    #[inline]
	pub fn get_num_inputs(&self) -> i32 {
		return 1;
    }

    #[inline]
	pub fn get_num_outputs(&self) -> i32 {
		return 1;
    }
    
	pub fn get_input_rate(&self, channel: i32) -> i32 {
		let rate: i32;
		match channel {
			0 => {
				rate = 1;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
    }
    
	pub fn get_output_rate(&self, channel: i32) -> i32 {
		let rate: i32;
		match channel {
			0 => {
				rate = 1;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	
	fn class_init(_sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fVslider0 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec0[l0 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	pub fn init(&mut self, sample_rate: i32) {
		Self::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
    
    #[inline]
	pub fn set_param(&mut self, value: f32) {
			self.fVslider0 = value;
    }
    
    #[inline]
	pub fn get_param(&mut self) -> f32 {
			self.fVslider0
	}
    
    #[target_feature(enable = "simd128")]
    #[inline]
	unsafe fn compute_internal(&mut self, count: i32, inputs: &[f32], outputs: &mut[f32]) {
		let inputs0 = inputs[..count as usize].iter();
		let outputs0 = outputs[..count as usize].iter_mut();
		
		let fSlow0: f32 = 0.00100000005 * f32::powf(10.0, 0.0500000007 * (self.fVslider0 as f32));
		let zipped_iterators = inputs0.zip(outputs0);
		for (input0, output0) in zipped_iterators {
			self.fRec0[0] = fSlow0 + (0.999000013 * self.fRec0[1]);
			*output0 = ((*input0 as f32) * self.fRec0[0]) as f32;
			self.fRec0[1] = self.fRec0[0];
		}
    }

    #[inline]
    pub fn compute(&mut self, count: i32) {
        let (inputs, outputs) = unsafe { 
            (::std::slice::from_raw_parts(IN_BUFFER.as_ptr(), count as usize),
            ::std::slice::from_raw_parts_mut(OUT_BUFFER.as_mut_ptr(), count as usize)) 
        };
        unsafe { self.compute_internal(count, inputs, outputs); }
    }
}

#[no_mangle]
pub fn get_sample_rate() -> f64 {
    unsafe { engine.get_sample_rate() as f64 }
}

// number of input channels (currently max 2)
#[no_mangle]
pub fn get_num_input_channels() -> u32 {
    unsafe { engine.get_num_inputs() as u32 }
}

// number of output channels (currently max 2)
#[no_mangle]
pub fn get_num_output_channels() -> u32 {
    unsafe { engine.get_num_outputs() as u32 }
}

#[no_mangle]
pub fn init(sample_rate: f64) -> () {
    unsafe { engine.init(sample_rate as i32); }
}

#[no_mangle]
pub fn set_param_float(_index: u32, v: f32) {
    unsafe { engine.set_param(v); }
}

#[no_mangle]
pub fn set_param_int(_index: u32, v: i32) {
    unsafe { engine.set_param(v as f32); }
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
pub fn get_param_float(_index: u32) -> f32 {
    unsafe { engine.get_param() }
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
    unsafe { engine.compute(frames as i32); }
}
