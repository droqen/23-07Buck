pub struct GodotNode {
    pub name : String,
    pub pos : Option<Vec3>,
    pub rot : Option<Quat>,
    pub path : Option<String>,
}

fn main() {
    debug_nodes(test_get_nodes());
}

pub fn debug_nodes(nodes : HashMap<String, GodotNode>) {
    for (_key,node) in nodes {
        println!("node\n\t~{}\n\t~p{:?}\n\t~r{:?}\n\t~{:?}", node.name, node.pos, node.rot, node.path);
    }
}

pub fn test_get_nodes() -> HashMap<String, GodotNode> {
    // let file_contents = std::fs::read_to_string("cubesSceneTest.tscn")
    //     .expect("Failed to read the file");

    let file_contents = r#"
[gd_scene load_steps=3 format=3 uid="uid://cw7bdgetk377t"]

[ext_resource type="Script" path="res://transform_debug.gd" id="1_x1vfo"]

[sub_resource type="BoxMesh" id="BoxMesh_oj0wp"]

[node name="sceneTest" type="Node3D"]

[node name="cube1" type="MeshInstance3D" parent="."]
transform = Transform3D(0.811788, 0, 0.583952, 0, 1, 0, -0.583952, 0, 0.811788, 0, 0, 1.21846)
mesh = SubResource("BoxMesh_oj0wp")

[node name="transform_debug" type="Node" parent="cube1"]
script = ExtResource("1_x1vfo")
transform = Transform3D(0.811788, 0, 0.583952, 0, 1, 0, -0.583952, 0, 0.811788, 0, 0, 1.21846)
origin = Vector3(0, 0, 1.21846)

[node name="cube2" type="MeshInstance3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1.25322, 0)
mesh = SubResource("BoxMesh_oj0wp")

[node name="camera" type="Camera3D" parent="."]
transform = Transform3D(0.707107, -0.5, 0.5, 0, 0.707107, 0.707107, -0.707107, -0.5, 0.5, 5, 5, 5)

[node name="transform_debug2" type="Node" parent="camera"]
script = ExtResource("1_x1vfo")
transform = Transform3D(0.707107, -0.5, 0.5, 0, 0.707107, 0.707107, -0.707107, -0.5, 0.5, 5, 5, 5)
origin = Vector3(5, 5, 5)
    "#;

    // println!("main.tscn contents = \n{file_contents}");
    let mut ext_paths : HashMap<String, String> = HashMap::new();
    let mut nodes : HashMap<String, GodotNode> = HashMap::new();
    let re_positioned_nodes = Regex::new(r#"\[node name=\"([^\"]*)\"[^\[]*Transform3D\((.*)\)"#).unwrap();
    for cap in re_positioned_nodes.captures_iter(&file_contents) {
        let t : Vec<f32> = cap[2]
            .split(", ")
            .filter_map(|s| s.parse::<f32>().ok())
            .collect::<Vec<f32>>();
        nodes.insert(cap[1].to_string(), GodotNode{
            name:cap[1].to_string(),
            pos:Some(Vec3::new(t[9], t[10], t[11])),
            rot:Some(
                Quat::from_mat3(&Mat3{
                    x_axis: Vec3::new(t[0], t[1], t[2]),
                    y_axis: Vec3::new(t[3], t[4], t[5]),
                    z_axis: Vec3::new(t[6], t[7], t[8]),
                })
            ),
            path:None, });
        // println!("Node header: {} @ t{}", &cap[1], &cap[2]);
    }

    let re_unpositioned_nodes = Regex::new(r#"\[node name=\"([^\"]*)\""#).unwrap();
    for cap in re_unpositioned_nodes.captures_iter(&file_contents) {
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
    for cap in re_ext_resources.captures_iter(&file_contents) {
        ext_paths.insert(cap[2].to_string(), cap[1].to_string());
    }

    let re_ext_resource_nodes = Regex::new(r#"\[node name=\"([^\"]*)\".*ExtResource\(\"([^\"]*)\"\)"#).unwrap();
    for cap in re_ext_resource_nodes.captures_iter(&file_contents) {
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