use ambient_api::prelude::*;
use ambient_api::input;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

#[main]
pub fn main() {
	let mut prev_keys : HashSet<KeyCode, RandomState> = HashSet::new();
	ambient_api::messages::Frame::subscribe(move |_|{
		let pin = input::get();
		let mut command_message = messages::PlayerCommand::new(0, 0);
		for (keycodes,dmvx,dmvy) in vec![
			(vec![KeyCode::W, KeyCode::Up], 0, -1),
			(vec![KeyCode::S, KeyCode::Down], 0, 1),
			(vec![KeyCode::A, KeyCode::Left], -1, 0),
			(vec![KeyCode::D, KeyCode::Right], 1, 0)
		] {
			for keycode in keycodes { if tapped(&pin.keys, &prev_keys, keycode) { command_message.mvx+=dmvx;command_message.mvy+=dmvy; } }
		}

		prev_keys = pin.keys;

		if command_message.mvx != 0 || command_message.mvy != 0 {
			command_message.send_server_reliable();
			println!("(x:{}, y:{})", command_message.mvx, command_message.mvy);
		}
	});
}

fn tapped(current_keys : &HashSet<KeyCode, RandomState>, prev_keys : &HashSet<KeyCode, RandomState>, key_code : KeyCode) -> bool {
	current_keys.contains(&key_code) && !prev_keys.contains(&key_code)
}