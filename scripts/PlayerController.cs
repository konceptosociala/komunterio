using Godot;

public partial class PlayerController : CharacterBody3D
{
	[Export] public float Speed = 5.0f;
	[Export] public float MouseSensitivity = 0.002f;
	[Export] public float JumpVelocity = 4.5f;

	private Camera3D camera;
	private Vector2 lookRotation = Vector2.Zero;
	private float gravity = ProjectSettings.GetSetting("physics/3d/default_gravity").AsSingle();

	public override void _Ready()
	{
		camera = GetNode<Camera3D>("Camera3D");
		Input.MouseMode = Input.MouseModeEnum.Captured;
	}

	public override void _Input(InputEvent @event)
	{
		if (@event is InputEventMouseMotion motion)
		{
			lookRotation.X -= motion.Relative.Y * MouseSensitivity;
			lookRotation.Y -= motion.Relative.X * MouseSensitivity;
			lookRotation.X = Mathf.Clamp(lookRotation.X, -Mathf.Pi / 2, Mathf.Pi / 2);
			Rotation = new Vector3(0, lookRotation.Y, 0);
			camera.Rotation = new Vector3(lookRotation.X, 0, 0);
		}
	}

	public override void _PhysicsProcess(double delta)
	{
		Vector3 velocity = Velocity;

		// Gravity
		if (!IsOnFloor())
			velocity.Y -= gravity * (float)delta;

		// Movement
		Vector2 inputDir = Input.GetVector("move_left", "move_right", "move_forward", "move_back");
		Vector3 direction = (Transform.Basis * new Vector3(inputDir.X, 0, inputDir.Y)).Normalized();
		velocity.X = direction.X * Speed;
		velocity.Z = direction.Z * Speed;

		// Jump
		if (Input.IsActionJustPressed("jump") && IsOnFloor())
			velocity.Y = JumpVelocity;

		Velocity = velocity;
		MoveAndSlide();
	}
}
