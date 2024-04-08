extends Control
@onready var main = $"../"
@onready var lobby = $"."
@onready var player_list = $VBoxContainer2/Player_List


func _on_start_pressed():
	hide()
	main.start_race()

func _on_quit_pressed():
	hide()
	main.disconnect_player()

func update_player_list():
	var temp_str = ""
	for i in GameManager.Players:
			temp_str += "%s \n" % str(GameManager.Players[i].id)
	player_list.text = temp_str
