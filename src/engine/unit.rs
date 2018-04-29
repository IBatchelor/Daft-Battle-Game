use engine::movement::MovementDetails;

pub trait Unit{
	fn get_movement_details(&self) -> MovementDetails;
}

pub enum UnitType{
	Hovercraft, //Not in the original game, but who cares? This is MINE
	TestUnit1,
	TestUnit2
}
