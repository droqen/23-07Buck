struct GodotNode {
    name : String,
    position : Option<Vec<f32>>,
    path : Option<String>,
}

fn main() {
    let file_contents = std::fs::read_to_string("main.tscn")
        .expect("Failed to read the file");
    // println!("main.tscn contents = \n{file_contents}");
    let mut ext_paths : HashMap<String, String> = HashMap::new();
    let mut nodes : HashMap<String, GodotNode> = HashMap::new();
    let re_positioned_nodes = Regex::new(r#"\[node name=\"([^\"]*)\"[^\[]*Transform3D\((.*)\)"#).unwrap();
    for cap in re_positioned_nodes.captures_iter(&file_contents) {
        nodes.insert(cap[1].to_string(), GodotNode{
            name:cap[1].to_string(),
            position:Some(
                cap[2]
                    .split(", ")
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect::<Vec<f32>>()
            ),
            path:None, });
        // println!("Node header: {} @ t{}", &cap[1], &cap[2]);
    }

    let re_unpositioned_nodes = Regex::new(r#"\[node name=\"([^\"]*)\""#).unwrap();
    for cap in re_unpositioned_nodes.captures_iter(&file_contents) {
        if !nodes.contains_key(&cap[1]) {
            nodes.insert(cap[1].to_string(), GodotNode{
                name:cap[1].to_string(),
                position:None,
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

    for (_key,node) in nodes {
        println!("node\n\t~{}\n\t~{:?}\n\t~{:?}", node.name, node.position, node.path);
    }
}

use std::{collections::HashMap};

use regex::Regex;