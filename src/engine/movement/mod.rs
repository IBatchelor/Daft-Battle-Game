//engine::movement
use engine::movement::terrain::Terrain;
use engine::movement::movement_type::MovementType;

#[cfg(test)]
pub mod movement_tests;
pub mod movement_type;
pub mod terrain;
pub mod wheels;

pub trait MovementClass {
	fn get_type(&self) -> MovementType;
	fn get_cost(&self, terrain : Terrain) -> Option<i32>;
}

pub struct MovementDetails<T : MovementClass>{
	pub distance: i32,
	pub details: T
}