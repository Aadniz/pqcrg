extends Control
@onready var main = $"../"

func _on_quit_pressed():
	main.disconnect_player()
	OS.delay_msec(100)
	get_tree().quit()


func _on_restart_pressed():
	main.del_map()
	main.show_lobby()
	hide()
	main.del_player.rpc(multiplayer.get_unique_id())
