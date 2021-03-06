

pub struct mydsp {
	
	fDummy: f32,
	fVslider0: f32,
	fRec0: [f32;2],
	fSamplingFreq: i32,
	
}

impl mydsp {
		
	pub fn new() -> mydsp { 
		mydsp {
			fDummy: 0 as f32,
			fVslider0: 0.0,
			fRec0: [0.0;2],
			fSamplingFreq: 0,
		}
	}
	
	pub fn metadata(&mut self, m: &mut Meta) { 
		m.declare("author", "Grame");
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.0");
		m.declare("copyright", "(c)GRAME 2006");
		m.declare("filename", "gain");
		m.declare("license", "BSD");
		m.declare("name", "volume");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.0");
		m.declare("version", "1.0");
	}

	pub fn getSampleRate(&mut self) -> i32 {
		return self.fSamplingFreq;
		
	}
	pub fn getNumInputs(&mut self) -> i32 {
		return 1;
		
	}
	pub fn getNumOutputs(&mut self) -> i32 {
		return 1;
		
	}
	pub fn getInputRate(&mut self, channel: i32) -> i32 {
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
	pub fn getOutputRate(&mut self, channel: i32) -> i32 {
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
	
	pub fn classInit(samplingFreq: i32) {
		
	}
	
	pub fn instanceResetUserInterface(&mut self) {
		self.fVslider0 = 0.0;
		
	}
	
	pub fn instanceClear(&mut self) {
		for l0 in 0..2 {
			self.fRec0[l0 as usize] = 0.0;
			
		}
		
	}
	
	pub fn instanceConstants(&mut self, samplingFreq: i32) {
		self.fSamplingFreq = samplingFreq;
		
	}
	
	pub fn instanceInit(&mut self, samplingFreq: i32) {
		self.instanceConstants(samplingFreq);
		self.instanceResetUserInterface();
		self.instanceClear();
	}
	
	pub fn init(&mut self, samplingFreq: i32) {
		mydsp::classInit(samplingFreq);
		self.instanceInit(samplingFreq);
	}
	
	pub fn buildUserInterface(&mut self, ui_interface: &mut UI<f32>) {
		ui_interface.openVerticalBox("volume");
		ui_interface.declare(&mut self.fVslider0, "1", "");
		ui_interface.addVerticalSlider("0x00", &mut self.fVslider0, 0.0, -70.0, 4.0, 0.10000000000000001);
		ui_interface.closeBox();
		
	}
	
	pub fn compute(&mut self, count: i32, inputs: &[&[f32]], outputs: &mut[&mut[f32]]) {
		let mut fSlow0: f32 = (0.00100000005 * f32::powf(10.0, (0.0500000007 * (self.fVslider0 as f32))));
		for i in 0..count {
			self.fRec0[0] = (fSlow0 + (0.999000013 * self.fRec0[1]));
			outputs[0][i as usize] = ((self.fRec0[0] * (inputs[0][i as usize] as f32)) as f32);
			self.fRec0[1] = self.fRec0[0];
			
		}
		
	}

}

