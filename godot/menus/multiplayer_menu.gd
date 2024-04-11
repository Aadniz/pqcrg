extends Control

@onready var main = $"../"
signal server_disconnected
var peer = ENetMultiplayerPeer.new()
@onready var name_text_edit = $MarginContainer/Panel/MarginContainer/VBoxContainer/NameTextEdit

const DEFAULT_PORT = 2522
const DEFAULT_PQC_PORT = 3522
const DEFAULT_IP = "127.0.0.1"
var no_pqc = false
@onready var ip_text_edit = $MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/IpTextEdit
@onready var port_text_edit = $MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/PortTextEdit
@onready var pqc_toggle_checkbox = $MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer2/CheckBox
@onready var pqc = Pqc.new()


# Called when the node enters the scene tree for the first time.
func _ready():
	multiplayer.peer_connected.connect(peer_connected)
	multiplayer.peer_disconnected.connect(peer_disconnected)
	multiplayer.connected_to_server.connect(connected_to_server)
	multiplayer.connection_failed.connect(connection_failed)
	multiplayer.server_disconnected.connect(_on_server_disconnected)
	var args = OS.get_cmdline_args()
	var options = {
		"--port": DEFAULT_PORT,
		"--server": false,
		"--no-pqc": false
	}
	
	for i in range(args.size()):
		if args[i] in options:
			if typeof(options[args[i]]) == TYPE_BOOL:
				options[args[i]] = true
			elif i + 1 < args.size():
				options[args[i]] = int(args[i + 1])
	
	if options["--server"]:
		host_game(options["--port"])
	
	if options["--no-pqc"]:
		no_pqc = false

	
func _on_server_disconnected():
	multiplayer.multiplayer_peer = null
	server_disconnected.emit()
	GameManager.Players.clear()
	main.main_menu()

func peer_connected(id):
	print("Player Connected " + str(id))
	
	
# this get called on the server and clients
func peer_disconnected(id):
	print("Player Disconnected " + str(id))
	if(GameManager.game_state and GameManager.Players[id].has_car):
		main.del_player.rpc(id)
	GameManager.Players.erase(id)
	main.update_lobby()
	
# called only from clients
func connected_to_server():
	print("connected To Sever!")
	SendPlayerInformation.rpc_id(1, name_text_edit.text, multiplayer.get_unique_id())
	

@rpc("any_peer")
func sync_game_state(gamestate):
	GameManager.game_state = gamestate
	print(GameManager.game_state)

@rpc("any_peer")
func SendPlayerInformation(player_name, id):
	if !GameManager.Players.has(id):
		GameManager.Players[id] ={
			"name" : player_name,
			"id" : id,
			"finished": false,
			"has_car": false
		}
	
	if multiplayer.is_server():
		for i in GameManager.Players:
			SendPlayerInformation.rpc(GameManager.Players[i].name, i)
	main.update_lobby()

func _on_check_box_toggled(toggled_on):
	if toggled_on:
		port_text_edit.placeholder_text = str(DEFAULT_PQC_PORT)
	else:
		port_text_edit.placeholder_text = str(DEFAULT_PORT)
		

# called only from clients
func connection_failed():
	print("Couldnt Connect")


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
	hide()
	main.show_lobby()


func _on_host_button_pressed():
	var port = port_text_edit.get_line(0)
	if (port == ""):
		port = DEFAULT_PORT
	host_game(int(port))
	SendPlayerInformation(name_text_edit.text, multiplayer.get_unique_id())
	hide()
	main.show_lobby()


func host_game(port: int):
	if pqc_toggle_checkbox.button_pressed == true and !no_pqc:
		pqc.start_host_bridge(port)
	peer.create_server(port)
	multiplayer.multiplayer_peer = peer
