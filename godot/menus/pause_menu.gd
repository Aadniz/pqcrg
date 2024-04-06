extends Control
@onready var main = $"../"
@onready var pause_menu = $"."


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_resume_pressed():
	main.pause()

func _on_main_menu_pressed():
	main.main_menu()
	pause_menu.hide()
	main.exit_game()


func _on_quit_pressed():
	main.exit_game()
	OS.delay_msec(100)
	get_tree().quit()

