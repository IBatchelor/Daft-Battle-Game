use engine::movement::MovementDetails;
use engine::movement::MovementClass;

pub trait Unit{
	fn get_movement_details<T : MovementClass>(&self) -> MovementDetails<T>;
}

pub enum UnitType{
	Hovercraft, //Not in the original game, but who cares? This is MINE
	TestUnit1,
	TestUnit2
}
