use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        primitives::{cube, quad},
        rendering::{
            color, fog_density, light_diffuse, sky, sun,
        },
        transform::{lookat_target, translation, scale},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable, make_sphere},
    prelude::*,
};

use components::{dofpitch,dofroll,dofyaw,
    dofpitch_vel,dofroll_vel,dofyaw_vel,
    dofpitch_pin,dofroll_pin,dofyaw_pin,
    throttle,throttle_pin};
use components::{temppointerent, tempcameraent };

#[main]
pub fn main() {
    spawnEnvironment();

    let playerCamera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        // .with(lookat_target(), vec3(0., 0., 0.))
        .with_default(rotation())
        .spawn();

    let playerPointer = Entity::new()
        .with_merge(make_transformable())
        // .with_default(cube())
        .with_merge(make_sphere())
        .with(scale(),Vec3::splat(0.1))
        .with(color(), vec4(1., 1., 1., 1.))
        .spawn();

    let playerDoffer = Entity::new()
        .with_merge(make_transformable())
        .with(translation(),vec3(2.5, 0., 2.5))
        .with(rotation(), Quat::from_rotation_x(1.5))
        .with_default(cube())
        .with_default(dofpitch())
        .with_default(dofroll())
        .with_default(dofyaw())
        .with_default(dofpitch_vel())
        .with_default(dofroll_vel())
        .with_default(dofyaw_vel())
        .with_default(dofpitch_pin())
        .with_default(dofroll_pin())
        .with_default(dofyaw_pin())
        .with_default(throttle())
        .with_default(throttle_pin())
        .with(temppointerent(), playerPointer)
        .with(tempcameraent(), playerCamera)
        .with(color(), vec4(0.2, 0.8, 1., 1.))
        .spawn();

    query( (dofpitch(),dofroll(),dofyaw(),dofpitch_vel(),dofroll_vel(),dofyaw_vel(),dofpitch_pin(),dofroll_pin(),dofyaw_pin()) ).each_frame(|doffers|{
        for (doffer,(_p,_r,_y,pv,rv,yv,ppin,rpin,ypin)) in doffers {
            // entity::set_component(doffer, rotation(), Quat::from_euler(glam::EulerRot::XYZ, p, r, y));
            // entity::mutate_component(doffer, dofpitch(), move |mut p|{ *p += pv; });
            // entity::mutate_component(doffer, dofroll(), move |mut r|{ *r += rv; });
            // entity::mutate_component(doffer, dofyaw(), move |mut y|{ *y += yv; });
            entity::mutate_component(doffer, rotation(), move |mut rot|{
                *rot *= Quat::from_euler(glam::EulerRot::XZY,
                    pv * VELTOPIN_MAX_ROTSPEED,
                    rv * VELTOPIN_MAX_ROTSPEED,
                    yv * VELTOPIN_MAX_ROTSPEED);
            });
            entity::set_component(doffer, dofpitch_vel(), veltopin(pv, ppin));
            entity::set_component(doffer, dofroll_vel(), veltopin(rv, rpin));
            entity::set_component(doffer, dofyaw_vel(), veltopin(yv, ypin));
        }
    });

    query( (translation(), rotation(), temppointerent(), tempcameraent()) ).each_frame(|pointerHaver|{
        for (pointerHaver,(pos,rot,pointer,camera)) in pointerHaver {
            entity::set_component(pointer, translation(), pos + rot * vec3(0., 0., 1.));
            entity::set_component(camera, translation(), pos + rot * vec3(0., 0., -5.));
            entity::set_component(camera, rotation(), rot);
        }
    });

    query( (translation(), rotation(), throttle(), throttle_pin()) ).each_frame(|movers|{
        for (mover,(pos,rot,speed,speed_pin)) in movers {
            entity::set_component(mover, translation(), pos + rot * vec3(0., 0., speed * VELTOPIN_THROTTLE_SPEED));
            entity::set_component(mover, throttle(), veltopin(speed, speed_pin));
        };
    });

    messages::DofPryInput::subscribe(move |source, msg|{
        entity::set_component(playerDoffer, dofpitch_pin(), -msg.p);
        entity::set_component(playerDoffer, dofroll_pin(), -msg.r);
        entity::set_component(playerDoffer, dofyaw_pin(), msg.y);
        entity::set_component(playerDoffer, throttle_pin(), msg.throttle);
    });

    println!("Hello, Ambient!");
}

fn tow(a:f32,b:f32,linrate:f32,multrate:f32) -> f32 {
    let a2:f32 = clerp(a, b, multrate);
    if a2 + linrate < b { return a2 + linrate; }
    if a2 - linrate > b { return a2 - linrate; }
    return b;
}

const VELTOPIN_THROTTLE_SPEED : f32 = 1.00;
const VELTOPIN_MAX_ROTSPEED : f32 = 0.04;

const VELTOPIN_ACTIVE_LINEAR_RATE : f32 = 0.002;
const        VELTOPIN_ACTIVE_EXP_RATE : f32 = 0.002;
const VELTOPIN_PASSIVE_LINEAR_RATE : f32 = 0.001;
const        VELTOPIN_PASSIVE_EXP_RATE : f32 = 0.;
const VELTOPIN_DRAG : f32 = 0.99;

fn veltopin(vel : f32, pinvalue : f32) -> f32 {
    return tow(
        vel,
        pinvalue,
        clerp(VELTOPIN_PASSIVE_LINEAR_RATE, VELTOPIN_ACTIVE_LINEAR_RATE, abs(pinvalue)),
        clerp(VELTOPIN_PASSIVE_EXP_RATE, VELTOPIN_ACTIVE_EXP_RATE, abs(pinvalue))
    )
    * VELTOPIN_DRAG;
}

fn clerp(a : f32, b : f32, t : f32) -> f32 {
    if t <= 0. { return a;}
    if t >= 1. { return b;}
    return a + (b-a) * t;
}
fn abs(a : f32) -> f32 {
    if a < 0. { return -a; }
    return a;
}

fn spawnEnvironment() {
    Entity::new()
        .with_merge(make_transformable())
        .with_default(sky())
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(sun())
        .with_default(main_scene())
        .with(light_diffuse(), Vec3::ONE)
        .with(fog_density(), 0.)
        .with(rotation(), Quat::from_rotation_y(3.0))
        .spawn();

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with_default(quad())
    //     .with(scale(), Vec3::splat(100.))
    //     .with(color(), vec4(0.5, 0.75, 0.45, 1.))
    //     .spawn();

    for i in 0..100 {
        let spherePosition = (random::<Vec3>()*2.-Vec3::ONE) * 1000.;
        Entity::new()
            .with_merge(make_sphere())
            .with(translation(), spherePosition)
            .with(color(), random::<Vec3>().extend(1.))
            .with(scale(), Vec3::splat(5. + random::<f32>()*30.))
            .spawn();
    }
    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                if random::<f32>()<0.6 { continue; }
                let cubePosition = (uvec3(x,y,z).as_vec3() - Vec3::splat(5.)) * 40.;
                Entity::new()
                    .with(translation(), cubePosition)
                    .with_default(cube())
                    .with(color(), random::<Vec3>().extend(1.))
                    .with(scale(), Vec3::splat(40.))
                    .spawn();
            }
        }
    }
}