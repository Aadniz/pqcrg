extends Node

var player_list = []
var checkpoint_list = []
var checkpoints = {}
var peer = ENetMultiplayerPeer.new()
@export var player_scene : PackedScene
@onready var ui = $UI
@onready var lobby = $Lobby
@onready var ip_text_edit = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/IpTextEdit
@onready var port_text_edit = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/PortTextEdit
@onready var pqc_toggle_checkbox = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer2/CheckBox
@onready var pqc = Pqc.new()

const DEFAULT_PORT = 2522
const DEFAULT_PQC_PORT = 3522
const DEFAULT_IP = "127.0.0.1"

# Called when the node enters the scene tree for the first time.
func _ready():
	var args = OS.get_cmdline_args()
	var options = {
		"--port": DEFAULT_PORT,
		"--server": false
	}
	
	for i in range(args.size()):
		if args[i] in options:
			if typeof(options[args[i]]) == TYPE_BOOL:
				options[args[i]] = true
			elif i + 1 < args.size():
				options[args[i]] = int(args[i + 1])
	
	if options["--server"]:
		host_game(options["--port"])


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_join_button_pressed():
	var ip = ip_text_edit.get_line(0)
	var port = port_text_edit.get_line(0)
	if (port == ""):
		if pqc_toggle_checkbox.button_pressed == true:
			port = DEFAULT_PQC_PORT
		else:
			port = DEFAULT_PORT
	if (ip == ""):
		ip = DEFAULT_IP
	if pqc_toggle_checkbox.button_pressed == true:
		pqc.start_client_bridge(ip, int(port))
		peer.create_client("127.0.0.1", 3522)
	else:
		peer.create_client(ip, int(port))
	multiplayer.multiplayer_peer = peer
	ui.hide()
	lobby.show()


func _on_host_button_pressed():
	var port = port_text_edit.get_line(0)
	if (port == ""):
		port = DEFAULT_PORT
	host_game(int(port))
	# Spawn itself
	add_player()
	ui.hide()
	lobby.show()


func host_game(port: int):
	if pqc_toggle_checkbox.button_pressed == true:
		pqc.start_host_bridge(port)
	peer.create_server(port)
	multiplayer.multiplayer_peer = peer
	# physically spawn a player
	multiplayer.peer_connected.connect(add_player)


func add_player(id=1):
	var player = player_scene.instantiate()
	player.name = str(id)
	player_list.append(player.name.to_int())
	print(player_list)
	call_deferred("add_child", player)

func exit_game():
	del_player(peer.get_unique_id())

func del_player(id):
	rpc("_disconnect_2", id)
	rpc("_del_player", id)

@rpc("any_peer","call_local") func _del_player(id):
	get_node(str(id)).queue_free()
	multiplayer.disconnect_peer(id)
	player_list.erase(id)
	print(player_list)

func start_race():
	lobby.hide()

func quit_lobby():
	ui.show()
	lobby.hide()

func main_menu():
	ui.show()


func _on_check_box_toggled(toggled_on):
	if toggled_on:
		port_text_edit.placeholder_text = str(DEFAULT_PQC_PORT)
	else:
		port_text_edit.placeholder_text = str(DEFAULT_PORT)
		
func add_checkpoint(id):
	print("checkpoint added %d" % id)
	checkpoint_list.append(id)
	print(checkpoint_list)

func activate_checkpoint(checkpoint, player):
	print("car %d" % player + "drove though checkpoint %d" %  checkpoint)
	checkpoints[[player,checkpoint]] = true
	print(checkpoints)

func check_checkpoints(player):
	for n in range(checkpoint_list.size()):
		var all = false
		if (checkpoints.has([player,checkpoint_list[n]])):
			all = true
		else:
			all = false
		if (all):
			print("All checkpoints")

