extends Node3D
@onready var main = $"../../../../"


# Called when the node enters the scene tree for the first time.
func _ready():
	main.set_start(position, Vector3(rotation.x+PI/2,rotation.y+PI/2,rotation.z))

