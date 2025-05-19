extends Button
class_name HotbarButton

var index: int;

signal index_selected(index: int);

func _pressed() -> void:
	self.index_selected.emit(self.index);
