extends Node3D
@onready var main = $"../../../../"


func _on_area_3d_body_entered(body):
	if body.is_in_group("player"):
		main.check_checkpoints(body.name.to_int())
		
