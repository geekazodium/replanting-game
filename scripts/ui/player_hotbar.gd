extends HBoxContainer
class_name PlayerHotbar

var hotbar_buttons: Array[HotbarButton] = [];
var hotbar_keys: Array[StringName] = [];
@export var button_scene: PackedScene = null;

func _ready() -> void:
	EventBus.player_hotbar_updated.connect(self.on_hotbar_updated);

func on_hotbar_updated(items: Array[StringName]) -> void:
	self.resize_display(items.size());
	self.hotbar_keys = items;
	for i in range(items.size()):
		var plant_name: StringName = items[i];
		self.hotbar_buttons[i].text = plant_name;

func resize_display(new_slot_count: int) -> void:
	var current_size: int = self.hotbar_buttons.size();
	if current_size > new_slot_count:
		for i in range(new_slot_count,current_size):
			self.hotbar_buttons[i].queue_free();
		self.hotbar_buttons = self.hotbar_buttons.slice(0,new_slot_count);
	elif current_size < new_slot_count:
		for i in range(current_size,new_slot_count):
			var new_instance: HotbarButton = self.button_scene.instantiate() as HotbarButton;
			self.hotbar_buttons.append(new_instance);
			self.add_child(new_instance);
			new_instance.index_selected.connect(self.select);
			new_instance.index = i;

func select(index: int) -> void:
	EventBus.player_hotbar_item_selected.emit(self.hotbar_keys[index]);
	print("hotbar index ", index, " selected");
