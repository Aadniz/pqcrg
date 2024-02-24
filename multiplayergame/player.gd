extends VehicleBody3D

@onready var camera_3d = $cam_origin/SpringArm3D/Camera3D
@onready var main = $"../"

const MAX_STEER = 0.8
const ENGINE_POWER = 600
var paused = false

# Called when the node enters the scene tree for the first time.
func _ready():
	camera_3d.current = is_multiplayer_authority()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if Input.is_action_just_pressed("respawn"):
		rotation=Vector3.ZERO
		linear_velocity = Vector3.ZERO
		position = $respawn_point.position
	if Input.is_action_just_pressed("menu"):
		pauseMenu()
	
func _enter_tree():
	set_multiplayer_authority(name.to_int())

func _physics_process(delta):
	if is_multiplayer_authority():
		steering = move_toward(steering, Input.get_axis("right","left")*MAX_STEER,delta*2.5)
		engine_force = Input.get_axis("backward","forward") * ENGINE_POWER
		var speed = get_linear_velocity()
		var temp_speed = sqrt(speed.z*speed.z + speed.x*speed.x)
		var speed_string = "Speed: %f" % temp_speed
		$CanvasLayer/speedometer.text = speed_string
		
func pauseMenu():
	if paused:
		$pause_menu.hide()
	else:
		$pause_menu.show()
	paused = !paused
 
func set_checkpoint(test):
	$respawn_point.position = test
	
	
func _on_resume_pressed():
	$pause_menu.hide()
	paused = !paused

func _on_quit_pressed():
	main.exit_game(name.to_int())
	get_tree().quit()
