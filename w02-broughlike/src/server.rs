#[main]
pub fn main() {

    setup_camera();
    setup_environment();
    setup_static_obstacles();

    query( ( cx(), cy(), translation(), ) ).each_frame(|grid_movers|{
        for (id,(x,y,cur_pos)) in grid_movers {
            entity::set_component(id, translation(), (cur_pos + cxy_to_vec3(x, y))/2. );
        }
    });

    // let pinControlled = query((cx(),cy())).requires(controlled_by_pin()).build();
    messages::PlayerCommand::subscribe(move |source,msg|{
        let Some(player_id) = source.client_entity_id() else { return; };
        entity::add_component(player_id, move_cx(), msg.mvx);
        entity::add_component(player_id, move_cy(), msg.mvy);
        // entity::mutate_component(player_id, cx(), |cx|*cx += msg.mvx);
        // entity::mutate_component(player_id, cy(), |cy|*cy += msg.mvy);
        // for (id,(cx0,cy0)) in pinControlled.evaluate() {
        //     entity::set_component(id, cx(), cx0 + msg.mvx);
        //     entity::set_component(id, cy(), cy0 + msg.mvy);
        // }
    });

    let find_grid_entities = query( (cx(),cy()) ).build();
    query( (cx(),cy(),move_cx(),move_cy()) ).each_frame(
        move |grid_movers|for(id,(x,y,dx,dy))in grid_movers{
            let x2=x+dx; let y2=y+dy;
            let mut blocking_ents : Vec<EntityId> = vec![];
            for (bid,(bx,by)) in find_grid_entities.evaluate() {
                if bx==x2 && by==y2 {blocking_ents.push(bid)};
            }
            if blocking_ents.is_empty() {
                entity::mutate_component(id, cx(), |curx|*curx=x2);
                entity::mutate_component(id, cy(), |cury|*cury=y2);
            } else {
                entity::mutate_component(id, translation(), |pos|*pos=(*pos+cxy_to_vec3(x2, y2))/2.);
                println!("Movement Blocked");
                push_ent(blocking_ents[0], id, dx, dy);
            }
            entity::remove_components(id, &[&move_cx(), &move_cy()]);
        }
    );

    query( (cx(),cy(),pushed_dx(),pushed_dy(),pushed_src()) ).each_frame(
        move|grid_pushed|for(id,(x,y,dx,dy,src))in grid_pushed{
            println!("A push was attempted.");
            dbg!(entity::get_component(id, name()));
            match dbg!(entity::get_component(id, pushable())) {
                Some(_) => {
                    entity::add_component(id, move_cx(), dx);
                    entity::add_component(id, move_cy(), dy);
                }
                _ => {
                    entity::mutate_component(id, translation(), |pos|*pos=0.75**pos + 0.25*cxy_to_vec3(x+dx, y+dy));
                }
            }
            entity::remove_components(id, &[&pushed_dx(),&pushed_dy(),&pushed_src()]);
        }
    );

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
                    .with_merge(make_cxyent(cx0, cy0))
            );

            entity::add_child(grid_player, Entity::new()
                .with(translation(), vec3(0., 0., 0.5))
                .with(scale(), Vec3::splat(0.5))
                // .with_default(cube())
                    .with(prefab_from_url(), asset::url("assets/person figure.glb").unwrap())
                    .with(scale(), vec3(0.15, 0.20, 0.15))
                    .with(rotation(), Quat::from_rotation_z(PI*0.25))
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

fn make_cxyent(x : i32, y : i32) -> Entity {
    Entity::new()
        .with(cx(), x)
        .with(cy(), y)
        .with(translation(), cxy_to_vec3(x, y))
}
fn cxy_to_vec3(x : i32, y : i32) -> Vec3 { vec2(x as f32, y as f32).extend(0.) }
fn push_ent(pushed : EntityId, pusher : EntityId, dx : i32, dy : i32) {
    entity::add_components(pushed, Entity::new()
        .with(pushed_src(), pusher)
        .with(pushed_dx(), dx)
        .with(pushed_dy(), dy)
    );
}

fn setup_camera() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(0., 10., 10.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();
}

fn setup_environment() {
    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(),Vec3::splat(20.))
        .with(color(),vec3(0.5,0.5,0.5).extend(1.))
        .spawn();
}

fn setup_static_obstacles() {
    let grid_wall = Entity::new()
        .with(name(), "grid_wall".to_string())
        .with_merge(make_transformable())
        .with_merge(make_grid_creature())
        .with_default(local_to_world())
        .with_merge(make_cxyent(0, 0))
        .spawn();
    entity::add_child(grid_wall, Entity::new()
        .with(translation(), vec3(0., 0., 0.1))
        .with(scale(), Vec3::splat(0.9))
        .with_default(cube())
        .with_default(local_to_parent())
        .with(color(), vec3(0.1, 0.1, 0.1).extend(1.))
    .spawn());

    let grid_crate = Entity::new()
        .with(name(), "grid_crate".to_string())
        .with_merge(make_transformable())
        .with_merge(make_grid_creature())
        .with_default(pushable())
        .with_default(local_to_world())
        .with_merge(make_cxyent(2, 0))
        .spawn();
    entity::add_child(grid_crate, Entity::new()
        .with(translation(), vec3(0., 0., 0.2))
        .with(scale(), Vec3::splat(0.8))
        // .with_default(cube())
                .with(prefab_from_url(), asset::url("assets/cactus plant block.glb").unwrap())
                .with(rotation(),Quat::from_rotation_x(1.0) * Quat::from_rotation_z(1.0) * Quat::from_rotation_y(-0.5))
        .with_default(local_to_parent())
        // .with(color(), vec3(1., 0.5, 0.).extend(1.))
    .spawn());
}

use std::f32::consts::PI;

use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::{prefab_from_url},
        primitives::{cube, quad},
        rendering::{color},
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*, entity::mutate_component,
};

use components::{cx, cy, move_cx, move_cy, pushed_src, pushed_dx, pushed_dy, health, controlled_by_pin,pushable, };
use concepts::{make_grid_creature, make_grid_player, };