tool
extends Node2D

export(int, 283) var item setget set_item


func set_item(to):
	item = to
	$AnimatedSprite.frame = to
