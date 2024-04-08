extends Node

var checkpoint_list = []
var checkpoints = {}

var startpoint = [Vector3.ZERO,Vector3.ZERO]
@export var player_scene : PackedScene
@onready var ui = $UI
@onready var lobby = $Lobby
@onready var pause_menu = $Pause_Menu
@onready var race_finished = $Race_Finished
var paused = false

# Called when the node enters the scene tree for the first time.
func _ready():
	pass
	

func add_player(id=1):
	var player = player_scene.instantiate()
	player.name = str(id)
	call_deferred("add_child", player)

func disconnect_player():
	multiplayer.multiplayer_peer.close()

func start_race():
	pass

func main_menu():
	ui.show()

func show_lobby():
	lobby.show()


@rpc("any_peer")
func player_finished(id):
	GameManager.Players[id].finished = true

func update_lobby():
	lobby.update_player_list()

func add_checkpoint(id):
	checkpoint_list.append(id)

func activate_checkpoint(checkpoint, player):
	#print("car %d" % player + "drove though checkpoint %d" %  checkpoint)
	checkpoints[[player,checkpoint]] = true
	

func check_checkpoints(player):
	var all = false
	for n in range(checkpoint_list.size()):
		if (checkpoints.has([player,checkpoint_list[n]])):
			all = true
		else:
			all = false
			break
	if (all):
		player_finished.rpc(player)
		if multiplayer.get_unique_id() == player:
			race_finished.show()


func set_start(position, rotation):
	startpoint[0] = position
	startpoint[1] = rotation

func get_startpoint():
	return startpoint

func pause():
	if paused:
		pause_menu.hide()
	else:
		pause_menu.show()
	paused = !paused
