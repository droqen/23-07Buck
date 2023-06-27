use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::{children,parent,},
        physics::{cube_collider,visualize_collider,},
        primitives::{cube,quad,},
        transform::{lookat_target, translation, local_to_parent, local_to_world, },
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    let parent_position_only = Entity::new()
        .with(name(), "position only (parent)".to_string())
        .with_merge(make_transformable())
        .with(translation(), vec3(-2., 2., 0.))
        .with_default(local_to_world())
        .spawn();

    let child_mesh_and_collider = Entity::new()
        .with(name(), "mesh and collider (child)".to_string())
        .with_default(cube())
        .with(cube_collider(), Vec3::ONE)
        .with_default(visualize_collider())
        .with_default(local_to_parent())
        .spawn();

    entity::add_component(parent_position_only, children(), vec![child_mesh_and_collider]);
    entity::add_component(child_mesh_and_collider, parent(), parent_position_only);

    let all_in_one = Entity::new()
        .with(name(), "all components in one".to_string())
        .with_merge(make_transformable())
        .with(translation(), vec3(2., -2., 0.))
        .with_default(cube())
        .with(cube_collider(), Vec3::ONE)
        .with_default(visualize_collider())
        .spawn();

    //showing how each responds to translation, rotation, scale
    
    query(rotation()).excludes(lookat_target()).each_frame(|rotatables|{
        for (id,(_rotation)) in rotatables {
            entity::mutate_component(id,rotation(),|mut rot|{
                *rot *= Quat::from_rotation_y(0.01);
            });
            entity::mutate_component(id,translation(),|mut pos|{
                *pos += vec3(0., 0., 0.01);
                if pos.z > 1. { *pos += vec3(0., 0., -2.); }
            });
            entity::mutate_component(id,scale(),|mut scl|{
                *scl -= vec3(0.005, 0.005, 0.005);
                if scl.z < 0.5 { *scl += vec3(1., 1., 1.); }
            });
        }
    });
}