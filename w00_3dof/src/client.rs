use ambient_api::prelude::*;
use ambient_api::input;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

const UD_TOGGLE_DOUBLE_TAP_BUFFER : u32 = 30;

#[main]
pub fn main() {

	let mut prev_keys : HashSet<KeyCode, RandomState> = HashSet::new();
	let mut ud_toggle_up_buf : u32 = 0;
	let mut ud_toggle_dn_buf : u32 = 0;
	let mut ud_toggle_value : i32 = 0;

	ambient_api::messages::Frame::subscribe(move |_|{
		let pin = input::get();
		let mut pry = messages::DofPryInput{ p:0., r:0., y:0., throttle:0. };
		if pin.keys.contains(&KeyCode::S) { pry.p -= 1.; }
		if pin.keys.contains(&KeyCode::W) { pry.p += 1.; }
		if pin.keys.contains(&KeyCode::Q) { pry.r -= 1.; }
		if pin.keys.contains(&KeyCode::E) { pry.r += 1.; }
		if pin.keys.contains(&KeyCode::A) { pry.y -= 1.; }
		if pin.keys.contains(&KeyCode::D) { pry.y += 1.; }
		if ud_toggle_value!=0 { pry.throttle = ud_toggle_value as f32; }
		else {
			if pin.keys.contains(&KeyCode::Up) { pry.throttle += 1.; }
			if pin.keys.contains(&KeyCode::Down) { pry.throttle -= 1.; }
		}

		if ud_toggle_up_buf>0{ud_toggle_up_buf-=1;}
		if ud_toggle_dn_buf>0{ud_toggle_dn_buf-=1;}

		if tapped(&pin.keys, &prev_keys, KeyCode::Up){
			if ud_toggle_up_buf > 0 { ud_toggle_value = 1; } else { ud_toggle_value = 0; }
			ud_toggle_up_buf = UD_TOGGLE_DOUBLE_TAP_BUFFER;
		}
		if tapped(&pin.keys, &prev_keys, KeyCode::Down){
			if ud_toggle_dn_buf > 0 { ud_toggle_value = -1; } else { ud_toggle_value = 0; }
			ud_toggle_dn_buf = UD_TOGGLE_DOUBLE_TAP_BUFFER;
		}

		prev_keys = pin.keys;

		pry.send_server_unreliable();
	});
}

fn _iabs(i:i32)->i32{
	if i>0{return i}-i
}

fn tapped(current_keys : &HashSet<KeyCode, RandomState>, prev_keys : &HashSet<KeyCode, RandomState>, key_code : KeyCode) -> bool {
	current_keys.contains(&key_code) && !prev_keys.contains(&key_code)
}