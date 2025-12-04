using Godot;
using System;

public partial class Utils : Node
{	

	[Export] 
	public Label fpsLabel;

	public override void _Ready()
	{
		ProcessMode = ProcessModeEnum.Always;
	}

	public override void _Process(double delta)
	{		
		if (fpsLabel != null)
		{
			fpsLabel.Text = $"FPS: {Engine.GetFramesPerSecond()}";
		}

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
