

mod sceneloader;

#[main]
pub fn main() {
    for (_key, node) in sceneloader::test_get_nodes() {
        let node_pos : Option<Vec3> = node.pos;
        let node_rot : Option<Quat> = node.rot;
        match node.name.as_str() {
            "cube1" | "cube2"=>{
                println!("Spawn one cube @ pos {:?} rot {:?}", node_pos.unwrap(), node_rot.unwrap());
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(translation(), node_pos.unwrap())
                    .with(rotation(), node_rot.unwrap())
                    .spawn();
            },
            _=>{

            }
        }
    }
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with_default(quad())
    //     .spawn();

    println!("Hello, Ambient!");
}

fn try_get_position_from_transform(transform : &Option<Vec<f32>>) -> Option<Vec3> {
    if let Some(t2) = transform {
        Some(Vec3::new(t2[9], t2[10], t2[11]))
    } else { None }
}

use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        primitives::{cube, quad},
        transform::{lookat_target, translation, rotation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};







