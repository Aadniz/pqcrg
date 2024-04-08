extends VehicleBody3D

@onready var camera_3d = $Node3D/SpringArm3D/Camera3D
@onready var main = $"../../"
@onready var speedometer = $Speedometer
@onready var multiplayer_synchronizer = $MultiplayerSynchronizer


const MAX_STEER = 0.5
const ENGINE_POWER = 300
var respawn_rotation = Vector3.ZERO
var respawn_momentum = Vector3.ZERO

# Called when the node enters the scene tree for the first time.
func _ready():
	#position += Vector3(RandomNumberGenerator.new().randf_range(-10.0, 10.0),0,0)
	multiplayer_synchronizer.set_multiplayer_authority(name.to_int())
	camera_3d.current = multiplayer_synchronizer.is_multiplayer_authority()
	speedometer.show()
	var spawn = main.get_startpoint()
	$respawn_point.position = spawn[0]
	respawn_rotation = spawn[1]
	respawn()

func _enter_tree():
	pass

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	if Input.is_action_just_pressed("respawn"):
		respawn()
	if Input.is_action_just_pressed("menu"):
		options_menu()

func _physics_process(delta):
	if !multiplayer_synchronizer.is_multiplayer_authority():
		return
	steering = move_toward(steering, Input.get_axis("right","left")*MAX_STEER,delta*2.5)
	engine_force = Input.get_axis("backward","forward") * ENGINE_POWER
	var speed = get_linear_velocity()
	var temp_speed = sqrt(speed.z*speed.z + speed.x*speed.x)
	var speed_string = "Speed: %d" % temp_speed
	speedometer.text = speed_string
	camera_3d.fov = lerp(camera_3d.fov, 80 + sqrt(temp_speed/100) * 60, 0.1)

func set_checkpoint(checkpoint_position, checkpoint_rotation):
	$respawn_point.position = checkpoint_position
	respawn_rotation = checkpoint_rotation
	
func respawn():
	rotation = respawn_rotation
	linear_velocity = respawn_momentum
	position = $respawn_point.position

func options_menu():
	if multiplayer_synchronizer.is_multiplayer_authority():
		main.pause()

func _on_testbutton_pressed():
	main.check_checkpoints(name.to_int())
