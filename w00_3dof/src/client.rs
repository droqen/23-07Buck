use ambient_api::prelude::*;
use ambient_api::input;

#[main]
pub fn main() {

	ambient_api::messages::Frame::subscribe(move |_|{
		let pin = input::get();
		let mut pry = messages::DofPryInput{ p:0., r:0., y:0., throttle:0. };
		if pin.keys.contains(&KeyCode::S) { pry.p -= 1.; }
		if pin.keys.contains(&KeyCode::W) { pry.p += 1.; }
		if pin.keys.contains(&KeyCode::Q) { pry.r -= 1.; }
		if pin.keys.contains(&KeyCode::E) { pry.r += 1.; }
		if pin.keys.contains(&KeyCode::A) { pry.y -= 1.; }
		if pin.keys.contains(&KeyCode::D) { pry.y += 1.; }
		if pin.keys.contains(&KeyCode::Up) { pry.throttle += 1.; }
		if pin.keys.contains(&KeyCode::Down) { pry.throttle -= 1.; }

		pry.send_server_unreliable();
	});
}

fn _iabs(i:i32)->i32{
	if i>0{return i}-i
} 