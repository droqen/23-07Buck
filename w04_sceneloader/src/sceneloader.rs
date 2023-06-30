pub struct GodotNode {
    pub name : String,
    pub pos : Option<Vec3>,
    pub rot : Option<Quat>,
    pub path : Option<String>,
}

fn main() {
    debug_nodes(&test_get_nodes());
}

pub fn debug_nodes(nodes : &HashMap<String, GodotNode>) {
    for (_key,node) in nodes {
        println!("node\n\t~{}\n\t~p{:?}\n\t~r{:?}\n\t~{:?}", node.name, node.pos, node.rot, node.path);
    }
}

mod scene_buildingside;
const scene_raw : &str = scene_buildingside::contents;

pub fn test_get_nodes() -> HashMap<String, GodotNode> {
    // let text_contents = std::fs::read_to_string("cubesSceneTest.tscn")
    //     .expect("Failed to read the file");

    // println!("main.tscn contents = \nscene_raw}");
    let mut ext_paths : HashMap<String, String> = HashMap::new();
    let mut nodes : HashMap<String, GodotNode> = HashMap::new();
    let re_positioned_nodes = Regex::new(r#"\[node name=\"([^\"]*)\"[^\[]*Transform3D\((.*)\)"#).unwrap();
    for cap in re_positioned_nodes.captures_iter(scene_raw) {
        let t : Vec<f32> = cap[2]
            .split(", ")
            .filter_map(|s| s.parse::<f32>().ok())
            .collect::<Vec<f32>>();
        nodes.insert(cap[1].to_string(), GodotNode{
            name:cap[1].to_string(),
            pos:Some(Vec3{x:t[9], z:t[10], y:t[11]}),
            rot:Some(
                Quat::from_mat3(&Mat3{
                    x_axis: Vec3{x:t[0], z:-t[1], y:t[2]},
                    z_axis: -Vec3{x:t[3], z:-t[4], y:t[5]},
                    y_axis: Vec3{x:t[6], z:-t[7], y:t[8]},
                })
                * Quat::from_rotation_z(90_f32.to_radians())
            ),
            path:None, });
        // println!("Node header: {} @ t{}", &cap[1], &cap[2]);
    }

    let re_unpositioned_nodes = Regex::new(r#"\[node name=\"([^\"]*)\""#).unwrap();
    for cap in re_unpositioned_nodes.captures_iter(scene_raw) {
        if !nodes.contains_key(&cap[1]) {
            nodes.insert(cap[1].to_string(), GodotNode{
                name:cap[1].to_string(),
                pos:None,
                rot:None,
                path:None,
            });
        }
    }

    let re_ext_resources = Regex::new(r#"path=\"res:\/\/(.*\.[^\"]*).*id=\"([^\"]*)"#).unwrap();
    for cap in re_ext_resources.captures_iter(scene_raw) {
        ext_paths.insert(cap[2].to_string(), cap[1].to_string());
    }

    let re_ext_resource_nodes = Regex::new(r#"\[node name=\"([^\"]*)\".*ExtResource\(\"([^\"]*)\"\)"#).unwrap();
    for cap in re_ext_resource_nodes.captures_iter(scene_raw) {
        if nodes.contains_key(&cap[1]) && ext_paths.contains_key(&cap[2]) {
            if let Some(node) = nodes.get_mut(&cap[1]) {
                node.path = Some(ext_paths[&cap[2]].to_string());
            };
        } else {
            println!("ERR: could not find node {:?} or path {:?}", &cap[1], &cap[2]);
        }
    }

    nodes
}

use std::{collections::HashMap};

use regex::Regex;

use glam::f32::{Vec3, Mat3, Quat};