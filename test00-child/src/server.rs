use ambient_api::{
    components::core::{
        app::{main_scene, name},
        camera::aspect_ratio_from_window,
        ecs::{children, parent},
        player::{player, user_id},
        primitives::{cube},
        rendering::color,
        transform::{lookat_target, translation, local_to_parent},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable, make_sphere},
    prelude::*,
};

const RED3 : Vec3 = vec3(1., 0., 0.);
const GRN3 : Vec3 = vec3(0., 1., 0.);
const BLU3 : Vec3 = vec3(0., 0., 1.);
const WHT3 : Vec3 = vec3(1., 1., 1.);

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(5., 5., 4.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();
    
    spawn_parent_child_pair(vec3(0., 0., 0.), RED3, "red", false, false);
    spawn_parent_child_pair(vec3(0., -2., 0.), GRN3, "green", true, false);
    spawn_parent_child_pair(vec3(0., -4., 0.), BLU3, "blue", false, true);
    spawn_parent_child_pair(vec3(0., -6., 0.), WHT3, "white", true, true);

    query((translation(), color())).requires(children()).each_frame(|parents|{
        for (ent, (_pos, _col)) in parents {
            entity::mutate_component(ent, translation(), move |mut pos|{
                // println!("{0:?} moved to -> {1:?}", _col, pos);
                if pos.x < -5. {
                    *pos += vec3(10., 0., 0.);
                } else {
                    *pos += vec3(-0.1, 0., 0.);
                }
            });
            entity::mutate_component(ent, rotation(), move |mut rot|{
                *rot *= Quat::from_rotation_z(-0.1);
            });
        }
    });
}

fn spawn_parent_child_pair(startPos : Vec3, color3 : Vec3, parent_name : &str, with_cube : bool, with_local_to_world : bool) -> EntityId {
    let a = make_transformable()
        .with(translation(), startPos)
        .with(color(), color3.extend(1.))
        .with(name(), parent_name.to_string())
        .spawn();

    if with_cube { entity::add_component(a, cube(), ()); }
    if with_local_to_world { entity::add_component(a, local_to_world(), Mat4::IDENTITY); }

    let b = Entity::new()
        .with_default(cube())
        .with(translation(), Vec3::Z * 2.)
        .with(color(), color3.extend(1.))
        .with(parent(), a)
        .with_default(local_to_parent())
        .spawn();

    entity::add_component(a, children(), vec![b]);

    return a;
}