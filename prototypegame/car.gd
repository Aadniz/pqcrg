extends VehicleBody3D

@onready var camera_pivot = $camera_pivot
@onready var camera_3d = $camera_pivot/Camera3D

const MAX_STEER = 0.8
const ENGINE_POWER = 300

var look_at

func _ready():
	look_at = global_position

func _physics_process(delta):
	steering = move_toward(steering, Input.get_axis("right","left")*MAX_STEER,delta*2.5)
	engine_force = Input.get_axis("backward","forward") * ENGINE_POWER
	camera_pivot.global_position = camera_pivot.global_position.lerp(global_position, delta*20.0)
	camera_pivot.transform = camera_pivot.transform.interpolate_with(transform, delta *5.0)
	look_at = look_at.lerp(global_position + linear_velocity, delta * 5.0)
	camera_3d.look_at(look_at)
	var speed = get_linear_velocity()
	Globals.speed = speed
