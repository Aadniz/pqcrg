extends Node3D

@onready var pause_menu = $pause_menu


var paused = false

func _ready():
	Engine.time_scale = 1

func _process(delta):
	if Input.is_action_just_pressed("pause"):
		pauseMenu()
# Called when the node enters the scene tree for the first time.
func pauseMenu():
	if paused:
		pause_menu.hide()
		Engine.time_scale = 1
	else:
		pause_menu.show()
		Engine.time_scale = 0
	paused = !paused
