#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(0., 3., 10.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .spawn();

    query( ( cx(), cy(), translation(), ) ).each_frame(|grid_movers|{
        for (id,(x,y,cur_pos)) in grid_movers {
            entity::set_component(id, translation(), (cur_pos + cxy_to_vec3(x, y))/2. );
        }
    });

    // let pinControlled = query((cx(),cy())).requires(controlled_by_pin()).build();
    messages::PlayerCommand::subscribe(move |source,msg|{
        let Some(player_id) = source.client_entity_id() else { return; };
        entity::mutate_component(player_id, cx(), |cx|*cx += msg.mvx);
        entity::mutate_component(player_id, cy(), |cy|*cy += msg.mvy);
        // for (id,(cx0,cy0)) in pinControlled.evaluate() {
        //     entity::set_component(id, cx(), cx0 + msg.mvx);
        //     entity::set_component(id, cy(), cy0 + msg.mvy);
        // }
    });

    spawn_query( (player(), user_id()) ).bind(|spawnedPlayers| {
        for (grid_player,(_,uid)) in spawnedPlayers {
            let cx0 = random::<i32>()%5-2;
            let cy0 = random::<i32>()%5-2;
            entity::add_components(
                grid_player,
                Entity::new()
                    .with(name(), "grid_player".to_string())
                    .with_merge(make_transformable())
                    .with_merge(make_grid_creature())
                    .with_merge(make_grid_player())
                    .with_default(local_to_world())
                    .with(cx(), cx0)
                    .with(cy(), cy0)
                    .with(translation(), cxy_to_vec3(cx0, cy0))
            );

            entity::add_child(grid_player, Entity::new()
                .with(translation(), vec3(0., 0., 0.5))
                .with(scale(), Vec3::splat(0.5))
                .with_default(cube())
                .with_default(local_to_parent())
                .with(user_id(), uid)
                .with(color(), random::<Vec3>().extend(1.)) // TODO: player colour should eventually derive from parent
            .spawn());
        }
    });

    // When a player despawns, clean up their objects. (Taken from minigolf)
    let player_objects_query = query(user_id()).build();
    despawn_query(user_id()).requires(player()).bind({
        move |players| {
            let player_objects = player_objects_query.evaluate();
            for (_, player_user_id) in &players {
                for (id, _) in player_objects
                    .iter()
                    .filter(|(_, object_user_id)| *player_user_id == *object_user_id)
                {
                    entity::despawn(*id);
                }
            }
        }
    });
}

fn cxy_to_vec3(x : i32, y : i32) -> Vec3 { vec2(x as f32, y as f32).extend(0.) }

use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        primitives::{cube, quad},
        rendering::{color},
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use components::{cx, cy, health, controlled_by_pin, };
use concepts::{make_grid_creature, make_grid_player, };