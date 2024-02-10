extends Control

@onready var main = $"../"

func _on_resume_pressed():
	main.pauseMenu()


func _on_main_menu_pressed():
	get_tree().change_scene_to_file("res://main_menu.tscn")


func _on_exit_game_pressed():
	get_tree().quit()
