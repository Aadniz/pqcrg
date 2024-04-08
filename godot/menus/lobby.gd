extends Control
@onready var main = $"../"
@onready var lobby = $"."
@onready var player_list = $VBoxContainer2/Player_List
@onready var start = $VBoxContainer2/HBoxContainer/Start

		
func hide_start():
	start.hide()
	
func _on_start_pressed():
	main.start_race.rpc()

func _on_quit_pressed():
	hide()
	main.disconnect_player()

func update_player_list():
	var temp_str = ""
	for i in GameManager.Players:
			temp_str += "%s \n" % str(GameManager.Players[i].id)
	player_list.text = temp_str
