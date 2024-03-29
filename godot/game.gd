extends Node

var peer = ENetMultiplayerPeer.new()
@export var player_scene : PackedScene
@onready var ui = $UI
@onready var ip_text_edit = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/IpTextEdit
@onready var port_text_edit = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/PortTextEdit
@onready var pqc = Pqc.new()
@onready var lobby = $Lobby

const DEFAULT_PORT = 2522
const DEFAULT_IP = "127.0.0.1"

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


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
	peer.create_client(ip, int(port))
	multiplayer.multiplayer_peer = peer
	ui.hide()
	lobby.show()


func _on_host_button_pressed():
	var port = port_text_edit.get_line(0)
	if (port == ""):
		port = DEFAULT_PORT
	pqc.start_host_bridge(int(port))
	peer.create_server(int(port))
	multiplayer.multiplayer_peer = peer
	# physically spawn a player
	multiplayer.peer_connected.connect(add_player)
	# Spawn itself
	add_player()
	ui.hide()
	lobby.show()

func add_player(id=1):
	var player = player_scene.instantiate()
	player.name = str(id)
	call_deferred("add_child", player)

