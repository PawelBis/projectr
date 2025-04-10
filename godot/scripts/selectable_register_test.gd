extends Node
var t: float

func _ready() -> void:
	SelectionSignalBus.selectable_entered_tree.connect(_on_selectable_spawned)

func _process(delta: float) -> void:
	t += delta
	if t > 10.0:
		var selectable = Selectable.new()
		add_child(selectable)
		queue_free()


func _on_selectable_spawned(node: Node) -> void:
	print("Selectable spawned: ", node)
