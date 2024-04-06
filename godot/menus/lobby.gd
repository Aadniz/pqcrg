extends Control
@onready var main = $"../"
@onready var lobby = $"."
@onready var player_list = $VBoxContainer2/Player_List


func _on_start_pressed():
	main.start_race()

func _on_quit_pressed():
	main.quit_lobby()
	main.exit_game()

func update_player_list(input):
	var temp_str = ""
	for n in input.size():
		temp_str += "%s \n" % str(input[n])
	player_list.text = temp_str
