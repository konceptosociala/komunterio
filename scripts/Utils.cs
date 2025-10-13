using Godot;
using System;

public partial class Utils : Node
{	
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		ProcessMode = ProcessModeEnum.Always;
	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
		if (Input.IsActionJustPressed("pause"))
		{
			if (GetTree().Paused)
			{
				GetTree().Paused = false;
				Input.MouseMode = Input.MouseModeEnum.Captured;
			}
			else
			{
				GetTree().Paused = true;
				Input.MouseMode = Input.MouseModeEnum.Visible;
			}
		}
	}
}
