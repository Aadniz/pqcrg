extends Control

var peer = ENetMultiplayerPeer.new()
@export var player_scene : PackedScene

@onready var ip_text_edit = $MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/IpTextEdit
@onready var port_text_edit = $MarginContainer/Panel/MarginContainer/VBoxContainer/HBoxContainer/PortTextEdit

const PORT = 2522
const _IP = "127.0.0.1"

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func hide_ui():
	hide()

