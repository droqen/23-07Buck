use ambient_api::{
	// components::core::{
	// 	active_camera,
	// },
	prelude::*,
};

use components::{camera_finder,};

#[main]
pub fn main() {
	query(camera_finder()).each_frame(|camera_entities|{
        let input = input::get();
		let (id, _) = camera_entities[0];
        let ray = camera::screen_position_to_world_ray(id, input.mouse_position);
		messages::PinRay{ray_origin:ray.origin, ray_dir:ray.dir}.send_server_unreliable(); // send a ray every frame
	});
}
