extends Control

var peer = ENetMultiplayerPeer.new()
@export var player_scene : PackedScene

@onready var ip_text_edit = $MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/IpTextEdit
@onready var port_text_edit = $MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/PortTextEdit

const PORT = 2522
const _IP = "127.0.0.1"

func hide_ui():
	hide()

func show_ui():
	show()
