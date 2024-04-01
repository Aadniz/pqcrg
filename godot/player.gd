extends VehicleBody3D

@onready var camera_3d = $Node3D/SpringArm3D/Camera3D
@onready var main = $"../"
@onready var speedometer = $Speedometer
@onready var pause_menu = $Control

const MAX_STEER = 0.8
const ENGINE_POWER = 300
var paused = false
var respawn_rotation = Vector3.ZERO
var respawn_momentum = Vector3.ZERO

# Called when the node enters the scene tree for the first time.
func _ready():
	#position += Vector3(RandomNumberGenerator.new().randf_range(-10.0, 10.0),0,0)
	camera_3d.current = is_multiplayer_authority()
	speedometer.show()
	

func _enter_tree():
	set_multiplayer_authority(name.to_int())

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if Input.is_action_just_pressed("respawn"):
		respawn()
	if Input.is_action_just_pressed("menu"):
		pause()

func _physics_process(delta):
	if !is_multiplayer_authority():
		return
	steering = move_toward(steering, Input.get_axis("right","left")*MAX_STEER,delta*2.5)
	engine_force = Input.get_axis("backward","forward") * ENGINE_POWER
	var speed = get_linear_velocity()
	var temp_speed = sqrt(speed.z*speed.z + speed.x*speed.x)
	var speed_string = "Speed: %d" % temp_speed
	speedometer.text = speed_string

func set_checkpoint(checkpoint_position, checkpoint_rotation):
	$respawn_point.position = checkpoint_position
	respawn_rotation = checkpoint_rotation
	
func respawn():
	rotation = respawn_rotation
	linear_velocity = respawn_momentum
	position = $respawn_point.position

func pause():
	if paused:
		pause_menu.hide()
	else:
		pause_menu.show()
	paused = !paused

func _on_resume_pressed():
	pause_menu.hide()
	paused = !paused

func _on_main_menu_pressed():
	main.main_menu()
	pause_menu.hide()


func _on_quit_pressed():
	main.exit_game(name.to_int())
	quit_game()

func quit_game():
	get_tree().quit()


