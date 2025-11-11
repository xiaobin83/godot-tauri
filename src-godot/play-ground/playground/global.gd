extends Node

func toggleConsole() -> void:
	JavaScriptBridge.eval("window.parent.postMessage('toggleConsole', '*');");

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("OpenConsole"):
		toggleConsole();
