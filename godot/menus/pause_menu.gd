extends Control
@onready var main = $"../"
@onready var pause_menu = $"."


func _on_resume_pressed():
	main.pause()

func _on_main_menu_pressed():
	main.pause()
	main.disconnect_player()

func _on_quit_pressed():
	main.disconnect_player()
	OS.delay_msec(100)
	get_tree().quit()

