

#[main]
pub async fn main() {
    let camera_eye = Entity::new()
        .with_merge(make_transformable())
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(0., 15., -5.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    ambient_api::messages::Frame::subscribe(move |_|{
        entity::set_component(camera_eye, translation(), vec3(
            0. + 5. * (time().as_secs_f64()*1.0).sin() as f32,
            15. + (time().as_secs_f64() * 0.03).sin() as f32,
            -5.
        ));
    });

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with_default(quad())
    //     .with(scale(), Vec3::splat(5.0))
    //     .with(
    //         pbr_material_from_url(),
    //         asset::url("assets/handmadeguy/handmadeguy/pipeline.toml/0/mat.json").unwrap())
    //     .spawn();

    let blender_scene = Entity::new()
        .with(name(), "BLENDER SCENE".to_string())
        .with(prefab_from_url(),asset::url("assets/buildingside/buildingside.glb").unwrap())
        .with(rotation(), Quat::from_euler(glam::EulerRot::XYZ, PI*1.5, PI*1.0, PI*0.))
        .with(scale(),Vec3::splat(10.))
        .spawn();
    
    println!("BEFORE AWAIT...");
    println!("BEFORE AWAIT...");
    println!("BEFORE AWAIT...");

    entity::wait_for_component(blender_scene, spawned()).await;
    sleep(5.).await;

    println!("AFTER AWAIT...");
    println!("AFTER AWAIT...");
    println!("AFTER AWAIT...");

    dbg!(entity::get_component(blender_scene, name()));
    dbg!(entity::get_component(blender_scene, children()));
    for (id,(entname)) in query(name()).build().evaluate() {
        if entname=="BLENDER SCENE".to_string() {
            println!("Found an entity {} with name {}", id, entname);
        }
    }

    println!("Hello, Ambient!");
}

async fn load_scene() {

}

use std::f32::consts::PI;

use ambient_api::{
    components::core::{
        app::{name,main_scene},
        camera::aspect_ratio_from_window,
        ecs::children,
        primitives::quad,
        prefab::{prefab_from_url, spawned},
        rendering::{sun,pbr_material_from_url,light_diffuse},
        transform::{lookat_target, translation,rotation,scale,local_to_parent,local_to_world},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};