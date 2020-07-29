
#![feature(wasm_target_feature)]
const MAX_BUFFER_SIZE: usize = 1024;
#[no_mangle]
pub static mut IN_BUFFER0: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
#[no_mangle]
pub static mut IN_BUFFER1: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
#[no_mangle]
pub static mut OUT_BUFFER0: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];
#[no_mangle]
pub static mut OUT_BUFFER1: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];

static mut ENGINE : mydsp = mydsp {
	fVslider0: 0.0,
	fRec0: [0.0;2],
	fSampleRate: 0,
};

type T = f32;

pub struct mydsp {
	fVslider0: f32,
	fRec0: [f32;2],
	fSampleRate: i32,
}

impl mydsp {
		
	fn new() -> mydsp { 
		mydsp {
			fVslider0: 0.0,
			fRec0: [0.0;2],
			fSampleRate: 0,
		}
	}
	pub fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	pub fn get_num_inputs(&self) -> i32 {
		return 1;
	}
	pub fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	pub fn get_input_rate(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
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
		let mut rate: i32;
		match (channel) {
			0 => {
				rate = 1;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
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
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	pub fn get_param(&self, param: u32) -> T {
		match param {
			0 => self.fVslider0,
			_ => 0.,
		}
	}
	
	pub fn set_param(&mut self, param: u32, value: T) {
		match param {
			0 => { self.fVslider0 = value }
			_ => {}
		}
	}
	
	#[target_feature(enable = "simd128")]
#[inline]
unsafe fn compute(&mut self, count: i32, inputs: &[&[T]], outputs: &mut[&mut[T]]) {
		let (inputs0) = if let [inputs0, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			(inputs0)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: f32 = (0.00100000005 * f32::powf(10.0, (0.0500000007 * (self.fVslider0 as f32))));
		let zipped_iterators = inputs0.zip(outputs0);
		for (input0, output0) in zipped_iterators {
			self.fRec0[0] = (fSlow0 + (0.999000013 * self.fRec0[1]));
			*output0 = (((*input0 as f32) * self.fRec0[0]) as f32);
			self.fRec0[1] = self.fRec0[0];
		}
	}


        #[inline]
        pub fn compute_external(&mut self, count: i32) {
            let (inputs, outputs) = unsafe { 
                (::std::slice::from_raw_parts(IN_BUFFER0.as_ptr(), count as usize),
                ::std::slice::from_raw_parts_mut(OUT_BUFFER0.as_mut_ptr(), count as usize)) 
            };
            unsafe { self.compute(count, &[inputs], &mut [outputs]); }
        }

}



#[no_mangle]
pub fn get_sample_rate() -> f64 {
    unsafe { ENGINE.get_sample_rate() as f64 }
}

// number of input channels (currently max 2)
#[no_mangle]
pub fn get_num_input_channels() -> u32 {
    unsafe { ENGINE.get_num_inputs() as u32 }
}

// number of output channels (currently max 2)
#[no_mangle]
pub fn get_num_output_channels() -> u32 {
    unsafe { ENGINE.get_num_outputs() as u32 }
}

#[no_mangle]
pub fn init(sample_rate: f64) -> () {
    unsafe { ENGINE.init(sample_rate as i32); }
}

#[no_mangle]
pub fn set_param_float(index: u32, v: f32) {
    unsafe { ENGINE.set_param(index, v); }
}

#[no_mangle]
pub fn set_param_int(index: u32, v: i32) {
    unsafe { ENGINE.set_param(index, v as f32); }
}

#[no_mangle]
pub fn get_param_float(index: u32) -> f32 {
    unsafe { ENGINE.get_param(index) }
}

#[no_mangle]
pub fn get_param_int(index: u32) -> i32 {
    unsafe { ENGINE.get_param(index) as i32 }
}

#[no_mangle]
pub fn compute(frames: u32) -> () {
    unsafe { ENGINE.compute_external(frames as i32); }
}
    