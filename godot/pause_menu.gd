extends Control
@onready var main = $"../../"
@onready var player = $"../"
@onready var pause_menu = $"."

func _process(delta):
	if Input.is_action_just_pressed("menu"):
		pause_menu.hide()

func _on_main_menu_pressed():
	main.main_menu()
	pause_menu.hide()


func _on_quit_pressed():
	main.exit_game(player.name.to_int())
	get_tree().quit()


func _on_resume_pressed():
	pause_menu.hide()
