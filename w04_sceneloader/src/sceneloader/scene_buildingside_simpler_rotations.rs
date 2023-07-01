pub const contents : &str = r#"
[gd_scene load_steps=3 format=3 uid="uid://b40ya78wp0im2"]

[ext_resource type="PackedScene" uid="uid://lv3sj0ceuydt" path="res://buildingside_only.glb" id="1_6fa1a"]

[sub_resource type="BoxMesh" id="BoxMesh_djo1f"]

[node name="model_place" type="Node3D"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0.168754, 0.340213)

[node name="buildingside_only" parent="." instance=ExtResource("1_6fa1a")]
transform = Transform3D(-1, 0, -8.74228e-08, 0, 1, 0, 8.74228e-08, 0, -1, -0.288356, 0, -0.364163)

[node name="Camera3D" type="Camera3D" parent="."]
transform = Transform3D(0.707107, -0.385118, 0.59303, 0, 0.838671, 0.544639, -0.707107, -0.385118, 0.59303, 1, 1, 1)

[node name="cube1" type="MeshInstance3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -0.142267, 2.38419e-07, -1.06209)
mesh = SubResource("BoxMesh_djo1f")
"#;