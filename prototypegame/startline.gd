extends Node3D

var car = preload("res://car.tscn")
# Called when the node enters the scene tree for the first time.
func _ready():
	spawn()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func spawn():
	var instance = car.instantiate()
	var instance2 = car.instantiate()
	add_child(instance)
