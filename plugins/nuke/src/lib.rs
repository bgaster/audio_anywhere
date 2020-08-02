#![feature(wasm_target_feature)]

#[inline]
fn mydsp_faustpower2_f(value: f32) -> f32 {
	return (value * value);
}

const MAX_BUFFER_SIZE: usize = 1024;
#[no_mangle]
pub static mut OUT_BUFFER0: [f32;MAX_BUFFER_SIZE] = [0.;MAX_BUFFER_SIZE];

static mut ENGINE : mydsp = mydsp {
	fHslider0: 0.0,
	iVec0: [0;2],
	fSampleRate: 0,
	fConst0: 0.0,
	fConst1: 0.0,
	fButton0: 0.0,
	fHslider1: 0.0,
	iRec1: [0;2],
	fHslider2: 0.0,
	fHslider3: 0.0,
	fHslider4: 0.0,
	fRec0: [0.0;2],
	fHslider5: 0.0,
	fRec4: [0.0;2],
	fHslider6: 0.0,
	fRec5: [0.0;2],
	fVec1: [0.0;2],
	fConst2: 0.0,
	fHslider7: 0.0,
	fVec2: [0.0;2],
	fConst3: 0.0,
	fRec6: [0.0;2],
	fVec3: [0.0;2],
	IOTA: 0,
	fVec4: [0.0;4096],
	fConst4: 0.0,
	fConst5: 0.0,
	fRec7: [0.0;2],
	fConst6: 0.0,
	iRec9: [0;2],
	fRec8: [0.0;2],
	fVec5: [0.0;2],
	fHslider8: 0.0,
	fRec10: [0.0;2],
	iRec12: [0;2],
	fRec11: [0.0;2],
	fVec6: [0.0;2],
	fVec7: [0.0;2],
	fRec13: [0.0;2],
	fVec8: [0.0;2],
	fVec9: [0.0;4096],
	iRec15: [0;2],
	fRec14: [0.0;2],
	fVec10: [0.0;2],
	fVec11: [0.0;2],
	fRec16: [0.0;2],
	fVec12: [0.0;2],
	fVec13: [0.0;4096],
	fConst7: 0.0,
	fRec17: [0.0;2],
	fRec18: [0.0;2],
	fVec14: [0.0;2],
	fRec19: [0.0;2],
	fVec15: [0.0;2],
	fVec16: [0.0;4096],
	fRec20: [0.0;2],
	iRec22: [0;2],
	fRec21: [0.0;2],
	fVec17: [0.0;2],
	fConst8: 0.0,
	fHslider9: 0.0,
	fRec23: [0.0;2],
	fRec3: [0.0;3],
	fRec2: [0.0;3],
	fHslider10: 0.0,
	fButton1: 0.0,
	iRec25: [0;2],
	fRec24: [0.0;2],
	fHslider11: 0.0,
	fVec18: [0.0;2],
	fRec28: [0.0;2],
	fVec19: [0.0;2],
	fVec20: [0.0;4096],
	fRec29: [0.0;2],
	iRec31: [0;2],
	fRec30: [0.0;2],
	fVec21: [0.0;2],
	iRec33: [0;2],
	fRec32: [0.0;2],
	fVec22: [0.0;2],
	fVec23: [0.0;2],
	fRec34: [0.0;2],
	fVec24: [0.0;2],
	fVec25: [0.0;4096],
	iRec36: [0;2],
	fRec35: [0.0;2],
	fVec26: [0.0;2],
	fVec27: [0.0;2],
	fRec37: [0.0;2],
	fVec28: [0.0;2],
	fVec29: [0.0;4096],
	fRec38: [0.0;2],
	fRec39: [0.0;2],
	fVec30: [0.0;2],
	fRec40: [0.0;2],
	fVec31: [0.0;2],
	fVec32: [0.0;4096],
	fRec41: [0.0;2],
	iRec43: [0;2],
	fRec42: [0.0;2],
	fVec33: [0.0;2],
	fRec27: [0.0;3],
	fRec26: [0.0;3],
	fHslider12: 0.0,
	fButton2: 0.0,
	iRec45: [0;2],
	fRec44: [0.0;2],
	fHslider13: 0.0,
	fVec34: [0.0;2],
	fRec48: [0.0;2],
	fVec35: [0.0;2],
	fVec36: [0.0;4096],
	fRec49: [0.0;2],
	iRec51: [0;2],
	fRec50: [0.0;2],
	fVec37: [0.0;2],
	iRec53: [0;2],
	fRec52: [0.0;2],
	fVec38: [0.0;2],
	fVec39: [0.0;2],
	fRec54: [0.0;2],
	fVec40: [0.0;2],
	fVec41: [0.0;4096],
	iRec56: [0;2],
	fRec55: [0.0;2],
	fVec42: [0.0;2],
	fVec43: [0.0;2],
	fRec57: [0.0;2],
	fVec44: [0.0;2],
	fVec45: [0.0;4096],
	fRec58: [0.0;2],
	fRec59: [0.0;2],
	fVec46: [0.0;2],
	fRec60: [0.0;2],
	fVec47: [0.0;2],
	fVec48: [0.0;4096],
	fRec61: [0.0;2],
	iRec63: [0;2],
	fRec62: [0.0;2],
	fVec49: [0.0;2],
	fRec47: [0.0;3],
	fRec46: [0.0;3],
	fHslider14: 0.0,
	fButton3: 0.0,
	iRec65: [0;2],
	fRec64: [0.0;2],
	fHslider15: 0.0,
	fVec50: [0.0;2],
	fRec68: [0.0;2],
	fVec51: [0.0;2],
	fVec52: [0.0;4096],
	fRec69: [0.0;2],
	iRec71: [0;2],
	fRec70: [0.0;2],
	fVec53: [0.0;2],
	iRec73: [0;2],
	fRec72: [0.0;2],
	fVec54: [0.0;2],
	fVec55: [0.0;2],
	fRec74: [0.0;2],
	fVec56: [0.0;2],
	fVec57: [0.0;4096],
	iRec76: [0;2],
	fRec75: [0.0;2],
	fVec58: [0.0;2],
	fVec59: [0.0;2],
	fRec77: [0.0;2],
	fVec60: [0.0;2],
	fVec61: [0.0;4096],
	fRec78: [0.0;2],
	fRec79: [0.0;2],
	fVec62: [0.0;2],
	fRec80: [0.0;2],
	fVec63: [0.0;2],
	fVec64: [0.0;4096],
	fRec81: [0.0;2],
	iRec83: [0;2],
	fRec82: [0.0;2],
	fVec65: [0.0;2],
	fRec67: [0.0;3],
	fRec66: [0.0;3],
	fHslider16: 0.0,
	fButton4: 0.0,
	iRec85: [0;2],
	fRec84: [0.0;2],
	fHslider17: 0.0,
	fVec66: [0.0;2],
	fRec88: [0.0;2],
	fVec67: [0.0;2],
	fVec68: [0.0;4096],
	fRec89: [0.0;2],
	iRec91: [0;2],
	fRec90: [0.0;2],
	fVec69: [0.0;2],
	iRec93: [0;2],
	fRec92: [0.0;2],
	fVec70: [0.0;2],
	fVec71: [0.0;2],
	fRec94: [0.0;2],
	fVec72: [0.0;2],
	fVec73: [0.0;4096],
	iRec96: [0;2],
	fRec95: [0.0;2],
	fVec74: [0.0;2],
	fVec75: [0.0;2],
	fRec97: [0.0;2],
	fVec76: [0.0;2],
	fVec77: [0.0;4096],
	fRec98: [0.0;2],
	fRec99: [0.0;2],
	fVec78: [0.0;2],
	fRec100: [0.0;2],
	fVec79: [0.0;2],
	fVec80: [0.0;4096],
	fRec101: [0.0;2],
	iRec103: [0;2],
	fRec102: [0.0;2],
	fVec81: [0.0;2],
	fRec87: [0.0;3],
	fRec86: [0.0;3],
	fHslider18: 0.0,
	fButton5: 0.0,
	iRec105: [0;2],
	fRec104: [0.0;2],
	fHslider19: 0.0,
	fVec82: [0.0;2],
	fRec108: [0.0;2],
	fVec83: [0.0;2],
	fVec84: [0.0;4096],
	fRec109: [0.0;2],
	iRec111: [0;2],
	fRec110: [0.0;2],
	fVec85: [0.0;2],
	iRec113: [0;2],
	fRec112: [0.0;2],
	fVec86: [0.0;2],
	fVec87: [0.0;2],
	fRec114: [0.0;2],
	fVec88: [0.0;2],
	fVec89: [0.0;4096],
	iRec116: [0;2],
	fRec115: [0.0;2],
	fVec90: [0.0;2],
	fVec91: [0.0;2],
	fRec117: [0.0;2],
	fVec92: [0.0;2],
	fVec93: [0.0;4096],
	fRec118: [0.0;2],
	fRec119: [0.0;2],
	fVec94: [0.0;2],
	fRec120: [0.0;2],
	fVec95: [0.0;2],
	fVec96: [0.0;4096],
	fRec121: [0.0;2],
	iRec123: [0;2],
	fRec122: [0.0;2],
	fVec97: [0.0;2],
	fRec107: [0.0;3],
	fRec106: [0.0;3],
};

type T = f32;

pub struct mydsp {
	fHslider0: f32,
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: f32,
	fConst1: f32,
	fButton0: f32,
	fHslider1: f32,
	iRec1: [i32;2],
	fHslider2: f32,
	fHslider3: f32,
	fHslider4: f32,
	fRec0: [f32;2],
	fHslider5: f32,
	fRec4: [f32;2],
	fHslider6: f32,
	fRec5: [f32;2],
	fVec1: [f32;2],
	fConst2: f32,
	fHslider7: f32,
	fVec2: [f32;2],
	fConst3: f32,
	fRec6: [f32;2],
	fVec3: [f32;2],
	IOTA: i32,
	fVec4: [f32;4096],
	fConst4: f32,
	fConst5: f32,
	fRec7: [f32;2],
	fConst6: f32,
	iRec9: [i32;2],
	fRec8: [f32;2],
	fVec5: [f32;2],
	fHslider8: f32,
	fRec10: [f32;2],
	iRec12: [i32;2],
	fRec11: [f32;2],
	fVec6: [f32;2],
	fVec7: [f32;2],
	fRec13: [f32;2],
	fVec8: [f32;2],
	fVec9: [f32;4096],
	iRec15: [i32;2],
	fRec14: [f32;2],
	fVec10: [f32;2],
	fVec11: [f32;2],
	fRec16: [f32;2],
	fVec12: [f32;2],
	fVec13: [f32;4096],
	fConst7: f32,
	fRec17: [f32;2],
	fRec18: [f32;2],
	fVec14: [f32;2],
	fRec19: [f32;2],
	fVec15: [f32;2],
	fVec16: [f32;4096],
	fRec20: [f32;2],
	iRec22: [i32;2],
	fRec21: [f32;2],
	fVec17: [f32;2],
	fConst8: f32,
	fHslider9: f32,
	fRec23: [f32;2],
	fRec3: [f32;3],
	fRec2: [f32;3],
	fHslider10: f32,
	fButton1: f32,
	iRec25: [i32;2],
	fRec24: [f32;2],
	fHslider11: f32,
	fVec18: [f32;2],
	fRec28: [f32;2],
	fVec19: [f32;2],
	fVec20: [f32;4096],
	fRec29: [f32;2],
	iRec31: [i32;2],
	fRec30: [f32;2],
	fVec21: [f32;2],
	iRec33: [i32;2],
	fRec32: [f32;2],
	fVec22: [f32;2],
	fVec23: [f32;2],
	fRec34: [f32;2],
	fVec24: [f32;2],
	fVec25: [f32;4096],
	iRec36: [i32;2],
	fRec35: [f32;2],
	fVec26: [f32;2],
	fVec27: [f32;2],
	fRec37: [f32;2],
	fVec28: [f32;2],
	fVec29: [f32;4096],
	fRec38: [f32;2],
	fRec39: [f32;2],
	fVec30: [f32;2],
	fRec40: [f32;2],
	fVec31: [f32;2],
	fVec32: [f32;4096],
	fRec41: [f32;2],
	iRec43: [i32;2],
	fRec42: [f32;2],
	fVec33: [f32;2],
	fRec27: [f32;3],
	fRec26: [f32;3],
	fHslider12: f32,
	fButton2: f32,
	iRec45: [i32;2],
	fRec44: [f32;2],
	fHslider13: f32,
	fVec34: [f32;2],
	fRec48: [f32;2],
	fVec35: [f32;2],
	fVec36: [f32;4096],
	fRec49: [f32;2],
	iRec51: [i32;2],
	fRec50: [f32;2],
	fVec37: [f32;2],
	iRec53: [i32;2],
	fRec52: [f32;2],
	fVec38: [f32;2],
	fVec39: [f32;2],
	fRec54: [f32;2],
	fVec40: [f32;2],
	fVec41: [f32;4096],
	iRec56: [i32;2],
	fRec55: [f32;2],
	fVec42: [f32;2],
	fVec43: [f32;2],
	fRec57: [f32;2],
	fVec44: [f32;2],
	fVec45: [f32;4096],
	fRec58: [f32;2],
	fRec59: [f32;2],
	fVec46: [f32;2],
	fRec60: [f32;2],
	fVec47: [f32;2],
	fVec48: [f32;4096],
	fRec61: [f32;2],
	iRec63: [i32;2],
	fRec62: [f32;2],
	fVec49: [f32;2],
	fRec47: [f32;3],
	fRec46: [f32;3],
	fHslider14: f32,
	fButton3: f32,
	iRec65: [i32;2],
	fRec64: [f32;2],
	fHslider15: f32,
	fVec50: [f32;2],
	fRec68: [f32;2],
	fVec51: [f32;2],
	fVec52: [f32;4096],
	fRec69: [f32;2],
	iRec71: [i32;2],
	fRec70: [f32;2],
	fVec53: [f32;2],
	iRec73: [i32;2],
	fRec72: [f32;2],
	fVec54: [f32;2],
	fVec55: [f32;2],
	fRec74: [f32;2],
	fVec56: [f32;2],
	fVec57: [f32;4096],
	iRec76: [i32;2],
	fRec75: [f32;2],
	fVec58: [f32;2],
	fVec59: [f32;2],
	fRec77: [f32;2],
	fVec60: [f32;2],
	fVec61: [f32;4096],
	fRec78: [f32;2],
	fRec79: [f32;2],
	fVec62: [f32;2],
	fRec80: [f32;2],
	fVec63: [f32;2],
	fVec64: [f32;4096],
	fRec81: [f32;2],
	iRec83: [i32;2],
	fRec82: [f32;2],
	fVec65: [f32;2],
	fRec67: [f32;3],
	fRec66: [f32;3],
	fHslider16: f32,
	fButton4: f32,
	iRec85: [i32;2],
	fRec84: [f32;2],
	fHslider17: f32,
	fVec66: [f32;2],
	fRec88: [f32;2],
	fVec67: [f32;2],
	fVec68: [f32;4096],
	fRec89: [f32;2],
	iRec91: [i32;2],
	fRec90: [f32;2],
	fVec69: [f32;2],
	iRec93: [i32;2],
	fRec92: [f32;2],
	fVec70: [f32;2],
	fVec71: [f32;2],
	fRec94: [f32;2],
	fVec72: [f32;2],
	fVec73: [f32;4096],
	iRec96: [i32;2],
	fRec95: [f32;2],
	fVec74: [f32;2],
	fVec75: [f32;2],
	fRec97: [f32;2],
	fVec76: [f32;2],
	fVec77: [f32;4096],
	fRec98: [f32;2],
	fRec99: [f32;2],
	fVec78: [f32;2],
	fRec100: [f32;2],
	fVec79: [f32;2],
	fVec80: [f32;4096],
	fRec101: [f32;2],
	iRec103: [i32;2],
	fRec102: [f32;2],
	fVec81: [f32;2],
	fRec87: [f32;3],
	fRec86: [f32;3],
	fHslider18: f32,
	fButton5: f32,
	iRec105: [i32;2],
	fRec104: [f32;2],
	fHslider19: f32,
	fVec82: [f32;2],
	fRec108: [f32;2],
	fVec83: [f32;2],
	fVec84: [f32;4096],
	fRec109: [f32;2],
	iRec111: [i32;2],
	fRec110: [f32;2],
	fVec85: [f32;2],
	iRec113: [i32;2],
	fRec112: [f32;2],
	fVec86: [f32;2],
	fVec87: [f32;2],
	fRec114: [f32;2],
	fVec88: [f32;2],
	fVec89: [f32;4096],
	iRec116: [i32;2],
	fRec115: [f32;2],
	fVec90: [f32;2],
	fVec91: [f32;2],
	fRec117: [f32;2],
	fVec92: [f32;2],
	fVec93: [f32;4096],
	fRec118: [f32;2],
	fRec119: [f32;2],
	fVec94: [f32;2],
	fRec120: [f32;2],
	fVec95: [f32;2],
	fVec96: [f32;4096],
	fRec121: [f32;2],
	iRec123: [i32;2],
	fRec122: [f32;2],
	fVec97: [f32;2],
	fRec107: [f32;3],
	fRec106: [f32;3],
}

