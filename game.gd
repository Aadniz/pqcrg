extends Node

var peer = ENetMultiplayerPeer.new()
@export var player_scene : PackedScene
@onready var ui = $UI
@onready var ip_text_edit = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/IpTextEdit
@onready var port_text_edit = $UI/MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/PortTextEdit

const PORT = 2522
const _IP = "127.0.0.1"

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
		port = PORT
	if (ip == ""):
		ip = _IP
	peer.create_client(ip, int(port))
	multiplayer.multiplayer_peer = peer
	ui.hide()


func _on_host_button_pressed():
	var port = port_text_edit.get_line(0)
	if (port == ""):
		port = PORT
	peer.create_server(int(port))
	multiplayer.multiplayer_peer = peer
	# physically spawn a player
	multiplayer.peer_connected.connect(add_player)
	# Spawn itself
	add_player()
	ui.hide()

func add_player(id=1):
	var player = player_scene.instantiate()
	player.name = str(id)
	call_deferred("add_child", player)

