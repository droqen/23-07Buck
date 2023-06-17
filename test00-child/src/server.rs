use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::{children, parent},
        physics::{
            character_controller_height, character_controller_radius, physics_controlled,
        },
        player::{player, user_id},
        primitives::{cube},
        rendering::{color},
        transform::{lookat_target, translation, local_to_parent},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable, make_sphere},
    prelude::*,
};

#[main]
pub fn main() {
    let bigCamera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(5., 5., 4.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    spawn_query((player(), user_id())).bind(move |players| {
        for (id, (_, uid)) in players {
            let playerAdditionalPiece = Entity::new()
                // .with_merge(make_perspective_infinite_reverse_camera())
                // .with(aspect_ratio_from_window(), EntityId::resources())
                .with_merge(make_sphere())
                .with_default(main_scene())
                .with(user_id(), uid)
                .with(translation(), Vec3::Z * 2.)
                .with(parent(), id)
                .with_default(local_to_parent())
                // .with(rotation(), Quat::from_rotation_x(PI / 2.))
                .spawn();

            entity::add_components(
                id,
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(color(), Vec4::ONE)
                    // .with(character_controller_height(), 2.)
                    // .with(character_controller_radius(), 0.5)
                    // .with_default(physics_controlled())
                    .with(children(), vec![playerAdditionalPiece])
            );
        }
    });

    query((player(), translation())).each_frame(|players|{
        for (player, (_player_component, _pos)) in players {
            physics::move_character(player, vec3(-1., 0., 0.), 100., frametime());
            entity::set_component(player, translation(), _pos + vec3(-0.1, 0., 0.));
        }
    });

    println!("Hello, Ambient!");
}
