extends MeshInstance3D

func _on_area_3d_body_entered(body):
	if body.is_in_group("player"):
		body.set_checkpoint(position)
