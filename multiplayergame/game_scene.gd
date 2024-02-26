extends Node3D


var peer = ENetMultiplayerPeer.new()
@export var player_scene : PackedScene


func _on_host_pressed():
	peer.create_server(7777)
	multiplayer.multiplayer_peer = peer
	multiplayer.peer_connected.connect(add_player)
	add_player()
	$CanvasLayer.hide()
	

func add_player(id=1):
	var player = player_scene.instantiate()
	player.name = str(id)
	call_deferred("add_child",player)

func exit_game(id):
	multiplayer.peer_disconnected.connect(del_player)
	del_player(id)

func del_player(id):
	rpc("_del_player", id)

@rpc("any_peer","call_local") func _del_player(id):
	get_node(str(id)).queue_free()


func _on_join_pressed():
	var ip = "127.0.0.1"
	ip = $CanvasLayer/ip_input.get_line(0)
	peer.create_client(ip, 7777)
	multiplayer.multiplayer_peer = peer
	$CanvasLayer.hide()
