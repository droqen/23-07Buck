pub const contents : &str = r#"
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