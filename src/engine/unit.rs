pub trait Unit{
	fn get_movement_details(&self) -> UnitMovementDetails;
}

pub struct UnitMovementDetails {
	pub distance: i32,
	pub details: MovementDetails
}

pub struct MovementDetails {
	pub movement_type: MovementType
}

pub enum MovementType{
	Wheels,
	Treads,
	Foot,
	Air,
	Hovercraft,
}

pub enum UnitType{
	Hovercraft, //Not in the original game, but who cares? This is MINE
	TestUnit1,
	TestUnit2
}
