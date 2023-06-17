use ambient_api::prelude::*;
use ambient_api::input;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

const NUMBER_KEY_ORDER : [&'static KeyCode; 9] = [
	&KeyCode::Key1, // 0
	&KeyCode::Key2,
	&KeyCode::Key3,
	&KeyCode::Key4,
	&KeyCode::Key5,
	&KeyCode::Key6,
	&KeyCode::Key7,
	&KeyCode::Key8,
	&KeyCode::Key9, // 8
];

#[main]
pub fn main() {

	let mut gear_value : i32 = 0;
	let mut prev_keys : HashSet<KeyCode, RandomState> = HashSet::new();

	ambient_api::messages::Frame::subscribe(move |_|{
		let pin = input::get();
		let mut pry = messages::DofPryInput{ p:0., r:0., y:0., throttle:0. };
		if pin.keys.contains(&KeyCode::S) { pry.p -= 1.; }
		if pin.keys.contains(&KeyCode::W) { pry.p += 1.; }
		if pin.keys.contains(&KeyCode::Q) { pry.r -= 1.; }
		if pin.keys.contains(&KeyCode::E) { pry.r += 1.; }
		if pin.keys.contains(&KeyCode::A) { pry.y -= 1.; }
		if pin.keys.contains(&KeyCode::D) { pry.y += 1.; }
		// if pin.keys.contains(&KeyCode::Up) { pry.throttle += 1.; }
		// if pin.keys.contains(&KeyCode::Down) { pry.throttle -= 1.; }

		for new_gear in 0..9 {
			if pin.keys.contains(NUMBER_KEY_ORDER[new_gear]) && ! prev_keys.contains(NUMBER_KEY_ORDER[new_gear]) {
				let delta:i32=(new_gear as i32)-gear_value;
				println!("Attempted throttle {gear_value:?} to {new_gear:?}, delta={delta:?}");
				if delta==0{println!("No change to throttle value");}
				else if iabs(delta) > 1 {println!("Throttle change FAILED");}
				else {
					gear_value=(new_gear as i32);println!("New throttle value {gear_value:?}");
					messages::PlayerChangeGear{ gear : gear_value }.send_server_reliable();
				}
			}
		}

		prev_keys = pin.keys;

		pry.send_server_unreliable();
	});
}

fn iabs(i:i32)->i32{
	if i>0{return i}-i
} 