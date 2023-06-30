

mod sceneloader;

#[main]
pub fn main() {
    let main_camera_ent = Entity::new()
        .with_merge(make_transformable())
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 1.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    let nodes = sceneloader::test_get_nodes();

    sceneloader::debug_nodes(&nodes);

    for (_key, node) in nodes {
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
            "camera"=>{
                println!("Yes: Found camera! @ pos {:?} rot {:?}", node_pos.unwrap(), node_rot.unwrap());
                entity::set_component(main_camera_ent, translation(), node_pos.unwrap());
                entity::set_component(main_camera_ent, rotation(), node_rot.unwrap());
            },
            _=>{
                if let Some(path) = node.path {
                    Entity::new()
                        .with_merge(make_transformable())
                        // .with_default(cube())
                        .with(translation(), node_pos.unwrap())
                        .with(rotation(), node_rot.unwrap())
                        .with(prefab_from_url(), asset::url("assets/".to_owned()+&path).unwrap())
                        .spawn();
                }
            }
        }
    }

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with_default(quad())
    //     .spawn();

    println!("Hello, Ambient!");
}

use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::{prefab_from_url},
        primitives::{cube, quad},
        transform::{lookat_target, translation, rotation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};







