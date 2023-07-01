pub const contents : &str = r#"
[gd_scene load_steps=3 format=3 uid="uid://b40ya78wp0im2"]

[ext_resource type="PackedScene" uid="uid://lv3sj0ceuydt" path="res://buildingside_only.glb" id="1_6fa1a"]

[sub_resource type="BoxMesh" id="BoxMesh_djo1f"]

[node name="model_place" type="Node3D"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0.168754, 0.340213)

[node name="buildingside_only" parent="." instance=ExtResource("1_6fa1a")]
transform = Transform3D(-2.64382, 0, 4.24384, 0, 5, 0, -4.24384, 0, -2.64382, -0.288356, 0, -0.364163)

[node name="Camera3D" type="Camera3D" parent="."]
transform = Transform3D(0.707107, -0.385118, 0.59303, 0, 0.838671, 0.544639, -0.707107, -0.385118, 0.59303, 1, 1, 1)

[node name="cube1" type="MeshInstance3D" parent="."]
transform = Transform3D(0.854206, 0, 0.519935, 0, 1, 0, -0.519935, 0, 0.854206, -1.38982, 0.783407, -0.577384)
mesh = SubResource("BoxMesh_djo1f")

[node name="cube2" type="MeshInstance3D" parent="."]
transform = Transform3D(0.851983, 0, 0.52357, 0, 0.0458346, 0, -0.52357, 0, 0.851983, -0.155422, -0.269891, -0.257289)
mesh = SubResource("BoxMesh_djo1f")

"#;