extends CanvasLayer
var menu= get_tree()

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_start_pressed():
	$".".hide


func _on_quit_pressed():
	menu.show()
