use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::{children, parent, },
        physics::{cube_collider, visualize_collider, },
        primitives::{cube, quad, },
        rendering::{color,},
        transform::{lookat_target, translation, local_to_parent, local_to_world, },
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use components::{hexx, hexy, hexheight, camera_finder};

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
        .with_default(camera_finder())
        .with(translation(), vec3(0., 10., 5.))
        .with(lookat_target(), vec3(0., 5., 0.))
        .spawn();

    change_query( (color(),hexheight(),children(),hexx(),hexy()) )
        .track_change((color(),hexheight()))
        .bind(move |hexes| {
            for (_, (new_hex_colour,hh,children,_hx,_hy)) in hexes {
                for child_of_hex in children {
                    entity::add_component(child_of_hex, color(), new_hex_colour);
                    entity::mutate_component(child_of_hex, translation(), |pos| { pos.z = 0.5 * hh; });
                    entity::mutate_component(child_of_hex, scale(), |siz| { siz.z = 1.0 * hh; });
                }
            }
        });

    messages::PinRay::subscribe(move |_source,msg|{
        if let Some(hit) = physics::raycast_first(msg.ray_origin, msg.ray_dir) {
            if let Some(parent_hex) = entity::get_component(hit.entity, parent()) {
                if entity::has_component(parent_hex, hexheight()) {
                    entity::mutate_component(parent_hex, hexheight(), |hh| {
                        *hh += 0.01;
                        println!("Mutating...");
                    });
                } else {
                    println!("Parent has no hh");
                }
            } else {
                println!("No parent");
            }
            // entity::set_component(cube_id, translation(), hit.position);
            // messages::WorldPosition::new(hit.position).send_client_broadcast_unreliable();
        } else {
            println!("No raycast hit");
        }
    });

    generate_hexgrid();

    for hex in get_neighbour_hexes(0, 5) {
        entity::add_component(hex, color(), vec3(1., 1., 1.).extend(1.));
    }
}

fn generate_hexgrid() {
    // big screen-filling grid
    for x in -4..5 { for y in 0..10 { make_hex(x-y/2,y); } }

    // // tiny test grid
    // for x in -1..1+1 { for y in 4..6+1 { make_hex(x-y/2,y); } }

    // // narrow test grid
    // for x in -4..5 { for y in 0..2 { make_hex(x,y); } }
}

fn get_hexgrid_dist(hx0 : &i32, hy0 : &i32, hx1 : &i32, hy1 : &i32) -> u32 {
    let dist = (i32::abs(hx0-hx1) + i32::abs(hy0-hy1) + i32::abs(hx0+hy0-hx1-hy1)) as u32 / 2;
    dist
}

fn get_neighbour_hexes(hx0 : i32, hy0 : i32) -> Vec<EntityId> {
    query((hexx(), hexy()))
        .build()
        .evaluate()
        .iter()
        .filter_map(|(id, (hx1, hy1,))| (get_hexgrid_dist(&hx0, &hy0, &hx1, &hy1) == 1).then_some(id)) // using borrows. why???
        .map(|id| id.clone()) // instead of .into_bindgen(), used .clone() - how wrong is this??
        .collect()
}

fn make_hex(hx : i32, hy : i32) -> EntityId {
    let hex = Entity::new()
        .with(hexx(), hx)
        .with(hexy(), hy)
        .with(hexheight(), (0.5 + 1.5 * random::<f32>()))
        .with_merge(make_transformable())
        .with(translation(), get_hex_translation(hx, hy))
        .with(scale(), Vec3::splat(0.95))
            .with(color(), (random::<Vec3>()*0.2).extend(1.))
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

fn make_hex_third( hex_parent : EntityId, third_index : u8) -> EntityId {
    Entity::new()
        .with_default(cube())
        .with(cube_collider(), Vec3::ONE)
        .with_default(visualize_collider())
        // .with_default(quad())
        .with_default(local_to_parent())
        .with(parent(), hex_parent)
        .with(scale(), vec3(1., 0.5774, 0.1))
        .with(rotation(), Quat::from_rotation_z(PI_THIRD * third_index as f32))
        .spawn()
}