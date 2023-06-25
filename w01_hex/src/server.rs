use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::{children, parent, },
        primitives::{cube, quad, },
        transform::{lookat_target, translation, local_to_parent, local_to_world, },
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use std::f32::consts::{PI, TAU};

const SQRT_3 : f32 = 1.73205080757;
const PI_THIRD : f32 = PI / 3.;
const UNIT_HEXA_SIDE : f32 = 1. / SQRT_3;
const UNIT_HEXA_ROW_OFFSET : f32 = UNIT_HEXA_SIDE * 1.5;
// const UNIT_HEXA_LONGDIAG : f32 = 1.1547 // this is just two sides lol

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(0., 10., 5.))
        .with(lookat_target(), vec3(0., 5., 0.))
        .spawn();

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with_default(quad())
    //     .spawn();

    for x in -4..5 { for y in 0..10 {
        make_hex(x-y/2,y);
    }}

    println!("Hello, Ambient!");
}

fn make_hex(hx : i32, hy : i32) -> EntityId {
    let hex = Entity::new()
        .with_merge(make_transformable())
        .with(translation(), dbg!(get_hex_translation(hx, hy)))
        .with(scale(), Vec3::splat(0.95))
        .with_default(local_to_world())
        .spawn();
    entity::add_component(hex, children(), vec![
        make_hex_third(hex, 0),
        make_hex_third(hex, 1),
        make_hex_third(hex, 2),
    ]);
    hex
}

fn get_hex_translation(hx : i32, hy : i32) -> Vec3 {
    let hx : f32 = hx as f32;
    let hy : f32 = hy as f32;
    vec3(
        hx*1.0 + hy*0.5,
        hy*UNIT_HEXA_ROW_OFFSET,
        0.
    )
}

fn make_hex_third(hex_parent : EntityId, third_index : u8) -> EntityId {
    Entity::new()
        // .with_default(cube())
        .with_default(quad())
        .with_default(local_to_parent())
        .with(parent(), hex_parent)
        .with(scale(), vec3(1., 0.5774, 0.1))
        .with(rotation(), Quat::from_rotation_z(PI_THIRD * third_index as f32))
        .spawn()
}