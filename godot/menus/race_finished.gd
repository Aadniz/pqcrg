extends Control
@onready var main = $"../"

func _on_quit_pressed():
	main.disconnect_player()
	OS.delay_msec(100)
	get_tree().quit()


func _on_restart_pressed():
	pass # Replace with function body.
