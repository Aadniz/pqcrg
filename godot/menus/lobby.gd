extends Control
@onready var main = $"../"
@onready var lobby = $"."


func _on_start_pressed():
	main.start_race()

func _on_quit_pressed():
	main.quit_lobby()
	main.exit_game()
