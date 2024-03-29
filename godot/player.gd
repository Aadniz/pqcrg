extends VehicleBody3D

@onready var camera_3d = $Node3D/SpringArm3D/Camera3D
@onready var main = $"../"


const MAX_STEER = 0.8
const ENGINE_POWER = 300
var paused = false
var respawn_rotation = Vector3.ZERO
var respawn_momentum = Vector3.ZERO

# Called when the node enters the scene tree for the first time.
func _ready():
	#position += Vector3(RandomNumberGenerator.new().randf_range(-10.0, 10.0),0,0)
	camera_3d.current = is_multiplayer_authority()

func _enter_tree():
	set_multiplayer_authority(name.to_int())

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if Input.is_action_just_pressed("respawn"):
		respawn()
	if Input.is_action_just_pressed("menu"):
		main.pause_menu()

func _physics_process(delta):
	if !is_multiplayer_authority():
		return
	steering = move_toward(steering, Input.get_axis("right","left")*MAX_STEER,delta*2.5)
	engine_force = Input.get_axis("backward","forward") * ENGINE_POWER
	var speed = get_linear_velocity()
	var temp_speed = sqrt(speed.z*speed.z + speed.x*speed.x)
	var speed_string = "Speed: %f" % temp_speed
	#$CanvasLayer/speedometer.text = speed_string

func respawn():
	rotation = respawn_rotation
	linear_velocity = respawn_momentum
	position = $respawn_point.position
