extends Node3D
@onready var main = $"../../../../"


# Called when the node enters the scene tree for the first time.
func _ready():
	main.set_start(position)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_area_3d_body_entered(body):
	pass