impl mydsp {
		
	fn new() -> mydsp { 
		mydsp {
			fHslider0: 0.0,
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fButton0: 0.0,
			fHslider1: 0.0,
			iRec1: [0;2],
			fHslider2: 0.0,
			fHslider3: 0.0,
			fHslider4: 0.0,
			fRec0: [0.0;2],
			fHslider5: 0.0,
			fRec4: [0.0;2],
			fHslider6: 0.0,
			fRec5: [0.0;2],
			fVec1: [0.0;2],
			fConst2: 0.0,
			fHslider7: 0.0,
			fVec2: [0.0;2],
			fConst3: 0.0,
			fRec6: [0.0;2],
			fVec3: [0.0;2],
			IOTA: 0,
			fVec4: [0.0;4096],
			fConst4: 0.0,
			fConst5: 0.0,
			fRec7: [0.0;2],
			fConst6: 0.0,
			iRec9: [0;2],
			fRec8: [0.0;2],
			fVec5: [0.0;2],
			fHslider8: 0.0,
			fRec10: [0.0;2],
			iRec12: [0;2],
			fRec11: [0.0;2],
			fVec6: [0.0;2],
			fVec7: [0.0;2],
			fRec13: [0.0;2],
			fVec8: [0.0;2],
			fVec9: [0.0;4096],
			iRec15: [0;2],
			fRec14: [0.0;2],
			fVec10: [0.0;2],
			fVec11: [0.0;2],
			fRec16: [0.0;2],
			fVec12: [0.0;2],
			fVec13: [0.0;4096],
			fConst7: 0.0,
			fRec17: [0.0;2],
			fRec18: [0.0;2],
			fVec14: [0.0;2],
			fRec19: [0.0;2],
			fVec15: [0.0;2],
			fVec16: [0.0;4096],
			fRec20: [0.0;2],
			iRec22: [0;2],
			fRec21: [0.0;2],
			fVec17: [0.0;2],
			fConst8: 0.0,
			fHslider9: 0.0,
			fRec23: [0.0;2],
			fRec3: [0.0;3],
			fRec2: [0.0;3],
			fHslider10: 0.0,
			fButton1: 0.0,
			iRec25: [0;2],
			fRec24: [0.0;2],
			fHslider11: 0.0,
			fVec18: [0.0;2],
			fRec28: [0.0;2],
			fVec19: [0.0;2],
			fVec20: [0.0;4096],
			fRec29: [0.0;2],
			iRec31: [0;2],
			fRec30: [0.0;2],
			fVec21: [0.0;2],
			iRec33: [0;2],
			fRec32: [0.0;2],
			fVec22: [0.0;2],
			fVec23: [0.0;2],
			fRec34: [0.0;2],
			fVec24: [0.0;2],
			fVec25: [0.0;4096],
			iRec36: [0;2],
			fRec35: [0.0;2],
			fVec26: [0.0;2],
			fVec27: [0.0;2],
			fRec37: [0.0;2],
			fVec28: [0.0;2],
			fVec29: [0.0;4096],
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fVec30: [0.0;2],
			fRec40: [0.0;2],
			fVec31: [0.0;2],
			fVec32: [0.0;4096],
			fRec41: [0.0;2],
			iRec43: [0;2],
			fRec42: [0.0;2],
			fVec33: [0.0;2],
			fRec27: [0.0;3],
			fRec26: [0.0;3],
			fHslider12: 0.0,
			fButton2: 0.0,
			iRec45: [0;2],
			fRec44: [0.0;2],
			fHslider13: 0.0,
			fVec34: [0.0;2],
			fRec48: [0.0;2],
			fVec35: [0.0;2],
			fVec36: [0.0;4096],
			fRec49: [0.0;2],
			iRec51: [0;2],
			fRec50: [0.0;2],
			fVec37: [0.0;2],
			iRec53: [0;2],
			fRec52: [0.0;2],
			fVec38: [0.0;2],
			fVec39: [0.0;2],
			fRec54: [0.0;2],
			fVec40: [0.0;2],
			fVec41: [0.0;4096],
			iRec56: [0;2],
			fRec55: [0.0;2],
			fVec42: [0.0;2],
			fVec43: [0.0;2],
			fRec57: [0.0;2],
			fVec44: [0.0;2],
			fVec45: [0.0;4096],
			fRec58: [0.0;2],
			fRec59: [0.0;2],
			fVec46: [0.0;2],
			fRec60: [0.0;2],
			fVec47: [0.0;2],
			fVec48: [0.0;4096],
			fRec61: [0.0;2],
			iRec63: [0;2],
			fRec62: [0.0;2],
			fVec49: [0.0;2],
			fRec47: [0.0;3],
			fRec46: [0.0;3],
			fHslider14: 0.0,
			fButton3: 0.0,
			iRec65: [0;2],
			fRec64: [0.0;2],
			fHslider15: 0.0,
			fVec50: [0.0;2],
			fRec68: [0.0;2],
			fVec51: [0.0;2],
			fVec52: [0.0;4096],
			fRec69: [0.0;2],
			iRec71: [0;2],
			fRec70: [0.0;2],
			fVec53: [0.0;2],
			iRec73: [0;2],
			fRec72: [0.0;2],
			fVec54: [0.0;2],
			fVec55: [0.0;2],
			fRec74: [0.0;2],
			fVec56: [0.0;2],
			fVec57: [0.0;4096],
			iRec76: [0;2],
			fRec75: [0.0;2],
			fVec58: [0.0;2],
			fVec59: [0.0;2],
			fRec77: [0.0;2],
			fVec60: [0.0;2],
			fVec61: [0.0;4096],
			fRec78: [0.0;2],
			fRec79: [0.0;2],
			fVec62: [0.0;2],
			fRec80: [0.0;2],
			fVec63: [0.0;2],
			fVec64: [0.0;4096],
			fRec81: [0.0;2],
			iRec83: [0;2],
			fRec82: [0.0;2],
			fVec65: [0.0;2],
			fRec67: [0.0;3],
			fRec66: [0.0;3],
			fHslider16: 0.0,
			fButton4: 0.0,
			iRec85: [0;2],
			fRec84: [0.0;2],
			fHslider17: 0.0,
			fVec66: [0.0;2],
			fRec88: [0.0;2],
			fVec67: [0.0;2],
			fVec68: [0.0;4096],
			fRec89: [0.0;2],
			iRec91: [0;2],
			fRec90: [0.0;2],
			fVec69: [0.0;2],
			iRec93: [0;2],
			fRec92: [0.0;2],
			fVec70: [0.0;2],
			fVec71: [0.0;2],
			fRec94: [0.0;2],
			fVec72: [0.0;2],
			fVec73: [0.0;4096],
			iRec96: [0;2],
			fRec95: [0.0;2],
			fVec74: [0.0;2],
			fVec75: [0.0;2],
			fRec97: [0.0;2],
			fVec76: [0.0;2],
			fVec77: [0.0;4096],
			fRec98: [0.0;2],
			fRec99: [0.0;2],
			fVec78: [0.0;2],
			fRec100: [0.0;2],
			fVec79: [0.0;2],
			fVec80: [0.0;4096],
			fRec101: [0.0;2],
			iRec103: [0;2],
			fRec102: [0.0;2],
			fVec81: [0.0;2],
			fRec87: [0.0;3],
			fRec86: [0.0;3],
			fHslider18: 0.0,
			fButton5: 0.0,
			iRec105: [0;2],
			fRec104: [0.0;2],
			fHslider19: 0.0,
			fVec82: [0.0;2],
			fRec108: [0.0;2],
			fVec83: [0.0;2],
			fVec84: [0.0;4096],
			fRec109: [0.0;2],
			iRec111: [0;2],
			fRec110: [0.0;2],
			fVec85: [0.0;2],
			iRec113: [0;2],
			fRec112: [0.0;2],
			fVec86: [0.0;2],
			fVec87: [0.0;2],
			fRec114: [0.0;2],
			fVec88: [0.0;2],
			fVec89: [0.0;4096],
			iRec116: [0;2],
			fRec115: [0.0;2],
			fVec90: [0.0;2],
			fVec91: [0.0;2],
			fRec117: [0.0;2],
			fVec92: [0.0;2],
			fVec93: [0.0;4096],
			fRec118: [0.0;2],
			fRec119: [0.0;2],
			fVec94: [0.0;2],
			fRec120: [0.0;2],
			fVec95: [0.0;2],
			fVec96: [0.0;4096],
			fRec121: [0.0;2],
			iRec123: [0;2],
			fRec122: [0.0;2],
			fVec97: [0.0;2],
			fRec107: [0.0;3],
			fRec106: [0.0;3],
		}
	}
	pub fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	pub fn get_num_inputs(&self) -> i32 {
		return 0;
	}
	pub fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	pub fn get_input_rate(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
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
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.iRec1[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec0[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec4[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec5[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fVec1[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fVec2[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec6[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fVec3[l8 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l9 in 0..4096 {
			self.fVec4[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec7[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.iRec9[l11 as usize] = 0;
		}
		for l12 in 0..2 {
			self.fRec8[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fVec5[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec10[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.iRec12[l15 as usize] = 0;
		}
		for l16 in 0..2 {
			self.fRec11[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fVec6[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fVec7[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec13[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fVec8[l20 as usize] = 0.0;
		}
		for l21 in 0..4096 {
			self.fVec9[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.iRec15[l22 as usize] = 0;
		}
		for l23 in 0..2 {
			self.fRec14[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fVec10[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec11[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec16[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fVec12[l27 as usize] = 0.0;
		}
		for l28 in 0..4096 {
			self.fVec13[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec17[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec18[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec14[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec19[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec15[l33 as usize] = 0.0;
		}
		for l34 in 0..4096 {
			self.fVec16[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec20[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.iRec22[l36 as usize] = 0;
		}
		for l37 in 0..2 {
			self.fRec21[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fVec17[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec23[l39 as usize] = 0.0;
		}
		for l40 in 0..3 {
			self.fRec3[l40 as usize] = 0.0;
		}
		for l41 in 0..3 {
			self.fRec2[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.iRec25[l42 as usize] = 0;
		}
		for l43 in 0..2 {
			self.fRec24[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fVec18[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec28[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec19[l46 as usize] = 0.0;
		}
		for l47 in 0..4096 {
			self.fVec20[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec29[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.iRec31[l49 as usize] = 0;
		}
		for l50 in 0..2 {
			self.fRec30[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fVec21[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.iRec33[l52 as usize] = 0;
		}
		for l53 in 0..2 {
			self.fRec32[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec22[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fVec23[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec34[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fVec24[l57 as usize] = 0.0;
		}
		for l58 in 0..4096 {
			self.fVec25[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.iRec36[l59 as usize] = 0;
		}
		for l60 in 0..2 {
			self.fRec35[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fVec26[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fVec27[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec37[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fVec28[l64 as usize] = 0.0;
		}
		for l65 in 0..4096 {
			self.fVec29[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec38[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec39[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fVec30[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec40[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fVec31[l70 as usize] = 0.0;
		}
		for l71 in 0..4096 {
			self.fVec32[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec41[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.iRec43[l73 as usize] = 0;
		}
		for l74 in 0..2 {
			self.fRec42[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fVec33[l75 as usize] = 0.0;
		}
		for l76 in 0..3 {
			self.fRec27[l76 as usize] = 0.0;
		}
		for l77 in 0..3 {
			self.fRec26[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.iRec45[l78 as usize] = 0;
		}
		for l79 in 0..2 {
			self.fRec44[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fVec34[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec48[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fVec35[l82 as usize] = 0.0;
		}
		for l83 in 0..4096 {
			self.fVec36[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec49[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.iRec51[l85 as usize] = 0;
		}
		for l86 in 0..2 {
			self.fRec50[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fVec37[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.iRec53[l88 as usize] = 0;
		}
		for l89 in 0..2 {
			self.fRec52[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fVec38[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fVec39[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec54[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fVec40[l93 as usize] = 0.0;
		}
		for l94 in 0..4096 {
			self.fVec41[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.iRec56[l95 as usize] = 0;
		}
		for l96 in 0..2 {
			self.fRec55[l96 as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fVec42[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fVec43[l98 as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec57[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fVec44[l100 as usize] = 0.0;
		}
		for l101 in 0..4096 {
			self.fVec45[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec58[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec59[l103 as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fVec46[l104 as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec60[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fVec47[l106 as usize] = 0.0;
		}
		for l107 in 0..4096 {
			self.fVec48[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec61[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.iRec63[l109 as usize] = 0;
		}
		for l110 in 0..2 {
			self.fRec62[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fVec49[l111 as usize] = 0.0;
		}
		for l112 in 0..3 {
			self.fRec47[l112 as usize] = 0.0;
		}
		for l113 in 0..3 {
			self.fRec46[l113 as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.iRec65[l114 as usize] = 0;
		}
		for l115 in 0..2 {
			self.fRec64[l115 as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fVec50[l116 as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec68[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fVec51[l118 as usize] = 0.0;
		}
		for l119 in 0..4096 {
			self.fVec52[l119 as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec69[l120 as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.iRec71[l121 as usize] = 0;
		}
		for l122 in 0..2 {
			self.fRec70[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fVec53[l123 as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.iRec73[l124 as usize] = 0;
		}
		for l125 in 0..2 {
			self.fRec72[l125 as usize] = 0.0;
		}
		for l126 in 0..2 {
			self.fVec54[l126 as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fVec55[l127 as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec74[l128 as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fVec56[l129 as usize] = 0.0;
		}
		for l130 in 0..4096 {
			self.fVec57[l130 as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.iRec76[l131 as usize] = 0;
		}
		for l132 in 0..2 {
			self.fRec75[l132 as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fVec58[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fVec59[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec77[l135 as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fVec60[l136 as usize] = 0.0;
		}
		for l137 in 0..4096 {
			self.fVec61[l137 as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec78[l138 as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec79[l139 as usize] = 0.0;
		}
		for l140 in 0..2 {
			self.fVec62[l140 as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec80[l141 as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fVec63[l142 as usize] = 0.0;
		}
		for l143 in 0..4096 {
			self.fVec64[l143 as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec81[l144 as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.iRec83[l145 as usize] = 0;
		}
		for l146 in 0..2 {
			self.fRec82[l146 as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fVec65[l147 as usize] = 0.0;
		}
		for l148 in 0..3 {
			self.fRec67[l148 as usize] = 0.0;
		}
		for l149 in 0..3 {
			self.fRec66[l149 as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.iRec85[l150 as usize] = 0;
		}
		for l151 in 0..2 {
			self.fRec84[l151 as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fVec66[l152 as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec88[l153 as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fVec67[l154 as usize] = 0.0;
		}
		for l155 in 0..4096 {
			self.fVec68[l155 as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec89[l156 as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.iRec91[l157 as usize] = 0;
		}
		for l158 in 0..2 {
			self.fRec90[l158 as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fVec69[l159 as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.iRec93[l160 as usize] = 0;
		}
		for l161 in 0..2 {
			self.fRec92[l161 as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fVec70[l162 as usize] = 0.0;
		}
		for l163 in 0..2 {
			self.fVec71[l163 as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec94[l164 as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fVec72[l165 as usize] = 0.0;
		}
		for l166 in 0..4096 {
			self.fVec73[l166 as usize] = 0.0;
		}
		for l167 in 0..2 {
			self.iRec96[l167 as usize] = 0;
		}
		for l168 in 0..2 {
			self.fRec95[l168 as usize] = 0.0;
		}
		for l169 in 0..2 {
			self.fVec74[l169 as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fVec75[l170 as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec97[l171 as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fVec76[l172 as usize] = 0.0;
		}
		for l173 in 0..4096 {
			self.fVec77[l173 as usize] = 0.0;
		}
		for l174 in 0..2 {
			self.fRec98[l174 as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec99[l175 as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fVec78[l176 as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec100[l177 as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fVec79[l178 as usize] = 0.0;
		}
		for l179 in 0..4096 {
			self.fVec80[l179 as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fRec101[l180 as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.iRec103[l181 as usize] = 0;
		}
		for l182 in 0..2 {
			self.fRec102[l182 as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fVec81[l183 as usize] = 0.0;
		}
		for l184 in 0..3 {
			self.fRec87[l184 as usize] = 0.0;
		}
		for l185 in 0..3 {
			self.fRec86[l185 as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.iRec105[l186 as usize] = 0;
		}
		for l187 in 0..2 {
			self.fRec104[l187 as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fVec82[l188 as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec108[l189 as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fVec83[l190 as usize] = 0.0;
		}
		for l191 in 0..4096 {
			self.fVec84[l191 as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fRec109[l192 as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.iRec111[l193 as usize] = 0;
		}
		for l194 in 0..2 {
			self.fRec110[l194 as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fVec85[l195 as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.iRec113[l196 as usize] = 0;
		}
		for l197 in 0..2 {
			self.fRec112[l197 as usize] = 0.0;
		}
		for l198 in 0..2 {
			self.fVec86[l198 as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fVec87[l199 as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fRec114[l200 as usize] = 0.0;
		}
		for l201 in 0..2 {
			self.fVec88[l201 as usize] = 0.0;
		}
		for l202 in 0..4096 {
			self.fVec89[l202 as usize] = 0.0;
		}
		for l203 in 0..2 {
			self.iRec116[l203 as usize] = 0;
		}
		for l204 in 0..2 {
			self.fRec115[l204 as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fVec90[l205 as usize] = 0.0;
		}
		for l206 in 0..2 {
			self.fVec91[l206 as usize] = 0.0;
		}
		for l207 in 0..2 {
			self.fRec117[l207 as usize] = 0.0;
		}
		for l208 in 0..2 {
			self.fVec92[l208 as usize] = 0.0;
		}
		for l209 in 0..4096 {
			self.fVec93[l209 as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fRec118[l210 as usize] = 0.0;
		}
		for l211 in 0..2 {
			self.fRec119[l211 as usize] = 0.0;
		}
		for l212 in 0..2 {
			self.fVec94[l212 as usize] = 0.0;
		}
		for l213 in 0..2 {
			self.fRec120[l213 as usize] = 0.0;
		}
		for l214 in 0..2 {
			self.fVec95[l214 as usize] = 0.0;
		}
		for l215 in 0..4096 {
			self.fVec96[l215 as usize] = 0.0;
		}
		for l216 in 0..2 {
			self.fRec121[l216 as usize] = 0.0;
		}
		for l217 in 0..2 {
			self.iRec123[l217 as usize] = 0;
		}
		for l218 in 0..2 {
			self.fRec122[l218 as usize] = 0.0;
		}
		for l219 in 0..2 {
			self.fVec97[l219 as usize] = 0.0;
		}
		for l220 in 0..3 {
			self.fRec107[l220 as usize] = 0.0;
		}
		for l221 in 0..3 {
			self.fRec106[l221 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = f32::min(192000.0, f32::max(1.0, (self.fSampleRate as f32)));
		self.fConst1 = (6.90999985 / self.fConst0);
		self.fConst2 = (0.25 * self.fConst0);
		self.fConst3 = (1.0 / self.fConst0);
		self.fConst4 = (3.0 / self.fConst0);
		self.fConst5 = (0.5 * self.fConst0);
		self.fConst6 = (2.0 * self.fConst0);
		self.fConst7 = (6.0 / self.fConst0);
		self.fConst8 = (3.14159274 / self.fConst0);
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
			9 => self.fButton0,
			12 => self.fButton1,
			15 => self.fButton2,
			18 => self.fButton3,
			21 => self.fButton4,
			24 => self.fButton5,
			10 => self.fHslider0,
			6 => self.fHslider1,
			13 => self.fHslider10,
			11 => self.fHslider11,
			16 => self.fHslider12,
			14 => self.fHslider13,
			19 => self.fHslider14,
			17 => self.fHslider15,
			22 => self.fHslider16,
			20 => self.fHslider17,
			25 => self.fHslider18,
			23 => self.fHslider19,
			4 => self.fHslider2,
			5 => self.fHslider3,
			7 => self.fHslider4,
			2 => self.fHslider5,
			3 => self.fHslider6,
			8 => self.fHslider7,
			1 => self.fHslider8,
			0 => self.fHslider9,
			_ => 0.,
		}
	}
	
	pub fn set_param(&mut self, param: u32, value: T) {
		match param {
			9 => { self.fButton0 = value }
			12 => { self.fButton1 = value }
			15 => { self.fButton2 = value }
			18 => { self.fButton3 = value }
			21 => { self.fButton4 = value }
			24 => { self.fButton5 = value }
			10 => { self.fHslider0 = value }
			6 => { self.fHslider1 = value }
			13 => { self.fHslider10 = value }
			11 => { self.fHslider11 = value }
			16 => { self.fHslider12 = value }
			14 => { self.fHslider13 = value }
			19 => { self.fHslider14 = value }
			17 => { self.fHslider15 = value }
			22 => { self.fHslider16 = value }
			20 => { self.fHslider17 = value }
			25 => { self.fHslider18 = value }
			23 => { self.fHslider19 = value }
			4 => { self.fHslider2 = value }
			5 => { self.fHslider3 = value }
			7 => { self.fHslider4 = value }
			2 => { self.fHslider5 = value }
			3 => { self.fHslider6 = value }
			8 => { self.fHslider7 = value }
			1 => { self.fHslider8 = value }
			0 => { self.fHslider9 = value }
			_ => {}
		}
	}
	#[target_feature(enable = "simd128")]
	#[inline]
	unsafe fn compute(&mut self, count: i32, inputs: &[T], outputs: &mut [&mut [T];1]) {
		let outputs0 = outputs[0][..count as usize].iter_mut();
		let mut fSlow0: f32 = (self.fHslider0 as f32);
		let mut iSlow1: i32 = (((self.fButton0 as f32) > 0.0) as i32);
		let mut fSlow2: f32 = (self.fHslider1 as f32);
		let mut fSlow3: f32 = (self.fHslider2 as f32);
		let mut iSlow4: i32 = ((self.fConst0 * fSlow3) as i32);
		let mut fSlow5: f32 = (self.fHslider3 as f32);
		let mut fSlow6: f32 = (6.90999985 * fSlow3);
		let mut fSlow7: f32 = (self.fHslider4 as f32);
		let mut fSlow8: f32 = (fSlow7 * (iSlow1 as f32));
		let mut fSlow9: f32 = (0.00100000005 * (self.fHslider5 as f32));
		let mut fSlow10: f32 = (0.00100000005 * (self.fHslider6 as f32));
		let mut fSlow11: f32 = (self.fHslider7 as f32);
		let mut fSlow12: f32 = f32::max((0.5 * fSlow11), 23.4489498);
		let mut fSlow13: f32 = f32::max(20.0, f32::abs(fSlow12));
		let mut fSlow14: f32 = (self.fConst2 / fSlow13);
		let mut fSlow15: f32 = (self.fConst0 / fSlow12);
		let mut fSlow16: f32 = (self.fConst4 * fSlow11);
		let mut fSlow17: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow12)));
		let mut fSlow18: f32 = f32::floor(fSlow17);
		let mut fSlow19: f32 = (fSlow18 + (1.0 - fSlow17));
		let mut iSlow20: i32 = (fSlow17 as i32);
		let mut fSlow21: f32 = (fSlow17 - fSlow18);
		let mut iSlow22: i32 = (iSlow20 + 1);
		let mut fSlow23: f32 = (self.fConst0 / fSlow11);
		let mut iSlow24: i32 = ((self.fConst6 / fSlow11) as i32);
		let mut fSlow25: f32 = (self.fConst3 * fSlow11);
		let mut fSlow26: f32 = (0.00100000005 * (self.fHslider8 as f32));
		let mut fSlow27: f32 = (1.0 / fSlow11);
		let mut fSlow28: f32 = (self.fConst7 * fSlow11);
		let mut fSlow29: f32 = f32::max(fSlow11, 23.4489498);
		let mut fSlow30: f32 = f32::max(20.0, f32::abs(fSlow29));
		let mut fSlow31: f32 = (self.fConst2 / fSlow30);
		let mut fSlow32: f32 = (self.fConst0 / fSlow29);
		let mut fSlow33: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow29)));
		let mut fSlow34: f32 = f32::floor(fSlow33);
		let mut fSlow35: f32 = (fSlow34 + (1.0 - fSlow33));
		let mut iSlow36: i32 = (fSlow33 as i32);
		let mut fSlow37: f32 = (fSlow33 - fSlow34);
		let mut iSlow38: i32 = (iSlow36 + 1);
		let mut iSlow39: i32 = (fSlow23 as i32);
		let mut fSlow40: f32 = (0.00100000005 * (self.fHslider9 as f32));
		let mut fSlow41: f32 = (self.fHslider10 as f32);
		let mut iSlow42: i32 = (((self.fButton1 as f32) > 0.0) as i32);
		let mut fSlow43: f32 = (fSlow7 * (iSlow42 as f32));
		let mut fSlow44: f32 = (self.fHslider11 as f32);
		let mut fSlow45: f32 = f32::max((0.5 * fSlow44), 23.4489498);
		let mut fSlow46: f32 = f32::max(20.0, f32::abs(fSlow45));
		let mut fSlow47: f32 = (self.fConst2 / fSlow46);
		let mut fSlow48: f32 = (self.fConst0 / fSlow45);
		let mut fSlow49: f32 = (self.fConst4 * fSlow44);
		let mut fSlow50: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow45)));
		let mut fSlow51: f32 = f32::floor(fSlow50);
		let mut fSlow52: f32 = (fSlow51 + (1.0 - fSlow50));
		let mut iSlow53: i32 = (fSlow50 as i32);
		let mut fSlow54: f32 = (fSlow50 - fSlow51);
		let mut iSlow55: i32 = (iSlow53 + 1);
		let mut fSlow56: f32 = (self.fConst0 / fSlow44);
		let mut iSlow57: i32 = ((self.fConst6 / fSlow44) as i32);
		let mut fSlow58: f32 = (self.fConst3 * fSlow44);
		let mut fSlow59: f32 = (1.0 / fSlow44);
		let mut fSlow60: f32 = (self.fConst7 * fSlow44);
		let mut fSlow61: f32 = f32::max(fSlow44, 23.4489498);
		let mut fSlow62: f32 = f32::max(20.0, f32::abs(fSlow61));
		let mut fSlow63: f32 = (self.fConst2 / fSlow62);
		let mut fSlow64: f32 = (self.fConst0 / fSlow61);
		let mut fSlow65: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow61)));
		let mut fSlow66: f32 = f32::floor(fSlow65);
		let mut fSlow67: f32 = (fSlow66 + (1.0 - fSlow65));
		let mut iSlow68: i32 = (fSlow65 as i32);
		let mut fSlow69: f32 = (fSlow65 - fSlow66);
		let mut iSlow70: i32 = (iSlow68 + 1);
		let mut iSlow71: i32 = (fSlow56 as i32);
		let mut fSlow72: f32 = (self.fHslider12 as f32);
		let mut iSlow73: i32 = (((self.fButton2 as f32) > 0.0) as i32);
		let mut fSlow74: f32 = (fSlow7 * (iSlow73 as f32));
		let mut fSlow75: f32 = (self.fHslider13 as f32);
		let mut fSlow76: f32 = f32::max((0.5 * fSlow75), 23.4489498);
		let mut fSlow77: f32 = f32::max(20.0, f32::abs(fSlow76));
		let mut fSlow78: f32 = (self.fConst2 / fSlow77);
		let mut fSlow79: f32 = (self.fConst0 / fSlow76);
		let mut fSlow80: f32 = (self.fConst4 * fSlow75);
		let mut fSlow81: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow76)));
		let mut fSlow82: f32 = f32::floor(fSlow81);
		let mut fSlow83: f32 = (fSlow82 + (1.0 - fSlow81));
		let mut iSlow84: i32 = (fSlow81 as i32);
		let mut fSlow85: f32 = (fSlow81 - fSlow82);
		let mut iSlow86: i32 = (iSlow84 + 1);
		let mut fSlow87: f32 = (self.fConst0 / fSlow75);
		let mut iSlow88: i32 = ((self.fConst6 / fSlow75) as i32);
		let mut fSlow89: f32 = (self.fConst3 * fSlow75);
		let mut fSlow90: f32 = (1.0 / fSlow75);
		let mut fSlow91: f32 = (self.fConst7 * fSlow75);
		let mut fSlow92: f32 = f32::max(fSlow75, 23.4489498);
		let mut fSlow93: f32 = f32::max(20.0, f32::abs(fSlow92));
		let mut fSlow94: f32 = (self.fConst2 / fSlow93);
		let mut fSlow95: f32 = (self.fConst0 / fSlow92);
		let mut fSlow96: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow92)));
		let mut fSlow97: f32 = f32::floor(fSlow96);
		let mut fSlow98: f32 = (fSlow97 + (1.0 - fSlow96));
		let mut iSlow99: i32 = (fSlow96 as i32);
		let mut fSlow100: f32 = (fSlow96 - fSlow97);
		let mut iSlow101: i32 = (iSlow99 + 1);
		let mut iSlow102: i32 = (fSlow87 as i32);
		let mut fSlow103: f32 = (self.fHslider14 as f32);
		let mut iSlow104: i32 = (((self.fButton3 as f32) > 0.0) as i32);
		let mut fSlow105: f32 = (fSlow7 * (iSlow104 as f32));
		let mut fSlow106: f32 = (self.fHslider15 as f32);
		let mut fSlow107: f32 = f32::max((0.5 * fSlow106), 23.4489498);
		let mut fSlow108: f32 = f32::max(20.0, f32::abs(fSlow107));
		let mut fSlow109: f32 = (self.fConst2 / fSlow108);
		let mut fSlow110: f32 = (self.fConst0 / fSlow107);
		let mut fSlow111: f32 = (self.fConst4 * fSlow106);
		let mut fSlow112: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow107)));
		let mut fSlow113: f32 = f32::floor(fSlow112);
		let mut fSlow114: f32 = (fSlow113 + (1.0 - fSlow112));
		let mut iSlow115: i32 = (fSlow112 as i32);
		let mut fSlow116: f32 = (fSlow112 - fSlow113);
		let mut iSlow117: i32 = (iSlow115 + 1);
		let mut fSlow118: f32 = (self.fConst0 / fSlow106);
		let mut iSlow119: i32 = ((self.fConst6 / fSlow106) as i32);
		let mut fSlow120: f32 = (self.fConst3 * fSlow106);
		let mut fSlow121: f32 = (1.0 / fSlow106);
		let mut fSlow122: f32 = (self.fConst7 * fSlow106);
		let mut fSlow123: f32 = f32::max(fSlow106, 23.4489498);
		let mut fSlow124: f32 = f32::max(20.0, f32::abs(fSlow123));
		let mut fSlow125: f32 = (self.fConst2 / fSlow124);
		let mut fSlow126: f32 = (self.fConst0 / fSlow123);
		let mut fSlow127: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow123)));
		let mut fSlow128: f32 = f32::floor(fSlow127);
		let mut fSlow129: f32 = (fSlow128 + (1.0 - fSlow127));
		let mut iSlow130: i32 = (fSlow127 as i32);
		let mut fSlow131: f32 = (fSlow127 - fSlow128);
		let mut iSlow132: i32 = (iSlow130 + 1);
		let mut iSlow133: i32 = (fSlow118 as i32);
		let mut fSlow134: f32 = (self.fHslider16 as f32);
		let mut iSlow135: i32 = (((self.fButton4 as f32) > 0.0) as i32);
		let mut fSlow136: f32 = (fSlow7 * (iSlow135 as f32));
		let mut fSlow137: f32 = (self.fHslider17 as f32);
		let mut fSlow138: f32 = f32::max((0.5 * fSlow137), 23.4489498);
		let mut fSlow139: f32 = f32::max(20.0, f32::abs(fSlow138));
		let mut fSlow140: f32 = (self.fConst2 / fSlow139);
		let mut fSlow141: f32 = (self.fConst0 / fSlow138);
		let mut fSlow142: f32 = (self.fConst4 * fSlow137);
		let mut fSlow143: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow138)));
		let mut fSlow144: f32 = f32::floor(fSlow143);
		let mut fSlow145: f32 = (fSlow144 + (1.0 - fSlow143));
		let mut iSlow146: i32 = (fSlow143 as i32);
		let mut fSlow147: f32 = (fSlow143 - fSlow144);
		let mut iSlow148: i32 = (iSlow146 + 1);
		let mut fSlow149: f32 = (self.fConst0 / fSlow137);
		let mut iSlow150: i32 = ((self.fConst6 / fSlow137) as i32);
		let mut fSlow151: f32 = (self.fConst3 * fSlow137);
		let mut fSlow152: f32 = (1.0 / fSlow137);
		let mut fSlow153: f32 = (self.fConst7 * fSlow137);
		let mut fSlow154: f32 = f32::max(fSlow137, 23.4489498);
		let mut fSlow155: f32 = f32::max(20.0, f32::abs(fSlow154));
		let mut fSlow156: f32 = (self.fConst2 / fSlow155);
		let mut fSlow157: f32 = (self.fConst0 / fSlow154);
		let mut fSlow158: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow154)));
		let mut fSlow159: f32 = f32::floor(fSlow158);
		let mut fSlow160: f32 = (fSlow159 + (1.0 - fSlow158));
		let mut iSlow161: i32 = (fSlow158 as i32);
		let mut fSlow162: f32 = (fSlow158 - fSlow159);
		let mut iSlow163: i32 = (iSlow161 + 1);
		let mut iSlow164: i32 = (fSlow149 as i32);
		let mut fSlow165: f32 = (self.fHslider18 as f32);
		let mut iSlow166: i32 = (((self.fButton5 as f32) > 0.0) as i32);
		let mut fSlow167: f32 = (fSlow7 * (iSlow166 as f32));
		let mut fSlow168: f32 = (self.fHslider19 as f32);
		let mut fSlow169: f32 = f32::max((0.5 * fSlow168), 23.4489498);
		let mut fSlow170: f32 = f32::max(20.0, f32::abs(fSlow169));
		let mut fSlow171: f32 = (self.fConst2 / fSlow170);
		let mut fSlow172: f32 = (self.fConst0 / fSlow169);
		let mut fSlow173: f32 = (self.fConst4 * fSlow168);
		let mut fSlow174: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow169)));
		let mut fSlow175: f32 = f32::floor(fSlow174);
		let mut fSlow176: f32 = (fSlow175 + (1.0 - fSlow174));
		let mut iSlow177: i32 = (fSlow174 as i32);
		let mut fSlow178: f32 = (fSlow174 - fSlow175);
		let mut iSlow179: i32 = (iSlow177 + 1);
		let mut fSlow180: f32 = (self.fConst0 / fSlow168);
		let mut iSlow181: i32 = ((self.fConst6 / fSlow168) as i32);
		let mut fSlow182: f32 = (self.fConst3 * fSlow168);
		let mut fSlow183: f32 = (1.0 / fSlow168);
		let mut fSlow184: f32 = (self.fConst7 * fSlow168);
		let mut fSlow185: f32 = f32::max(fSlow168, 23.4489498);
		let mut fSlow186: f32 = f32::max(20.0, f32::abs(fSlow185));
		let mut fSlow187: f32 = (self.fConst2 / fSlow186);
		let mut fSlow188: f32 = (self.fConst0 / fSlow185);
		let mut fSlow189: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fSlow185)));
		let mut fSlow190: f32 = f32::floor(fSlow189);
		let mut fSlow191: f32 = (fSlow190 + (1.0 - fSlow189));
		let mut iSlow192: i32 = (fSlow189 as i32);
		let mut fSlow193: f32 = (fSlow189 - fSlow190);
		let mut iSlow194: i32 = (iSlow192 + 1);
		let mut iSlow195: i32 = (fSlow180 as i32);
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.iVec0[0] = 1;
			self.iRec1[0] = (iSlow1 * (self.iRec1[1] + 1));
			let mut iTemp0: i32 = ((self.iRec1[0] < iSlow4) as i32);
			let mut fTemp1: f32 = f32::exp((0.0 - (self.fConst1 / if (iSlow1 as i32 != 0) { if (iTemp0 as i32 != 0) { fSlow6 } else { fSlow5 } } else { fSlow2 })));
			self.fRec0[0] = ((self.fRec0[1] * fTemp1) + (if (iSlow1 as i32 != 0) { if (iTemp0 as i32 != 0) { 1.58730161 } else { fSlow8 } } else { 0.0 } * (1.0 - fTemp1)));
			self.fRec4[0] = (fSlow9 + (0.999000013 * self.fRec4[1]));
			self.fRec5[0] = (fSlow10 + (0.999000013 * self.fRec5[1]));
			let mut fTemp2: f32 = f32::min(1.0, f32::max(0.0, (2.0 - self.fRec5[0])));
			self.fVec1[0] = 0.25;
			self.fVec2[0] = fSlow13;
			let mut fTemp3: f32 = (self.iVec0[1] as f32);
			let mut fTemp4: f32 = (self.fRec6[1] + (self.fConst3 * self.fVec2[1]));
			self.fRec6[0] = (fTemp4 - f32::floor(fTemp4));
			let mut fTemp5: f32 = mydsp_faustpower2_f(((2.0 * self.fRec6[0]) + -1.0));
			self.fVec3[0] = fTemp5;
			let mut fTemp6: f32 = (fSlow14 * (fTemp3 * (fTemp5 - self.fVec3[1])));
			self.fVec4[(self.IOTA & 4095) as usize] = fTemp6;
			let mut fTemp7: f32 = f32::min(0.5, (0.5 * self.fRec5[0]));
			let mut fTemp8: f32 = f32::max(0.0, f32::min(2047.0, (fSlow15 * fTemp7)));
			let mut iTemp9: i32 = (fTemp8 as i32);
			let mut fTemp10: f32 = f32::floor(fTemp8);
			self.fRec7[0] = ((fTemp6 + (0.999000013 * self.fRec7[1])) - ((fSlow19 * self.fVec4[((self.IOTA - iSlow20) & 4095) as usize]) + (fSlow21 * self.fVec4[((self.IOTA - iSlow22) & 4095) as usize])));
			let mut fTemp11: f32 = f32::min(1.0, f32::max(0.0, (self.fRec5[0] + -2.0)));
			let mut fTemp12: f32 = (1.0 - (fTemp2 + fTemp11));
			let mut fTemp13: f32 = (self.fVec1[1] * fTemp11);
			self.iRec9[0] = ((self.iVec0[1] + self.iRec9[1]) % iSlow24);
			let mut fTemp14: f32 = (0.100000001 * (f32::max(3.0, self.fRec5[0]) + -3.0));
			let mut fTemp15: f32 = (fTemp14 + 0.5);
			let mut fTemp16: f32 = ((self.fRec8[1] * (1.0 - (((((self.iRec9[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow25 * fTemp15));
			self.fRec8[0] = (fTemp16 - f32::floor(fTemp16));
			let mut fTemp17: f32 = mydsp_faustpower2_f(((2.0 * self.fRec8[0]) + -1.0));
			self.fVec5[0] = fTemp17;
			self.fRec10[0] = (fSlow26 + (0.999000013 * self.fRec10[1]));
			let mut fTemp18: f32 = (((self.fRec10[0] >= 2.0) as i32) as f32);
			let mut iTemp19: i32 = ((self.fRec10[0] >= 3.0) as i32);
			let mut fTemp20: f32 = if ((((self.fRec10[0] == 0.0) as i32) + iTemp19) as i32 != 0) { 1.0 } else { f32::max(f32::max(1.0, ((0.0199999996 * (self.fRec10[0] + -2.0999999)) + 1.0)), ((0.0199999996 * (1.0 - self.fRec10[0])) + 1.0)) };
			self.iRec12[0] = ((self.iVec0[1] + self.iRec12[1]) % ((fSlow23 / fTemp20) as i32));
			let mut fTemp21: f32 = (fTemp20 + fTemp14);
			let mut fTemp22: f32 = ((self.fRec11[1] * (1.0 - (((((self.iRec12[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow25 * fTemp21));
			self.fRec11[0] = (fTemp22 - f32::floor(fTemp22));
			let mut fTemp23: f32 = mydsp_faustpower2_f(((2.0 * self.fRec11[0]) + -1.0));
			self.fVec6[0] = fTemp23;
			let mut fTemp24: f32 = f32::max((fSlow11 * fTemp20), 23.4489498);
			let mut fTemp25: f32 = f32::max(20.0, f32::abs(fTemp24));
			self.fVec7[0] = fTemp25;
			let mut fTemp26: f32 = (self.fRec13[1] + (self.fConst3 * self.fVec7[1]));
			self.fRec13[0] = (fTemp26 - f32::floor(fTemp26));
			let mut fTemp27: f32 = mydsp_faustpower2_f(((2.0 * self.fRec13[0]) + -1.0));
			self.fVec8[0] = fTemp27;
			let mut fTemp28: f32 = ((fTemp3 * (fTemp27 - self.fVec8[1])) / fTemp25);
			self.fVec9[(self.IOTA & 4095) as usize] = fTemp28;
			let mut fTemp29: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp24))));
			let mut iTemp30: i32 = (fTemp29 as i32);
			let mut fTemp31: f32 = f32::floor(fTemp29);
			let mut fTemp32: f32 = ((fSlow27 * ((fTemp13 * (fTemp23 - self.fVec6[1])) / fTemp21)) + (0.25 * (fTemp2 * ((fTemp28 - (self.fVec9[((self.IOTA - iTemp30) & 4095) as usize] * (fTemp31 + (1.0 - fTemp29)))) - ((fTemp29 - fTemp31) * self.fVec9[((self.IOTA - (iTemp30 + 1)) & 4095) as usize])))));
			let mut fTemp33: f32 = if (iTemp19 as i32 != 0) { 1.49829996 } else { 1.0 };
			let mut fTemp34: f32 = (fTemp20 / fTemp33);
			self.iRec15[0] = ((self.iVec0[1] + self.iRec15[1]) % ((fSlow23 * fTemp34) as i32));
			let mut fTemp35: f32 = (fTemp33 / fTemp20);
			let mut fTemp36: f32 = (fTemp14 + fTemp35);
			let mut fTemp37: f32 = ((self.fRec14[1] * (1.0 - (((((self.iRec15[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow25 * fTemp36));
			self.fRec14[0] = (fTemp37 - f32::floor(fTemp37));
			let mut fTemp38: f32 = mydsp_faustpower2_f(((2.0 * self.fRec14[0]) + -1.0));
			self.fVec10[0] = fTemp38;
			let mut fTemp39: f32 = f32::max((fSlow11 * fTemp35), 23.4489498);
			let mut fTemp40: f32 = f32::max(20.0, f32::abs(fTemp39));
			self.fVec11[0] = fTemp40;
			let mut fTemp41: f32 = (self.fRec16[1] + (self.fConst3 * self.fVec11[1]));
			self.fRec16[0] = (fTemp41 - f32::floor(fTemp41));
			let mut fTemp42: f32 = mydsp_faustpower2_f(((2.0 * self.fRec16[0]) + -1.0));
			self.fVec12[0] = fTemp42;
			let mut fTemp43: f32 = ((fTemp3 * (fTemp42 - self.fVec12[1])) / fTemp40);
			self.fVec13[(self.IOTA & 4095) as usize] = fTemp43;
			let mut fTemp44: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp39))));
			let mut iTemp45: i32 = (fTemp44 as i32);
			let mut fTemp46: f32 = f32::floor(fTemp44);
			let mut fTemp47: f32 = ((fSlow27 * ((fTemp13 * (fTemp38 - self.fVec10[1])) / fTemp36)) + (0.25 * (fTemp2 * ((fTemp43 - (self.fVec13[((self.IOTA - iTemp45) & 4095) as usize] * (fTemp46 + (1.0 - fTemp44)))) - ((fTemp44 - fTemp46) * self.fVec13[((self.IOTA - (iTemp45 + 1)) & 4095) as usize])))));
			let mut fTemp48: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp24)));
			let mut iTemp49: i32 = (fTemp48 as i32);
			let mut fTemp50: f32 = f32::floor(fTemp48);
			self.fRec17[0] = ((0.999000013 * self.fRec17[1]) + (self.fConst2 * ((fTemp28 - (self.fVec9[((self.IOTA - iTemp49) & 4095) as usize] * (fTemp50 + (1.0 - fTemp48)))) - ((fTemp48 - fTemp50) * self.fVec9[((self.IOTA - (iTemp49 + 1)) & 4095) as usize]))));
			let mut fTemp51: f32 = (self.fRec17[0] * fTemp20);
			let mut fTemp52: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp39)));
			let mut iTemp53: i32 = (fTemp52 as i32);
			let mut fTemp54: f32 = f32::floor(fTemp52);
			self.fRec18[0] = ((0.999000013 * self.fRec18[1]) + (self.fConst2 * ((fTemp43 - (self.fVec13[((self.IOTA - iTemp53) & 4095) as usize] * (fTemp54 + (1.0 - fTemp52)))) - ((fTemp52 - fTemp54) * self.fVec13[((self.IOTA - (iTemp53 + 1)) & 4095) as usize]))));
			let mut fTemp55: f32 = (self.fRec18[0] * fTemp33);
			let mut fTemp56: f32 = (((self.fRec10[0] < 2.0) as i32) as f32);
			let mut fTemp57: f32 = f32::min(1.0, f32::max(0.0, (2.0 - self.fRec10[0])));
			let mut fTemp58: f32 = (1.0 - fTemp57);
			let mut fTemp59: f32 = (1.0 - self.fRec4[0]);
			self.fVec14[0] = fSlow30;
			let mut fTemp60: f32 = (self.fRec19[1] + (self.fConst3 * self.fVec14[1]));
			self.fRec19[0] = (fTemp60 - f32::floor(fTemp60));
			let mut fTemp61: f32 = mydsp_faustpower2_f(((2.0 * self.fRec19[0]) + -1.0));
			self.fVec15[0] = fTemp61;
			let mut fTemp62: f32 = (fSlow31 * (fTemp3 * (fTemp61 - self.fVec15[1])));
			self.fVec16[(self.IOTA & 4095) as usize] = fTemp62;
			let mut fTemp63: f32 = f32::max(0.0, f32::min(2047.0, (fSlow32 * fTemp7)));
			let mut iTemp64: i32 = (fTemp63 as i32);
			let mut fTemp65: f32 = f32::floor(fTemp63);
			self.fRec20[0] = ((fTemp62 + (0.999000013 * self.fRec20[1])) - ((fSlow35 * self.fVec16[((self.IOTA - iSlow36) & 4095) as usize]) + (fSlow37 * self.fVec16[((self.IOTA - iSlow38) & 4095) as usize])));
			self.iRec22[0] = ((self.iVec0[1] + self.iRec22[1]) % iSlow39);
			let mut fTemp66: f32 = (fTemp14 + 1.0);
			let mut fTemp67: f32 = ((self.fRec21[1] * (1.0 - (((((self.iRec22[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow25 * fTemp66));
			self.fRec21[0] = (fTemp67 - f32::floor(fTemp67));
			let mut fTemp68: f32 = mydsp_faustpower2_f(((2.0 * self.fRec21[0]) + -1.0));
			self.fVec17[0] = fTemp68;
			self.fRec23[0] = (fSlow40 + (0.999000013 * self.fRec23[1]));
			let mut fTemp69: f32 = f32::tan((self.fConst8 * ((10000.0 * mydsp_faustpower2_f(self.fRec23[0])) + 100.0)));
			let mut fTemp70: f32 = (1.0 / fTemp69);
			let mut fTemp71: f32 = (((fTemp70 + -0.800000012) / fTemp69) + 1.0);
			let mut fTemp72: f32 = (1.0 - (1.0 / mydsp_faustpower2_f(fTemp69)));
			let mut fTemp73: f32 = (((fTemp70 + 0.800000012) / fTemp69) + 1.0);
			self.fRec3[0] = (((self.fRec4[0] * (((fTemp2 * (fTemp6 - ((self.fVec4[((self.IOTA - iTemp9) & 4095) as usize] * (fTemp10 + (1.0 - fTemp8))) + ((fTemp8 - fTemp10) * self.fVec4[((self.IOTA - (iTemp9 + 1)) & 4095) as usize])))) + (fSlow16 * (self.fRec7[0] * fTemp12))) + (fSlow23 * ((fTemp13 * (fTemp17 - self.fVec5[1])) / fTemp15)))) + (((fTemp18 * ((self.fConst0 * (fTemp32 + fTemp47)) + (fSlow28 * (fTemp12 * (fTemp51 + (fTemp55 / fTemp20)))))) + ((fTemp56 * ((fSlow28 * (fTemp51 * fTemp12)) + (self.fConst0 * fTemp32))) * (fTemp58 + (fTemp57 * ((fSlow28 * ((fTemp55 * fTemp12) / fTemp20)) + (self.fConst0 * fTemp47)))))) + (fTemp59 * (((fTemp2 * (fTemp62 - ((self.fVec16[((self.IOTA - iTemp64) & 4095) as usize] * (fTemp65 + (1.0 - fTemp63))) + ((fTemp63 - fTemp65) * self.fVec16[((self.IOTA - (iTemp64 + 1)) & 4095) as usize])))) + (fSlow28 * (self.fRec20[0] * fTemp12))) + (fSlow23 * ((fTemp13 * (fTemp68 - self.fVec17[1])) / fTemp66)))))) - (((self.fRec3[2] * fTemp71) + (2.0 * (self.fRec3[1] * fTemp72))) / fTemp73));
			self.fRec2[0] = ((((self.fRec3[1] + (0.5 * self.fRec3[0])) + (0.5 * self.fRec3[2])) - ((fTemp71 * self.fRec2[2]) + (2.0 * (fTemp72 * self.fRec2[1])))) / fTemp73);
			self.iRec25[0] = (iSlow42 * (self.iRec25[1] + 1));
			let mut iTemp74: i32 = ((self.iRec25[0] < iSlow4) as i32);
			let mut fTemp75: f32 = f32::exp((0.0 - (self.fConst1 / if (iSlow42 as i32 != 0) { if (iTemp74 as i32 != 0) { fSlow6 } else { fSlow5 } } else { fSlow2 })));
			self.fRec24[0] = ((self.fRec24[1] * fTemp75) + (if (iSlow42 as i32 != 0) { if (iTemp74 as i32 != 0) { 1.58730161 } else { fSlow43 } } else { 0.0 } * (1.0 - fTemp75)));
			self.fVec18[0] = fSlow46;
			let mut fTemp76: f32 = (self.fRec28[1] + (self.fConst3 * self.fVec18[1]));
			self.fRec28[0] = (fTemp76 - f32::floor(fTemp76));
			let mut fTemp77: f32 = mydsp_faustpower2_f(((2.0 * self.fRec28[0]) + -1.0));
			self.fVec19[0] = fTemp77;
			let mut fTemp78: f32 = (fSlow47 * (fTemp3 * (fTemp77 - self.fVec19[1])));
			self.fVec20[(self.IOTA & 4095) as usize] = fTemp78;
			let mut fTemp79: f32 = f32::max(0.0, f32::min(2047.0, (fSlow48 * fTemp7)));
			let mut iTemp80: i32 = (fTemp79 as i32);
			let mut fTemp81: f32 = f32::floor(fTemp79);
			self.fRec29[0] = ((fTemp78 + (0.999000013 * self.fRec29[1])) - ((fSlow52 * self.fVec20[((self.IOTA - iSlow53) & 4095) as usize]) + (fSlow54 * self.fVec20[((self.IOTA - iSlow55) & 4095) as usize])));
			self.iRec31[0] = ((self.iVec0[1] + self.iRec31[1]) % iSlow57);
			let mut fTemp82: f32 = ((self.fRec30[1] * (1.0 - (((((self.iRec31[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow58 * fTemp15));
			self.fRec30[0] = (fTemp82 - f32::floor(fTemp82));
			let mut fTemp83: f32 = mydsp_faustpower2_f(((2.0 * self.fRec30[0]) + -1.0));
			self.fVec21[0] = fTemp83;
			self.iRec33[0] = ((self.iVec0[1] + self.iRec33[1]) % ((fSlow56 / fTemp20) as i32));
			let mut fTemp84: f32 = ((self.fRec32[1] * (1.0 - (((((self.iRec33[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow58 * fTemp21));
			self.fRec32[0] = (fTemp84 - f32::floor(fTemp84));
			let mut fTemp85: f32 = mydsp_faustpower2_f(((2.0 * self.fRec32[0]) + -1.0));
			self.fVec22[0] = fTemp85;
			let mut fTemp86: f32 = f32::max((fSlow44 * fTemp20), 23.4489498);
			let mut fTemp87: f32 = f32::max(20.0, f32::abs(fTemp86));
			self.fVec23[0] = fTemp87;
			let mut fTemp88: f32 = (self.fRec34[1] + (self.fConst3 * self.fVec23[1]));
			self.fRec34[0] = (fTemp88 - f32::floor(fTemp88));
			let mut fTemp89: f32 = mydsp_faustpower2_f(((2.0 * self.fRec34[0]) + -1.0));
			self.fVec24[0] = fTemp89;
			let mut fTemp90: f32 = ((fTemp3 * (fTemp89 - self.fVec24[1])) / fTemp87);
			self.fVec25[(self.IOTA & 4095) as usize] = fTemp90;
			let mut fTemp91: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp86))));
			let mut iTemp92: i32 = (fTemp91 as i32);
			let mut fTemp93: f32 = f32::floor(fTemp91);
			let mut fTemp94: f32 = ((fSlow59 * ((fTemp13 * (fTemp85 - self.fVec22[1])) / fTemp21)) + (0.25 * (fTemp2 * ((fTemp90 - (self.fVec25[((self.IOTA - iTemp92) & 4095) as usize] * (fTemp93 + (1.0 - fTemp91)))) - ((fTemp91 - fTemp93) * self.fVec25[((self.IOTA - (iTemp92 + 1)) & 4095) as usize])))));
			self.iRec36[0] = ((self.iVec0[1] + self.iRec36[1]) % ((fSlow56 * fTemp34) as i32));
			let mut fTemp95: f32 = ((self.fRec35[1] * (1.0 - (((((self.iRec36[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow58 * fTemp36));
			self.fRec35[0] = (fTemp95 - f32::floor(fTemp95));
			let mut fTemp96: f32 = mydsp_faustpower2_f(((2.0 * self.fRec35[0]) + -1.0));
			self.fVec26[0] = fTemp96;
			let mut fTemp97: f32 = f32::max((fSlow44 * fTemp35), 23.4489498);
			let mut fTemp98: f32 = f32::max(20.0, f32::abs(fTemp97));
			self.fVec27[0] = fTemp98;
			let mut fTemp99: f32 = (self.fRec37[1] + (self.fConst3 * self.fVec27[1]));
			self.fRec37[0] = (fTemp99 - f32::floor(fTemp99));
			let mut fTemp100: f32 = mydsp_faustpower2_f(((2.0 * self.fRec37[0]) + -1.0));
			self.fVec28[0] = fTemp100;
			let mut fTemp101: f32 = ((fTemp3 * (fTemp100 - self.fVec28[1])) / fTemp98);
			self.fVec29[(self.IOTA & 4095) as usize] = fTemp101;
			let mut fTemp102: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp97))));
			let mut iTemp103: i32 = (fTemp102 as i32);
			let mut fTemp104: f32 = f32::floor(fTemp102);
			let mut fTemp105: f32 = ((fSlow59 * ((fTemp13 * (fTemp96 - self.fVec26[1])) / fTemp36)) + (0.25 * (fTemp2 * ((fTemp101 - (self.fVec29[((self.IOTA - iTemp103) & 4095) as usize] * (fTemp104 + (1.0 - fTemp102)))) - ((fTemp102 - fTemp104) * self.fVec29[((self.IOTA - (iTemp103 + 1)) & 4095) as usize])))));
			let mut fTemp106: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp86)));
			let mut iTemp107: i32 = (fTemp106 as i32);
			let mut fTemp108: f32 = f32::floor(fTemp106);
			self.fRec38[0] = ((0.999000013 * self.fRec38[1]) + (self.fConst2 * ((fTemp90 - (self.fVec25[((self.IOTA - iTemp107) & 4095) as usize] * (fTemp108 + (1.0 - fTemp106)))) - ((fTemp106 - fTemp108) * self.fVec25[((self.IOTA - (iTemp107 + 1)) & 4095) as usize]))));
			let mut fTemp109: f32 = (self.fRec38[0] * fTemp20);
			let mut fTemp110: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp97)));
			let mut iTemp111: i32 = (fTemp110 as i32);
			let mut fTemp112: f32 = f32::floor(fTemp110);
			self.fRec39[0] = ((0.999000013 * self.fRec39[1]) + (self.fConst2 * ((fTemp101 - (self.fVec29[((self.IOTA - iTemp111) & 4095) as usize] * (fTemp112 + (1.0 - fTemp110)))) - ((fTemp110 - fTemp112) * self.fVec29[((self.IOTA - (iTemp111 + 1)) & 4095) as usize]))));
			let mut fTemp113: f32 = (self.fRec39[0] * fTemp33);
			self.fVec30[0] = fSlow62;
			let mut fTemp114: f32 = (self.fRec40[1] + (self.fConst3 * self.fVec30[1]));
			self.fRec40[0] = (fTemp114 - f32::floor(fTemp114));
			let mut fTemp115: f32 = mydsp_faustpower2_f(((2.0 * self.fRec40[0]) + -1.0));
			self.fVec31[0] = fTemp115;
			let mut fTemp116: f32 = (fSlow63 * (fTemp3 * (fTemp115 - self.fVec31[1])));
			self.fVec32[(self.IOTA & 4095) as usize] = fTemp116;
			let mut fTemp117: f32 = f32::max(0.0, f32::min(2047.0, (fSlow64 * fTemp7)));
			let mut iTemp118: i32 = (fTemp117 as i32);
			let mut fTemp119: f32 = f32::floor(fTemp117);
			self.fRec41[0] = ((fTemp116 + (0.999000013 * self.fRec41[1])) - ((fSlow67 * self.fVec32[((self.IOTA - iSlow68) & 4095) as usize]) + (fSlow69 * self.fVec32[((self.IOTA - iSlow70) & 4095) as usize])));
			self.iRec43[0] = ((self.iVec0[1] + self.iRec43[1]) % iSlow71);
			let mut fTemp120: f32 = ((self.fRec42[1] * (1.0 - (((((self.iRec43[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow58 * fTemp66));
			self.fRec42[0] = (fTemp120 - f32::floor(fTemp120));
			let mut fTemp121: f32 = mydsp_faustpower2_f(((2.0 * self.fRec42[0]) + -1.0));
			self.fVec33[0] = fTemp121;
			self.fRec27[0] = (((self.fRec4[0] * (((fTemp2 * (fTemp78 - ((self.fVec20[((self.IOTA - iTemp80) & 4095) as usize] * (fTemp81 + (1.0 - fTemp79))) + ((fTemp79 - fTemp81) * self.fVec20[((self.IOTA - (iTemp80 + 1)) & 4095) as usize])))) + (fSlow49 * (self.fRec29[0] * fTemp12))) + (fSlow56 * ((fTemp13 * (fTemp83 - self.fVec21[1])) / fTemp15)))) + (((fTemp18 * ((self.fConst0 * (fTemp94 + fTemp105)) + (fSlow60 * (fTemp12 * (fTemp109 + (fTemp113 / fTemp20)))))) + ((fTemp56 * ((fSlow60 * (fTemp109 * fTemp12)) + (self.fConst0 * fTemp94))) * (fTemp58 + (fTemp57 * ((fSlow60 * ((fTemp113 * fTemp12) / fTemp20)) + (self.fConst0 * fTemp105)))))) + (fTemp59 * (((fTemp2 * (fTemp116 - ((self.fVec32[((self.IOTA - iTemp118) & 4095) as usize] * (fTemp119 + (1.0 - fTemp117))) + ((fTemp117 - fTemp119) * self.fVec32[((self.IOTA - (iTemp118 + 1)) & 4095) as usize])))) + (fSlow60 * (self.fRec41[0] * fTemp12))) + (fSlow56 * ((fTemp13 * (fTemp121 - self.fVec33[1])) / fTemp66)))))) - (((fTemp71 * self.fRec27[2]) + (2.0 * (fTemp72 * self.fRec27[1]))) / fTemp73));
			self.fRec26[0] = ((((self.fRec27[1] + (0.5 * self.fRec27[0])) + (0.5 * self.fRec27[2])) - ((fTemp71 * self.fRec26[2]) + (2.0 * (fTemp72 * self.fRec26[1])))) / fTemp73);
			self.iRec45[0] = (iSlow73 * (self.iRec45[1] + 1));
			let mut iTemp122: i32 = ((self.iRec45[0] < iSlow4) as i32);
			let mut fTemp123: f32 = f32::exp((0.0 - (self.fConst1 / if (iSlow73 as i32 != 0) { if (iTemp122 as i32 != 0) { fSlow6 } else { fSlow5 } } else { fSlow2 })));
			self.fRec44[0] = ((self.fRec44[1] * fTemp123) + (if (iSlow73 as i32 != 0) { if (iTemp122 as i32 != 0) { 1.58730161 } else { fSlow74 } } else { 0.0 } * (1.0 - fTemp123)));
			self.fVec34[0] = fSlow77;
			let mut fTemp124: f32 = (self.fRec48[1] + (self.fConst3 * self.fVec34[1]));
			self.fRec48[0] = (fTemp124 - f32::floor(fTemp124));
			let mut fTemp125: f32 = mydsp_faustpower2_f(((2.0 * self.fRec48[0]) + -1.0));
			self.fVec35[0] = fTemp125;
			let mut fTemp126: f32 = (fSlow78 * (fTemp3 * (fTemp125 - self.fVec35[1])));
			self.fVec36[(self.IOTA & 4095) as usize] = fTemp126;
			let mut fTemp127: f32 = f32::max(0.0, f32::min(2047.0, (fSlow79 * fTemp7)));
			let mut iTemp128: i32 = (fTemp127 as i32);
			let mut fTemp129: f32 = f32::floor(fTemp127);
			self.fRec49[0] = ((fTemp126 + (0.999000013 * self.fRec49[1])) - ((fSlow83 * self.fVec36[((self.IOTA - iSlow84) & 4095) as usize]) + (fSlow85 * self.fVec36[((self.IOTA - iSlow86) & 4095) as usize])));
			self.iRec51[0] = ((self.iVec0[1] + self.iRec51[1]) % iSlow88);
			let mut fTemp130: f32 = ((self.fRec50[1] * (1.0 - (((((self.iRec51[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow89 * fTemp15));
			self.fRec50[0] = (fTemp130 - f32::floor(fTemp130));
			let mut fTemp131: f32 = mydsp_faustpower2_f(((2.0 * self.fRec50[0]) + -1.0));
			self.fVec37[0] = fTemp131;
			self.iRec53[0] = ((self.iVec0[1] + self.iRec53[1]) % ((fSlow87 / fTemp20) as i32));
			let mut fTemp132: f32 = ((self.fRec52[1] * (1.0 - (((((self.iRec53[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow89 * fTemp21));
			self.fRec52[0] = (fTemp132 - f32::floor(fTemp132));
			let mut fTemp133: f32 = mydsp_faustpower2_f(((2.0 * self.fRec52[0]) + -1.0));
			self.fVec38[0] = fTemp133;
			let mut fTemp134: f32 = f32::max((fSlow75 * fTemp20), 23.4489498);
			let mut fTemp135: f32 = f32::max(20.0, f32::abs(fTemp134));
			self.fVec39[0] = fTemp135;
			let mut fTemp136: f32 = (self.fRec54[1] + (self.fConst3 * self.fVec39[1]));
			self.fRec54[0] = (fTemp136 - f32::floor(fTemp136));
			let mut fTemp137: f32 = mydsp_faustpower2_f(((2.0 * self.fRec54[0]) + -1.0));
			self.fVec40[0] = fTemp137;
			let mut fTemp138: f32 = ((fTemp3 * (fTemp137 - self.fVec40[1])) / fTemp135);
			self.fVec41[(self.IOTA & 4095) as usize] = fTemp138;
			let mut fTemp139: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp134))));
			let mut iTemp140: i32 = (fTemp139 as i32);
			let mut fTemp141: f32 = f32::floor(fTemp139);
			let mut fTemp142: f32 = ((fSlow90 * ((fTemp13 * (fTemp133 - self.fVec38[1])) / fTemp21)) + (0.25 * (fTemp2 * ((fTemp138 - (self.fVec41[((self.IOTA - iTemp140) & 4095) as usize] * (fTemp141 + (1.0 - fTemp139)))) - ((fTemp139 - fTemp141) * self.fVec41[((self.IOTA - (iTemp140 + 1)) & 4095) as usize])))));
			self.iRec56[0] = ((self.iVec0[1] + self.iRec56[1]) % ((fSlow87 * fTemp34) as i32));
			let mut fTemp143: f32 = ((self.fRec55[1] * (1.0 - (((((self.iRec56[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow89 * fTemp36));
			self.fRec55[0] = (fTemp143 - f32::floor(fTemp143));
			let mut fTemp144: f32 = mydsp_faustpower2_f(((2.0 * self.fRec55[0]) + -1.0));
			self.fVec42[0] = fTemp144;
			let mut fTemp145: f32 = f32::max((fSlow75 * fTemp35), 23.4489498);
			let mut fTemp146: f32 = f32::max(20.0, f32::abs(fTemp145));
			self.fVec43[0] = fTemp146;
			let mut fTemp147: f32 = (self.fRec57[1] + (self.fConst3 * self.fVec43[1]));
			self.fRec57[0] = (fTemp147 - f32::floor(fTemp147));
			let mut fTemp148: f32 = mydsp_faustpower2_f(((2.0 * self.fRec57[0]) + -1.0));
			self.fVec44[0] = fTemp148;
			let mut fTemp149: f32 = ((fTemp3 * (fTemp148 - self.fVec44[1])) / fTemp146);
			self.fVec45[(self.IOTA & 4095) as usize] = fTemp149;
			let mut fTemp150: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp145))));
			let mut iTemp151: i32 = (fTemp150 as i32);
			let mut fTemp152: f32 = f32::floor(fTemp150);
			let mut fTemp153: f32 = ((fSlow90 * ((fTemp13 * (fTemp144 - self.fVec42[1])) / fTemp36)) + (0.25 * (fTemp2 * ((fTemp149 - (self.fVec45[((self.IOTA - iTemp151) & 4095) as usize] * (fTemp152 + (1.0 - fTemp150)))) - ((fTemp150 - fTemp152) * self.fVec45[((self.IOTA - (iTemp151 + 1)) & 4095) as usize])))));
			let mut fTemp154: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp134)));
			let mut iTemp155: i32 = (fTemp154 as i32);
			let mut fTemp156: f32 = f32::floor(fTemp154);
			self.fRec58[0] = ((0.999000013 * self.fRec58[1]) + (self.fConst2 * ((fTemp138 - (self.fVec41[((self.IOTA - iTemp155) & 4095) as usize] * (fTemp156 + (1.0 - fTemp154)))) - ((fTemp154 - fTemp156) * self.fVec41[((self.IOTA - (iTemp155 + 1)) & 4095) as usize]))));
			let mut fTemp157: f32 = (self.fRec58[0] * fTemp20);
			let mut fTemp158: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp145)));
			let mut iTemp159: i32 = (fTemp158 as i32);
			let mut fTemp160: f32 = f32::floor(fTemp158);
			self.fRec59[0] = ((0.999000013 * self.fRec59[1]) - (self.fConst2 * (((self.fVec45[((self.IOTA - iTemp159) & 4095) as usize] * (fTemp160 + (1.0 - fTemp158))) - fTemp149) + ((fTemp158 - fTemp160) * self.fVec45[((self.IOTA - (iTemp159 + 1)) & 4095) as usize]))));
			let mut fTemp161: f32 = (self.fRec59[0] * fTemp33);
			self.fVec46[0] = fSlow93;
			let mut fTemp162: f32 = (self.fRec60[1] + (self.fConst3 * self.fVec46[1]));
			self.fRec60[0] = (fTemp162 - f32::floor(fTemp162));
			let mut fTemp163: f32 = mydsp_faustpower2_f(((2.0 * self.fRec60[0]) + -1.0));
			self.fVec47[0] = fTemp163;
			let mut fTemp164: f32 = (fSlow94 * (fTemp3 * (fTemp163 - self.fVec47[1])));
			self.fVec48[(self.IOTA & 4095) as usize] = fTemp164;
			let mut fTemp165: f32 = f32::max(0.0, f32::min(2047.0, (fSlow95 * fTemp7)));
			let mut iTemp166: i32 = (fTemp165 as i32);
			let mut fTemp167: f32 = f32::floor(fTemp165);
			self.fRec61[0] = ((fTemp164 + (0.999000013 * self.fRec61[1])) - ((fSlow98 * self.fVec48[((self.IOTA - iSlow99) & 4095) as usize]) + (fSlow100 * self.fVec48[((self.IOTA - iSlow101) & 4095) as usize])));
			self.iRec63[0] = ((self.iVec0[1] + self.iRec63[1]) % iSlow102);
			let mut fTemp168: f32 = ((self.fRec62[1] * (1.0 - (((((self.iRec63[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow89 * fTemp66));
			self.fRec62[0] = (fTemp168 - f32::floor(fTemp168));
			let mut fTemp169: f32 = mydsp_faustpower2_f(((2.0 * self.fRec62[0]) + -1.0));
			self.fVec49[0] = fTemp169;
			self.fRec47[0] = (((self.fRec4[0] * (((fTemp2 * (fTemp126 - ((self.fVec36[((self.IOTA - iTemp128) & 4095) as usize] * (fTemp129 + (1.0 - fTemp127))) + ((fTemp127 - fTemp129) * self.fVec36[((self.IOTA - (iTemp128 + 1)) & 4095) as usize])))) + (fSlow80 * (self.fRec49[0] * fTemp12))) + (fSlow87 * ((fTemp13 * (fTemp131 - self.fVec37[1])) / fTemp15)))) + (((fTemp18 * ((self.fConst0 * (fTemp142 + fTemp153)) + (fSlow91 * (fTemp12 * (fTemp157 + (fTemp161 / fTemp20)))))) + ((fTemp56 * ((fSlow91 * (fTemp157 * fTemp12)) + (self.fConst0 * fTemp142))) * (fTemp58 + (fTemp57 * ((fSlow91 * ((fTemp161 * fTemp12) / fTemp20)) + (self.fConst0 * fTemp153)))))) + (fTemp59 * (((fTemp2 * (fTemp164 - ((self.fVec48[((self.IOTA - iTemp166) & 4095) as usize] * (fTemp167 + (1.0 - fTemp165))) + ((fTemp165 - fTemp167) * self.fVec48[((self.IOTA - (iTemp166 + 1)) & 4095) as usize])))) + (fSlow91 * (self.fRec61[0] * fTemp12))) + (fSlow87 * ((fTemp13 * (fTemp169 - self.fVec49[1])) / fTemp66)))))) - (((fTemp71 * self.fRec47[2]) + (2.0 * (fTemp72 * self.fRec47[1]))) / fTemp73));
			self.fRec46[0] = ((((self.fRec47[1] + (0.5 * self.fRec47[0])) + (0.5 * self.fRec47[2])) - ((fTemp71 * self.fRec46[2]) + (2.0 * (fTemp72 * self.fRec46[1])))) / fTemp73);
			self.iRec65[0] = (iSlow104 * (self.iRec65[1] + 1));
			let mut iTemp170: i32 = ((self.iRec65[0] < iSlow4) as i32);
			let mut fTemp171: f32 = f32::exp((0.0 - (self.fConst1 / if (iSlow104 as i32 != 0) { if (iTemp170 as i32 != 0) { fSlow6 } else { fSlow5 } } else { fSlow2 })));
			self.fRec64[0] = ((self.fRec64[1] * fTemp171) + (if (iSlow104 as i32 != 0) { if (iTemp170 as i32 != 0) { 1.58730161 } else { fSlow105 } } else { 0.0 } * (1.0 - fTemp171)));
			self.fVec50[0] = fSlow108;
			let mut fTemp172: f32 = (self.fRec68[1] + (self.fConst3 * self.fVec50[1]));
			self.fRec68[0] = (fTemp172 - f32::floor(fTemp172));
			let mut fTemp173: f32 = mydsp_faustpower2_f(((2.0 * self.fRec68[0]) + -1.0));
			self.fVec51[0] = fTemp173;
			let mut fTemp174: f32 = (fSlow109 * (fTemp3 * (fTemp173 - self.fVec51[1])));
			self.fVec52[(self.IOTA & 4095) as usize] = fTemp174;
			let mut fTemp175: f32 = f32::max(0.0, f32::min(2047.0, (fSlow110 * fTemp7)));
			let mut iTemp176: i32 = (fTemp175 as i32);
			let mut fTemp177: f32 = f32::floor(fTemp175);
			self.fRec69[0] = ((fTemp174 + (0.999000013 * self.fRec69[1])) - ((fSlow114 * self.fVec52[((self.IOTA - iSlow115) & 4095) as usize]) + (fSlow116 * self.fVec52[((self.IOTA - iSlow117) & 4095) as usize])));
			self.iRec71[0] = ((self.iVec0[1] + self.iRec71[1]) % iSlow119);
			let mut fTemp178: f32 = ((self.fRec70[1] * (1.0 - (((((self.iRec71[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow120 * fTemp15));
			self.fRec70[0] = (fTemp178 - f32::floor(fTemp178));
			let mut fTemp179: f32 = mydsp_faustpower2_f(((2.0 * self.fRec70[0]) + -1.0));
			self.fVec53[0] = fTemp179;
			self.iRec73[0] = ((self.iVec0[1] + self.iRec73[1]) % ((fSlow118 / fTemp20) as i32));
			let mut fTemp180: f32 = ((self.fRec72[1] * (1.0 - (((((self.iRec73[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow120 * fTemp21));
			self.fRec72[0] = (fTemp180 - f32::floor(fTemp180));
			let mut fTemp181: f32 = mydsp_faustpower2_f(((2.0 * self.fRec72[0]) + -1.0));
			self.fVec54[0] = fTemp181;
			let mut fTemp182: f32 = f32::max((fSlow106 * fTemp20), 23.4489498);
			let mut fTemp183: f32 = f32::max(20.0, f32::abs(fTemp182));
			self.fVec55[0] = fTemp183;
			let mut fTemp184: f32 = (self.fRec74[1] + (self.fConst3 * self.fVec55[1]));
			self.fRec74[0] = (fTemp184 - f32::floor(fTemp184));
			let mut fTemp185: f32 = mydsp_faustpower2_f(((2.0 * self.fRec74[0]) + -1.0));
			self.fVec56[0] = fTemp185;
			let mut fTemp186: f32 = ((fTemp3 * (fTemp185 - self.fVec56[1])) / fTemp183);
			self.fVec57[(self.IOTA & 4095) as usize] = fTemp186;
			let mut fTemp187: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp182))));
			let mut iTemp188: i32 = (fTemp187 as i32);
			let mut fTemp189: f32 = f32::floor(fTemp187);
			let mut fTemp190: f32 = ((fSlow121 * ((fTemp13 * (fTemp181 - self.fVec54[1])) / fTemp21)) + (0.25 * (fTemp2 * ((fTemp186 - (self.fVec57[((self.IOTA - iTemp188) & 4095) as usize] * (fTemp189 + (1.0 - fTemp187)))) - ((fTemp187 - fTemp189) * self.fVec57[((self.IOTA - (iTemp188 + 1)) & 4095) as usize])))));
			self.iRec76[0] = ((self.iVec0[1] + self.iRec76[1]) % ((fSlow118 * fTemp34) as i32));
			let mut fTemp191: f32 = ((self.fRec75[1] * (1.0 - (((((self.iRec76[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow120 * fTemp36));
			self.fRec75[0] = (fTemp191 - f32::floor(fTemp191));
			let mut fTemp192: f32 = mydsp_faustpower2_f(((2.0 * self.fRec75[0]) + -1.0));
			self.fVec58[0] = fTemp192;
			let mut fTemp193: f32 = f32::max((fSlow106 * fTemp35), 23.4489498);
			let mut fTemp194: f32 = f32::max(20.0, f32::abs(fTemp193));
			self.fVec59[0] = fTemp194;
			let mut fTemp195: f32 = (self.fRec77[1] + (self.fConst3 * self.fVec59[1]));
			self.fRec77[0] = (fTemp195 - f32::floor(fTemp195));
			let mut fTemp196: f32 = mydsp_faustpower2_f(((2.0 * self.fRec77[0]) + -1.0));
			self.fVec60[0] = fTemp196;
			let mut fTemp197: f32 = ((fTemp3 * (fTemp196 - self.fVec60[1])) / fTemp194);
			self.fVec61[(self.IOTA & 4095) as usize] = fTemp197;
			let mut fTemp198: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp193))));
			let mut iTemp199: i32 = (fTemp198 as i32);
			let mut fTemp200: f32 = f32::floor(fTemp198);
			let mut fTemp201: f32 = ((fSlow121 * ((fTemp13 * (fTemp192 - self.fVec58[1])) / fTemp36)) + (0.25 * (fTemp2 * ((fTemp197 - (self.fVec61[((self.IOTA - iTemp199) & 4095) as usize] * (fTemp200 + (1.0 - fTemp198)))) - ((fTemp198 - fTemp200) * self.fVec61[((self.IOTA - (iTemp199 + 1)) & 4095) as usize])))));
			let mut fTemp202: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp182)));
			let mut iTemp203: i32 = (fTemp202 as i32);
			let mut fTemp204: f32 = f32::floor(fTemp202);
			self.fRec78[0] = ((0.999000013 * self.fRec78[1]) + (self.fConst2 * ((fTemp186 - (self.fVec57[((self.IOTA - iTemp203) & 4095) as usize] * (fTemp204 + (1.0 - fTemp202)))) - ((fTemp202 - fTemp204) * self.fVec57[((self.IOTA - (iTemp203 + 1)) & 4095) as usize]))));
			let mut fTemp205: f32 = (self.fRec78[0] * fTemp20);
			let mut fTemp206: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp193)));
			let mut iTemp207: i32 = (fTemp206 as i32);
			let mut fTemp208: f32 = f32::floor(fTemp206);
			self.fRec79[0] = ((0.999000013 * self.fRec79[1]) + (self.fConst2 * ((fTemp197 - (self.fVec61[((self.IOTA - iTemp207) & 4095) as usize] * (fTemp208 + (1.0 - fTemp206)))) - ((fTemp206 - fTemp208) * self.fVec61[((self.IOTA - (iTemp207 + 1)) & 4095) as usize]))));
			let mut fTemp209: f32 = (self.fRec79[0] * fTemp33);
			self.fVec62[0] = fSlow124;
			let mut fTemp210: f32 = (self.fRec80[1] + (self.fConst3 * self.fVec62[1]));
			self.fRec80[0] = (fTemp210 - f32::floor(fTemp210));
			let mut fTemp211: f32 = mydsp_faustpower2_f(((2.0 * self.fRec80[0]) + -1.0));
			self.fVec63[0] = fTemp211;
			let mut fTemp212: f32 = (fSlow125 * (fTemp3 * (fTemp211 - self.fVec63[1])));
			self.fVec64[(self.IOTA & 4095) as usize] = fTemp212;
			let mut fTemp213: f32 = f32::max(0.0, f32::min(2047.0, (fSlow126 * fTemp7)));
			let mut iTemp214: i32 = (fTemp213 as i32);
			let mut fTemp215: f32 = f32::floor(fTemp213);
			self.fRec81[0] = ((fTemp212 + (0.999000013 * self.fRec81[1])) - ((fSlow129 * self.fVec64[((self.IOTA - iSlow130) & 4095) as usize]) + (fSlow131 * self.fVec64[((self.IOTA - iSlow132) & 4095) as usize])));
			self.iRec83[0] = ((self.iVec0[1] + self.iRec83[1]) % iSlow133);
			let mut fTemp216: f32 = ((self.fRec82[1] * (1.0 - (((((self.iRec83[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow120 * fTemp66));
			self.fRec82[0] = (fTemp216 - f32::floor(fTemp216));
			let mut fTemp217: f32 = mydsp_faustpower2_f(((2.0 * self.fRec82[0]) + -1.0));
			self.fVec65[0] = fTemp217;
			self.fRec67[0] = (((self.fRec4[0] * (((fTemp2 * (fTemp174 - ((self.fVec52[((self.IOTA - iTemp176) & 4095) as usize] * (fTemp177 + (1.0 - fTemp175))) + ((fTemp175 - fTemp177) * self.fVec52[((self.IOTA - (iTemp176 + 1)) & 4095) as usize])))) + (fSlow111 * (self.fRec69[0] * fTemp12))) + (fSlow118 * ((fTemp13 * (fTemp179 - self.fVec53[1])) / fTemp15)))) + (((fTemp18 * ((self.fConst0 * (fTemp190 + fTemp201)) + (fSlow122 * (fTemp12 * (fTemp205 + (fTemp209 / fTemp20)))))) + ((fTemp56 * ((fSlow122 * (fTemp205 * fTemp12)) + (self.fConst0 * fTemp190))) * (fTemp58 + (fTemp57 * ((fSlow122 * ((fTemp209 * fTemp12) / fTemp20)) + (self.fConst0 * fTemp201)))))) + (fTemp59 * (((fTemp2 * (fTemp212 - ((self.fVec64[((self.IOTA - iTemp214) & 4095) as usize] * (fTemp215 + (1.0 - fTemp213))) + ((fTemp213 - fTemp215) * self.fVec64[((self.IOTA - (iTemp214 + 1)) & 4095) as usize])))) + (fSlow122 * (self.fRec81[0] * fTemp12))) + (fSlow118 * ((fTemp13 * (fTemp217 - self.fVec65[1])) / fTemp66)))))) - (((fTemp71 * self.fRec67[2]) + (2.0 * (fTemp72 * self.fRec67[1]))) / fTemp73));
			self.fRec66[0] = ((((self.fRec67[1] + (0.5 * self.fRec67[0])) + (0.5 * self.fRec67[2])) - ((fTemp71 * self.fRec66[2]) + (2.0 * (fTemp72 * self.fRec66[1])))) / fTemp73);
			self.iRec85[0] = (iSlow135 * (self.iRec85[1] + 1));
			let mut iTemp218: i32 = ((self.iRec85[0] < iSlow4) as i32);
			let mut fTemp219: f32 = f32::exp((0.0 - (self.fConst1 / if (iSlow135 as i32 != 0) { if (iTemp218 as i32 != 0) { fSlow6 } else { fSlow5 } } else { fSlow2 })));
			self.fRec84[0] = ((self.fRec84[1] * fTemp219) + (if (iSlow135 as i32 != 0) { if (iTemp218 as i32 != 0) { 1.58730161 } else { fSlow136 } } else { 0.0 } * (1.0 - fTemp219)));
			self.fVec66[0] = fSlow139;
			let mut fTemp220: f32 = (self.fRec88[1] + (self.fConst3 * self.fVec66[1]));
			self.fRec88[0] = (fTemp220 - f32::floor(fTemp220));
			let mut fTemp221: f32 = mydsp_faustpower2_f(((2.0 * self.fRec88[0]) + -1.0));
			self.fVec67[0] = fTemp221;
			let mut fTemp222: f32 = (fSlow140 * (fTemp3 * (fTemp221 - self.fVec67[1])));
			self.fVec68[(self.IOTA & 4095) as usize] = fTemp222;
			let mut fTemp223: f32 = f32::max(0.0, f32::min(2047.0, (fSlow141 * fTemp7)));
			let mut iTemp224: i32 = (fTemp223 as i32);
			let mut fTemp225: f32 = f32::floor(fTemp223);
			self.fRec89[0] = ((fTemp222 + (0.999000013 * self.fRec89[1])) - ((fSlow145 * self.fVec68[((self.IOTA - iSlow146) & 4095) as usize]) + (fSlow147 * self.fVec68[((self.IOTA - iSlow148) & 4095) as usize])));
			self.iRec91[0] = ((self.iVec0[1] + self.iRec91[1]) % iSlow150);
			let mut fTemp226: f32 = ((self.fRec90[1] * (1.0 - (((((self.iRec91[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow151 * fTemp15));
			self.fRec90[0] = (fTemp226 - f32::floor(fTemp226));
			let mut fTemp227: f32 = mydsp_faustpower2_f(((2.0 * self.fRec90[0]) + -1.0));
			self.fVec69[0] = fTemp227;
			self.iRec93[0] = ((self.iVec0[1] + self.iRec93[1]) % ((fSlow149 / fTemp20) as i32));
			let mut fTemp228: f32 = ((self.fRec92[1] * (1.0 - (((((self.iRec93[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow151 * fTemp21));
			self.fRec92[0] = (fTemp228 - f32::floor(fTemp228));
			let mut fTemp229: f32 = mydsp_faustpower2_f(((2.0 * self.fRec92[0]) + -1.0));
			self.fVec70[0] = fTemp229;
			let mut fTemp230: f32 = f32::max((fSlow137 * fTemp20), 23.4489498);
			let mut fTemp231: f32 = f32::max(20.0, f32::abs(fTemp230));
			self.fVec71[0] = fTemp231;
			let mut fTemp232: f32 = (self.fRec94[1] + (self.fConst3 * self.fVec71[1]));
			self.fRec94[0] = (fTemp232 - f32::floor(fTemp232));
			let mut fTemp233: f32 = mydsp_faustpower2_f(((2.0 * self.fRec94[0]) + -1.0));
			self.fVec72[0] = fTemp233;
			let mut fTemp234: f32 = ((fTemp3 * (fTemp233 - self.fVec72[1])) / fTemp231);
			self.fVec73[(self.IOTA & 4095) as usize] = fTemp234;
			let mut fTemp235: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp230))));
			let mut iTemp236: i32 = (fTemp235 as i32);
			let mut fTemp237: f32 = f32::floor(fTemp235);
			let mut fTemp238: f32 = ((fSlow152 * ((fTemp13 * (fTemp229 - self.fVec70[1])) / fTemp21)) + (0.25 * (fTemp2 * ((fTemp234 - (self.fVec73[((self.IOTA - iTemp236) & 4095) as usize] * (fTemp237 + (1.0 - fTemp235)))) - ((fTemp235 - fTemp237) * self.fVec73[((self.IOTA - (iTemp236 + 1)) & 4095) as usize])))));
			self.iRec96[0] = ((self.iVec0[1] + self.iRec96[1]) % ((fSlow149 * fTemp34) as i32));
			let mut fTemp239: f32 = ((self.fRec95[1] * (1.0 - (((((self.iRec96[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow151 * fTemp36));
			self.fRec95[0] = (fTemp239 - f32::floor(fTemp239));
			let mut fTemp240: f32 = mydsp_faustpower2_f(((2.0 * self.fRec95[0]) + -1.0));
			self.fVec74[0] = fTemp240;
			let mut fTemp241: f32 = f32::max((fSlow137 * fTemp35), 23.4489498);
			let mut fTemp242: f32 = f32::max(20.0, f32::abs(fTemp241));
			self.fVec75[0] = fTemp242;
			let mut fTemp243: f32 = (self.fRec97[1] + (self.fConst3 * self.fVec75[1]));
			self.fRec97[0] = (fTemp243 - f32::floor(fTemp243));
			let mut fTemp244: f32 = mydsp_faustpower2_f(((2.0 * self.fRec97[0]) + -1.0));
			self.fVec76[0] = fTemp244;
			let mut fTemp245: f32 = ((fTemp3 * (fTemp244 - self.fVec76[1])) / fTemp242);
			self.fVec77[(self.IOTA & 4095) as usize] = fTemp245;
			let mut fTemp246: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp241))));
			let mut iTemp247: i32 = (fTemp246 as i32);
			let mut fTemp248: f32 = f32::floor(fTemp246);
			let mut fTemp249: f32 = ((fSlow152 * ((fTemp13 * (fTemp240 - self.fVec74[1])) / fTemp36)) + (0.25 * (fTemp2 * ((fTemp245 - (self.fVec77[((self.IOTA - iTemp247) & 4095) as usize] * (fTemp248 + (1.0 - fTemp246)))) - ((fTemp246 - fTemp248) * self.fVec77[((self.IOTA - (iTemp247 + 1)) & 4095) as usize])))));
			let mut fTemp250: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp230)));
			let mut iTemp251: i32 = (fTemp250 as i32);
			let mut fTemp252: f32 = f32::floor(fTemp250);
			self.fRec98[0] = ((0.999000013 * self.fRec98[1]) + (self.fConst2 * ((fTemp234 - (self.fVec73[((self.IOTA - iTemp251) & 4095) as usize] * (fTemp252 + (1.0 - fTemp250)))) - ((fTemp250 - fTemp252) * self.fVec73[((self.IOTA - (iTemp251 + 1)) & 4095) as usize]))));
			let mut fTemp253: f32 = (self.fRec98[0] * fTemp20);
			let mut fTemp254: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp241)));
			let mut iTemp255: i32 = (fTemp254 as i32);
			let mut fTemp256: f32 = f32::floor(fTemp254);
			self.fRec99[0] = ((0.999000013 * self.fRec99[1]) + (self.fConst2 * ((fTemp245 - (self.fVec77[((self.IOTA - iTemp255) & 4095) as usize] * (fTemp256 + (1.0 - fTemp254)))) - ((fTemp254 - fTemp256) * self.fVec77[((self.IOTA - (iTemp255 + 1)) & 4095) as usize]))));
			let mut fTemp257: f32 = (self.fRec99[0] * fTemp33);
			self.fVec78[0] = fSlow155;
			let mut fTemp258: f32 = (self.fRec100[1] + (self.fConst3 * self.fVec78[1]));
			self.fRec100[0] = (fTemp258 - f32::floor(fTemp258));
			let mut fTemp259: f32 = mydsp_faustpower2_f(((2.0 * self.fRec100[0]) + -1.0));
			self.fVec79[0] = fTemp259;
			let mut fTemp260: f32 = (fSlow156 * (fTemp3 * (fTemp259 - self.fVec79[1])));
			self.fVec80[(self.IOTA & 4095) as usize] = fTemp260;
			let mut fTemp261: f32 = f32::max(0.0, f32::min(2047.0, (fSlow157 * fTemp7)));
			let mut iTemp262: i32 = (fTemp261 as i32);
			let mut fTemp263: f32 = f32::floor(fTemp261);
			self.fRec101[0] = ((fTemp260 + (0.999000013 * self.fRec101[1])) - ((fSlow160 * self.fVec80[((self.IOTA - iSlow161) & 4095) as usize]) + (fSlow162 * self.fVec80[((self.IOTA - iSlow163) & 4095) as usize])));
			self.iRec103[0] = ((self.iVec0[1] + self.iRec103[1]) % iSlow164);
			let mut fTemp264: f32 = ((self.fRec102[1] * (1.0 - (((((self.iRec103[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow151 * fTemp66));
			self.fRec102[0] = (fTemp264 - f32::floor(fTemp264));
			let mut fTemp265: f32 = mydsp_faustpower2_f(((2.0 * self.fRec102[0]) + -1.0));
			self.fVec81[0] = fTemp265;
			self.fRec87[0] = (((self.fRec4[0] * (((fTemp2 * (fTemp222 - ((self.fVec68[((self.IOTA - iTemp224) & 4095) as usize] * (fTemp225 + (1.0 - fTemp223))) + ((fTemp223 - fTemp225) * self.fVec68[((self.IOTA - (iTemp224 + 1)) & 4095) as usize])))) + (fSlow142 * (self.fRec89[0] * fTemp12))) + (fSlow149 * ((fTemp13 * (fTemp227 - self.fVec69[1])) / fTemp15)))) + (((fTemp18 * ((self.fConst0 * (fTemp238 + fTemp249)) + (fSlow153 * (fTemp12 * (fTemp253 + (fTemp257 / fTemp20)))))) + ((fTemp56 * ((fSlow153 * (fTemp253 * fTemp12)) + (self.fConst0 * fTemp238))) * (fTemp58 + (fTemp57 * ((fSlow153 * ((fTemp257 * fTemp12) / fTemp20)) + (self.fConst0 * fTemp249)))))) + (fTemp59 * (((fTemp2 * (fTemp260 - ((self.fVec80[((self.IOTA - iTemp262) & 4095) as usize] * (fTemp263 + (1.0 - fTemp261))) + ((fTemp261 - fTemp263) * self.fVec80[((self.IOTA - (iTemp262 + 1)) & 4095) as usize])))) + (fSlow153 * (self.fRec101[0] * fTemp12))) + (fSlow149 * ((fTemp13 * (fTemp265 - self.fVec81[1])) / fTemp66)))))) - (((fTemp71 * self.fRec87[2]) + (2.0 * (fTemp72 * self.fRec87[1]))) / fTemp73));
			self.fRec86[0] = ((((self.fRec87[1] + (0.5 * self.fRec87[0])) + (0.5 * self.fRec87[2])) - ((fTemp71 * self.fRec86[2]) + (2.0 * (fTemp72 * self.fRec86[1])))) / fTemp73);
			self.iRec105[0] = (iSlow166 * (self.iRec105[1] + 1));
			let mut iTemp266: i32 = ((self.iRec105[0] < iSlow4) as i32);
			let mut fTemp267: f32 = f32::exp((0.0 - (self.fConst1 / if (iSlow166 as i32 != 0) { if (iTemp266 as i32 != 0) { fSlow6 } else { fSlow5 } } else { fSlow2 })));
			self.fRec104[0] = ((self.fRec104[1] * fTemp267) + (if (iSlow166 as i32 != 0) { if (iTemp266 as i32 != 0) { 1.58730161 } else { fSlow167 } } else { 0.0 } * (1.0 - fTemp267)));
			self.fVec82[0] = fSlow170;
			let mut fTemp268: f32 = (self.fRec108[1] + (self.fConst3 * self.fVec82[1]));
			self.fRec108[0] = (fTemp268 - f32::floor(fTemp268));
			let mut fTemp269: f32 = mydsp_faustpower2_f(((2.0 * self.fRec108[0]) + -1.0));
			self.fVec83[0] = fTemp269;
			let mut fTemp270: f32 = (fSlow171 * (fTemp3 * (fTemp269 - self.fVec83[1])));
			self.fVec84[(self.IOTA & 4095) as usize] = fTemp270;
			let mut fTemp271: f32 = f32::max(0.0, f32::min(2047.0, (fSlow172 * fTemp7)));
			let mut iTemp272: i32 = (fTemp271 as i32);
			let mut fTemp273: f32 = f32::floor(fTemp271);
			self.fRec109[0] = ((fTemp270 + (0.999000013 * self.fRec109[1])) - ((fSlow176 * self.fVec84[((self.IOTA - iSlow177) & 4095) as usize]) + (fSlow178 * self.fVec84[((self.IOTA - iSlow179) & 4095) as usize])));
			self.iRec111[0] = ((self.iVec0[1] + self.iRec111[1]) % iSlow181);
			let mut fTemp274: f32 = ((self.fRec110[1] * (1.0 - (((((self.iRec111[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow182 * fTemp15));
			self.fRec110[0] = (fTemp274 - f32::floor(fTemp274));
			let mut fTemp275: f32 = mydsp_faustpower2_f(((2.0 * self.fRec110[0]) + -1.0));
			self.fVec85[0] = fTemp275;
			self.iRec113[0] = ((self.iVec0[1] + self.iRec113[1]) % ((fSlow180 / fTemp20) as i32));
			let mut fTemp276: f32 = ((self.fRec112[1] * (1.0 - (((((self.iRec113[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow182 * fTemp21));
			self.fRec112[0] = (fTemp276 - f32::floor(fTemp276));
			let mut fTemp277: f32 = mydsp_faustpower2_f(((2.0 * self.fRec112[0]) + -1.0));
			self.fVec86[0] = fTemp277;
			let mut fTemp278: f32 = f32::max((fSlow168 * fTemp20), 23.4489498);
			let mut fTemp279: f32 = f32::max(20.0, f32::abs(fTemp278));
			self.fVec87[0] = fTemp279;
			let mut fTemp280: f32 = (self.fRec114[1] + (self.fConst3 * self.fVec87[1]));
			self.fRec114[0] = (fTemp280 - f32::floor(fTemp280));
			let mut fTemp281: f32 = mydsp_faustpower2_f(((2.0 * self.fRec114[0]) + -1.0));
			self.fVec88[0] = fTemp281;
			let mut fTemp282: f32 = ((fTemp3 * (fTemp281 - self.fVec88[1])) / fTemp279);
			self.fVec89[(self.IOTA & 4095) as usize] = fTemp282;
			let mut fTemp283: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp278))));
			let mut iTemp284: i32 = (fTemp283 as i32);
			let mut fTemp285: f32 = f32::floor(fTemp283);
			let mut fTemp286: f32 = ((fSlow183 * ((fTemp13 * (fTemp277 - self.fVec86[1])) / fTemp21)) + (0.25 * (fTemp2 * ((fTemp282 - (self.fVec89[((self.IOTA - iTemp284) & 4095) as usize] * (fTemp285 + (1.0 - fTemp283)))) - ((fTemp283 - fTemp285) * self.fVec89[((self.IOTA - (iTemp284 + 1)) & 4095) as usize])))));
			self.iRec116[0] = ((self.iVec0[1] + self.iRec116[1]) % ((fSlow180 * fTemp34) as i32));
			let mut fTemp287: f32 = ((self.fRec115[1] * (1.0 - (((((self.iRec116[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow182 * fTemp36));
			self.fRec115[0] = (fTemp287 - f32::floor(fTemp287));
			let mut fTemp288: f32 = mydsp_faustpower2_f(((2.0 * self.fRec115[0]) + -1.0));
			self.fVec90[0] = fTemp288;
			let mut fTemp289: f32 = f32::max((fSlow168 * fTemp35), 23.4489498);
			let mut fTemp290: f32 = f32::max(20.0, f32::abs(fTemp289));
			self.fVec91[0] = fTemp290;
			let mut fTemp291: f32 = (self.fRec117[1] + (self.fConst3 * self.fVec91[1]));
			self.fRec117[0] = (fTemp291 - f32::floor(fTemp291));
			let mut fTemp292: f32 = mydsp_faustpower2_f(((2.0 * self.fRec117[0]) + -1.0));
			self.fVec92[0] = fTemp292;
			let mut fTemp293: f32 = ((fTemp3 * (fTemp292 - self.fVec92[1])) / fTemp290);
			self.fVec93[(self.IOTA & 4095) as usize] = fTemp293;
			let mut fTemp294: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst0 * (fTemp7 / fTemp289))));
			let mut iTemp295: i32 = (fTemp294 as i32);
			let mut fTemp296: f32 = f32::floor(fTemp294);
			let mut fTemp297: f32 = ((fSlow183 * ((fTemp13 * (fTemp288 - self.fVec90[1])) / fTemp36)) + (0.25 * (fTemp2 * ((fTemp293 - (self.fVec93[((self.IOTA - iTemp295) & 4095) as usize] * (fTemp296 + (1.0 - fTemp294)))) - ((fTemp294 - fTemp296) * self.fVec93[((self.IOTA - (iTemp295 + 1)) & 4095) as usize])))));
			let mut fTemp298: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp278)));
			let mut iTemp299: i32 = (fTemp298 as i32);
			let mut fTemp300: f32 = f32::floor(fTemp298);
			self.fRec118[0] = ((0.999000013 * self.fRec118[1]) + (self.fConst2 * ((fTemp282 - (self.fVec89[((self.IOTA - iTemp299) & 4095) as usize] * (fTemp300 + (1.0 - fTemp298)))) - ((fTemp298 - fTemp300) * self.fVec89[((self.IOTA - (iTemp299 + 1)) & 4095) as usize]))));
			let mut fTemp301: f32 = (self.fRec118[0] * fTemp20);
			let mut fTemp302: f32 = f32::max(0.0, f32::min(2047.0, (self.fConst5 / fTemp289)));
			let mut iTemp303: i32 = (fTemp302 as i32);
			let mut fTemp304: f32 = f32::floor(fTemp302);
			self.fRec119[0] = ((0.999000013 * self.fRec119[1]) + (self.fConst2 * ((fTemp293 - (self.fVec93[((self.IOTA - iTemp303) & 4095) as usize] * (fTemp304 + (1.0 - fTemp302)))) - ((fTemp302 - fTemp304) * self.fVec93[((self.IOTA - (iTemp303 + 1)) & 4095) as usize]))));
			let mut fTemp305: f32 = (self.fRec119[0] * fTemp33);
			self.fVec94[0] = fSlow186;
			let mut fTemp306: f32 = (self.fRec120[1] + (self.fConst3 * self.fVec94[1]));
			self.fRec120[0] = (fTemp306 - f32::floor(fTemp306));
			let mut fTemp307: f32 = mydsp_faustpower2_f(((2.0 * self.fRec120[0]) + -1.0));
			self.fVec95[0] = fTemp307;
			let mut fTemp308: f32 = (fSlow187 * (fTemp3 * (fTemp307 - self.fVec95[1])));
			self.fVec96[(self.IOTA & 4095) as usize] = fTemp308;
			let mut fTemp309: f32 = f32::max(0.0, f32::min(2047.0, (fSlow188 * fTemp7)));
			let mut iTemp310: i32 = (fTemp309 as i32);
			let mut fTemp311: f32 = f32::floor(fTemp309);
			self.fRec121[0] = ((fTemp308 + (0.999000013 * self.fRec121[1])) - ((fSlow191 * self.fVec96[((self.IOTA - iSlow192) & 4095) as usize]) + (fSlow193 * self.fVec96[((self.IOTA - iSlow194) & 4095) as usize])));
			self.iRec123[0] = ((self.iVec0[1] + self.iRec123[1]) % iSlow195);
			let mut fTemp312: f32 = ((self.fRec122[1] * (1.0 - (((((self.iRec123[0] == 0) as i32) > 0) as i32) as f32))) + (fSlow182 * fTemp66));
			self.fRec122[0] = (fTemp312 - f32::floor(fTemp312));
			let mut fTemp313: f32 = mydsp_faustpower2_f(((2.0 * self.fRec122[0]) + -1.0));
			self.fVec97[0] = fTemp313;
			self.fRec107[0] = (((self.fRec4[0] * (((fTemp2 * (fTemp270 - ((self.fVec84[((self.IOTA - iTemp272) & 4095) as usize] * (fTemp273 + (1.0 - fTemp271))) + ((fTemp271 - fTemp273) * self.fVec84[((self.IOTA - (iTemp272 + 1)) & 4095) as usize])))) + (fSlow173 * (self.fRec109[0] * fTemp12))) + (fSlow180 * ((fTemp13 * (fTemp275 - self.fVec85[1])) / fTemp15)))) + (((fTemp18 * ((self.fConst0 * (fTemp286 + fTemp297)) + (fSlow184 * (fTemp12 * (fTemp301 + (fTemp305 / fTemp20)))))) + ((fTemp56 * ((fSlow184 * (fTemp301 * fTemp12)) + (self.fConst0 * fTemp286))) * (fTemp58 + (fTemp57 * ((fSlow184 * ((fTemp305 * fTemp12) / fTemp20)) + (self.fConst0 * fTemp297)))))) + (fTemp59 * (((fTemp2 * (fTemp308 - ((self.fVec96[((self.IOTA - iTemp310) & 4095) as usize] * (fTemp311 + (1.0 - fTemp309))) + ((fTemp309 - fTemp311) * self.fVec96[((self.IOTA - (iTemp310 + 1)) & 4095) as usize])))) + (fSlow184 * (self.fRec121[0] * fTemp12))) + (fSlow180 * ((fTemp13 * (fTemp313 - self.fVec97[1])) / fTemp66)))))) - (((fTemp71 * self.fRec107[2]) + (2.0 * (fTemp72 * self.fRec107[1]))) / fTemp73));
			self.fRec106[0] = ((((self.fRec107[1] + (0.5 * self.fRec107[0])) + (0.5 * self.fRec107[2])) - ((fTemp71 * self.fRec106[2]) + (2.0 * (fTemp72 * self.fRec106[1])))) / fTemp73);
			*output0 = ((((((((fSlow0 * (f32::min(1.0, self.fRec0[0]) * ((self.fRec2[1] + (0.5 * self.fRec2[0])) + (0.5 * self.fRec2[2])))) + (fSlow41 * (f32::min(1.0, self.fRec24[0]) * ((self.fRec26[1] + (0.5 * self.fRec26[0])) + (0.5 * self.fRec26[2]))))) + (fSlow72 * (f32::min(1.0, self.fRec44[0]) * ((self.fRec46[1] + (0.5 * self.fRec46[0])) + (0.5 * self.fRec46[2]))))) + (fSlow103 * (f32::min(1.0, self.fRec64[0]) * ((self.fRec66[1] + (0.5 * self.fRec66[0])) + (0.5 * self.fRec66[2]))))) + (fSlow134 * (f32::min(1.0, self.fRec84[0]) * ((self.fRec86[1] + (0.5 * self.fRec86[0])) + (0.5 * self.fRec86[2]))))) + (fSlow165 * (f32::min(1.0, self.fRec104[0]) * ((self.fRec106[1] + (0.5 * self.fRec106[0])) + (0.5 * self.fRec106[2]))))) / fTemp73) as f32);
			self.iVec0[1] = self.iVec0[0];
			self.iRec1[1] = self.iRec1[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.fVec1[1] = self.fVec1[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec6[1] = self.fRec6[0];
			self.fVec3[1] = self.fVec3[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec7[1] = self.fRec7[0];
			self.iRec9[1] = self.iRec9[0];
			self.fRec8[1] = self.fRec8[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec10[1] = self.fRec10[0];
			self.iRec12[1] = self.iRec12[0];
			self.fRec11[1] = self.fRec11[0];
			self.fVec6[1] = self.fVec6[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec13[1] = self.fRec13[0];
			self.fVec8[1] = self.fVec8[0];
			self.iRec15[1] = self.iRec15[0];
			self.fRec14[1] = self.fRec14[0];
			self.fVec10[1] = self.fVec10[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec16[1] = self.fRec16[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec18[1] = self.fRec18[0];
			self.fVec14[1] = self.fVec14[0];
			self.fRec19[1] = self.fRec19[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec20[1] = self.fRec20[0];
			self.iRec22[1] = self.iRec22[0];
			self.fRec21[1] = self.fRec21[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fRec2[2] = self.fRec2[1];
			self.fRec2[1] = self.fRec2[0];
			self.iRec25[1] = self.iRec25[0];
			self.fRec24[1] = self.fRec24[0];
			self.fVec18[1] = self.fVec18[0];
			self.fRec28[1] = self.fRec28[0];
			self.fVec19[1] = self.fVec19[0];
			self.fRec29[1] = self.fRec29[0];
			self.iRec31[1] = self.iRec31[0];
			self.fRec30[1] = self.fRec30[0];
			self.fVec21[1] = self.fVec21[0];
			self.iRec33[1] = self.iRec33[0];
			self.fRec32[1] = self.fRec32[0];
			self.fVec22[1] = self.fVec22[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec34[1] = self.fRec34[0];
			self.fVec24[1] = self.fVec24[0];
			self.iRec36[1] = self.iRec36[0];
			self.fRec35[1] = self.fRec35[0];
			self.fVec26[1] = self.fVec26[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec37[1] = self.fRec37[0];
			self.fVec28[1] = self.fVec28[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fVec30[1] = self.fVec30[0];
			self.fRec40[1] = self.fRec40[0];
			self.fVec31[1] = self.fVec31[0];
			self.fRec41[1] = self.fRec41[0];
			self.iRec43[1] = self.iRec43[0];
			self.fRec42[1] = self.fRec42[0];
			self.fVec33[1] = self.fVec33[0];
			self.fRec27[2] = self.fRec27[1];
			self.fRec27[1] = self.fRec27[0];
			self.fRec26[2] = self.fRec26[1];
			self.fRec26[1] = self.fRec26[0];
			self.iRec45[1] = self.iRec45[0];
			self.fRec44[1] = self.fRec44[0];
			self.fVec34[1] = self.fVec34[0];
			self.fRec48[1] = self.fRec48[0];
			self.fVec35[1] = self.fVec35[0];
			self.fRec49[1] = self.fRec49[0];
			self.iRec51[1] = self.iRec51[0];
			self.fRec50[1] = self.fRec50[0];
			self.fVec37[1] = self.fVec37[0];
			self.iRec53[1] = self.iRec53[0];
			self.fRec52[1] = self.fRec52[0];
			self.fVec38[1] = self.fVec38[0];
			self.fVec39[1] = self.fVec39[0];
			self.fRec54[1] = self.fRec54[0];
			self.fVec40[1] = self.fVec40[0];
			self.iRec56[1] = self.iRec56[0];
			self.fRec55[1] = self.fRec55[0];
			self.fVec42[1] = self.fVec42[0];
			self.fVec43[1] = self.fVec43[0];
			self.fRec57[1] = self.fRec57[0];
			self.fVec44[1] = self.fVec44[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec59[1] = self.fRec59[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec60[1] = self.fRec60[0];
			self.fVec47[1] = self.fVec47[0];
			self.fRec61[1] = self.fRec61[0];
			self.iRec63[1] = self.iRec63[0];
			self.fRec62[1] = self.fRec62[0];
			self.fVec49[1] = self.fVec49[0];
			self.fRec47[2] = self.fRec47[1];
			self.fRec47[1] = self.fRec47[0];
			self.fRec46[2] = self.fRec46[1];
			self.fRec46[1] = self.fRec46[0];
			self.iRec65[1] = self.iRec65[0];
			self.fRec64[1] = self.fRec64[0];
			self.fVec50[1] = self.fVec50[0];
			self.fRec68[1] = self.fRec68[0];
			self.fVec51[1] = self.fVec51[0];
			self.fRec69[1] = self.fRec69[0];
			self.iRec71[1] = self.iRec71[0];
			self.fRec70[1] = self.fRec70[0];
			self.fVec53[1] = self.fVec53[0];
			self.iRec73[1] = self.iRec73[0];
			self.fRec72[1] = self.fRec72[0];
			self.fVec54[1] = self.fVec54[0];
			self.fVec55[1] = self.fVec55[0];
			self.fRec74[1] = self.fRec74[0];
			self.fVec56[1] = self.fVec56[0];
			self.iRec76[1] = self.iRec76[0];
			self.fRec75[1] = self.fRec75[0];
			self.fVec58[1] = self.fVec58[0];
			self.fVec59[1] = self.fVec59[0];
			self.fRec77[1] = self.fRec77[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec79[1] = self.fRec79[0];
			self.fVec62[1] = self.fVec62[0];
			self.fRec80[1] = self.fRec80[0];
			self.fVec63[1] = self.fVec63[0];
			self.fRec81[1] = self.fRec81[0];
			self.iRec83[1] = self.iRec83[0];
			self.fRec82[1] = self.fRec82[0];
			self.fVec65[1] = self.fVec65[0];
			self.fRec67[2] = self.fRec67[1];
			self.fRec67[1] = self.fRec67[0];
			self.fRec66[2] = self.fRec66[1];
			self.fRec66[1] = self.fRec66[0];
			self.iRec85[1] = self.iRec85[0];
			self.fRec84[1] = self.fRec84[0];
			self.fVec66[1] = self.fVec66[0];
			self.fRec88[1] = self.fRec88[0];
			self.fVec67[1] = self.fVec67[0];
			self.fRec89[1] = self.fRec89[0];
			self.iRec91[1] = self.iRec91[0];
			self.fRec90[1] = self.fRec90[0];
			self.fVec69[1] = self.fVec69[0];
			self.iRec93[1] = self.iRec93[0];
			self.fRec92[1] = self.fRec92[0];
			self.fVec70[1] = self.fVec70[0];
			self.fVec71[1] = self.fVec71[0];
			self.fRec94[1] = self.fRec94[0];
			self.fVec72[1] = self.fVec72[0];
			self.iRec96[1] = self.iRec96[0];
			self.fRec95[1] = self.fRec95[0];
			self.fVec74[1] = self.fVec74[0];
			self.fVec75[1] = self.fVec75[0];
			self.fRec97[1] = self.fRec97[0];
			self.fVec76[1] = self.fVec76[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec99[1] = self.fRec99[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec100[1] = self.fRec100[0];
			self.fVec79[1] = self.fVec79[0];
			self.fRec101[1] = self.fRec101[0];
			self.iRec103[1] = self.iRec103[0];
			self.fRec102[1] = self.fRec102[0];
			self.fVec81[1] = self.fVec81[0];
			self.fRec87[2] = self.fRec87[1];
			self.fRec87[1] = self.fRec87[0];
			self.fRec86[2] = self.fRec86[1];
			self.fRec86[1] = self.fRec86[0];
			self.iRec105[1] = self.iRec105[0];
			self.fRec104[1] = self.fRec104[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec108[1] = self.fRec108[0];
			self.fVec83[1] = self.fVec83[0];
			self.fRec109[1] = self.fRec109[0];
			self.iRec111[1] = self.iRec111[0];
			self.fRec110[1] = self.fRec110[0];
			self.fVec85[1] = self.fVec85[0];
			self.iRec113[1] = self.iRec113[0];
			self.fRec112[1] = self.fRec112[0];
			self.fVec86[1] = self.fVec86[0];
			self.fVec87[1] = self.fVec87[0];
			self.fRec114[1] = self.fRec114[0];
			self.fVec88[1] = self.fVec88[0];
			self.iRec116[1] = self.iRec116[0];
			self.fRec115[1] = self.fRec115[0];
			self.fVec90[1] = self.fVec90[0];
			self.fVec91[1] = self.fVec91[0];
			self.fRec117[1] = self.fRec117[0];
			self.fVec92[1] = self.fVec92[0];
			self.fRec118[1] = self.fRec118[0];
			self.fRec119[1] = self.fRec119[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec120[1] = self.fRec120[0];
			self.fVec95[1] = self.fVec95[0];
			self.fRec121[1] = self.fRec121[0];
			self.iRec123[1] = self.iRec123[0];
			self.fRec122[1] = self.fRec122[0];
			self.fVec97[1] = self.fVec97[0];
			self.fRec107[2] = self.fRec107[1];
			self.fRec107[1] = self.fRec107[0];
			self.fRec106[2] = self.fRec106[1];
			self.fRec106[1] = self.fRec106[0];
		}
	}

	#[inline]
	pub fn compute_external(&mut self, count: i32) {
		let (output0) = unsafe {
			(::std::slice::from_raw_parts_mut(OUT_BUFFER0.as_mut_ptr(), count as usize))
		};
		unsafe { self.compute(count, &[], &mut [output0]); }
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
    