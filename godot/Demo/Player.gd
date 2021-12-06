extends KinematicBody2D

onready var target: Vector2 = position
onready var moving = false
var speed = 500

func _physics_process(delta):
	if moving:
		var collision = move_and_slide((target - position).normalized() * speed)
		if target.distance_to(position) <= 5: 
			moving = false
			$AnimatedSprite.animation = "idle"
		#position = position + (target - position).normalized() * speed * delta

		

func move_to(pos: Vector2):
	target = pos
	moving = true
	$AnimatedSprite.animation = "run"
	
	if target.x < position.x:
		$AnimatedSprite.flip_h = true
	else:
		$AnimatedSprite.flip_h = false
