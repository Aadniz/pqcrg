extends Control
@onready var main = $"../"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_quit_pressed():
	main.exit_game()
	OS.delay_msec(100)
	get_tree().quit()


func _on_restart_pressed():
	pass # Replace with function body.
