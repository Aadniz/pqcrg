extends Node3D
@export var player_scene : PackedScene

# Called when the node enters the scene tree for the first time.
func _ready():
	for i in GameManager.Players:
		var current_player = player_scene.instantiate()
		current_player.name = str(GameManager.Players[i].id)
		add_child(current_player)
		GameManager.Players[i].has_car = true
		print(current_player.name)
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
	
