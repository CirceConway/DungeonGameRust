extends CharacterBody3D

@onready var camera = $Camera3D

const SPEED = 15.0
const JUMP_VELOCITY = 4.5

# Get the gravity from the project settings to be synced with RigidBody nodes.
var gravity = 0
#ProjectSettings.get_setting("physics/3d/default_gravity")

var mouseSense = 0.3

func _input(event):
	if event is InputEventMouseMotion:
		var rotx = ((-event.relative.y) * mouseSense)
		var crotx = rad_to_deg(camera.rotation.x)
		if (rotx > 0 and crotx <= 90) or (rotx < 0 and crotx >= -90):
			camera.rotate_x(deg_to_rad(rotx))
		var roty = deg_to_rad((-event.relative.x) * mouseSense)
		rotate_y(roty)
		
	if event.is_action_pressed("ui_cancel"):
		Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
	if event.is_action_pressed("ui_accept"):
		if Input.mouse_mode == Input.MOUSE_MODE_VISIBLE:
			Input.mouse_mode = Input.MOUSE_MODE_CAPTURED

func _physics_process(delta):
	# Add the gravity.
	#if not is_on_floor():
		#velocity.y -= gravity * delta
	# Handle jump.
	#if Input.is_action_just_pressed("ui_accept") and is_on_floor():
		#velocity.y = JUMP_VELOCITY

	# Get the input direction and handle the movement/deceleration.
	# As good practice, you should replace UI actions with custom gameplay actions.
	var input_dir = Input.get_vector("left", "right", "forward", "backward")
	var vert = Input.get_axis("down", "up")
	var direction = (transform.basis * Vector3(input_dir.x, vert, input_dir.y)).normalized()
	if direction:
		velocity.x = direction.x * SPEED
		velocity.y = direction.y * SPEED
		velocity.z = direction.z * SPEED
	else:
		velocity.x = move_toward(velocity.x, 0, SPEED)
		velocity.y = move_toward(velocity.y, 0, SPEED)
		velocity.z = move_toward(velocity.z, 0, SPEED)

	move_and_slide()
