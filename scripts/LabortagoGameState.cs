using Godot;
using System;

public enum LevelState
{
	DREAM,
	APARTMENT,
	TOWN,
	FACTORY,
	OFFICE,
	FIRE_ESCAPE
}

public partial class LabortagoGameState : Node
{
	[Export] public LevelState CurrentLevel = LevelState.DREAM;
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
	}
}
