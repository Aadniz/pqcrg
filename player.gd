extends VehicleBody3D

const MAX_STEER = 0.8
const ENGINE_POWER = 300
var paused = false
var respawn_rotation = Vector3.ZERO
var respawn_momentum = Vector3.ZERO

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func _physics_process(delta):
	steering = move_toward(steering, Input.get_axis("right","left")*MAX_STEER,delta*2.5)
	engine_force = Input.get_axis("backward","forward") * ENGINE_POWER
	var speed = get_linear_velocity()
	var temp_speed = sqrt(speed.z*speed.z + speed.x*speed.x)
	var speed_string = "Speed: %f" % temp_speed
	#$CanvasLayer/speedometer.text = speed_string
