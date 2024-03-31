extends Node

var player_list = []
var peer = ENetMultiplayerPeer.new()
@export var player_scene : PackedScene
@onready var ui = $UI
@onready var lobby = $Lobby
@onready var pause = $Pause_Menu
@onready var ip_text_edit = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/IpTextEdit
@onready var port_text_edit = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/PortTextEdit
@onready var pqc = Pqc.new()

const DEFAULT_PORT = 2522
const DEFAULT_IP = "192.168.111.202"

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
		port = DEFAULT_PORT
	if (ip == ""):
		ip = DEFAULT_IP
	pqc.start_client_bridge(ip, int(port))
	peer.create_client("127.0.0.1", 3522)
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
	pqc.start_host_bridge(port)
	peer.create_server(port)
	multiplayer.multiplayer_peer = peer
	# physically spawn a player
	multiplayer.peer_connected.connect(add_player)


func add_player(id=1):
	var player = player_scene.instantiate()
	player.name = str(id)
	player_list += [player.name]
	print(player_list)
	call_deferred("add_child", player)

func exit_game(id):
	multiplayer.peer_disconnected.connect(del_player)
	print(id)
	player_list.erase(id)
	print(player_list)
	del_player(id)

func del_player(id):
	rpc("_del_player", id)

@rpc("any_peer","call_local") func _del_player(id):
	get_node(str(id)).queue_free()

func start_race():
	lobby.hide()

func quit_lobby():
	ui.show()
	lobby.hide()

func pause_menu():
	pause.show()

func main_menu():
	ui.show()
