use engine::movement::terrain::Terrain;
use engine::movement::movement_type::MovementType;
use engine::movement::MovementClass;

pub struct Wheels{
	
}

impl MovementClass for Wheels{
	pub fn get_type(&self) -> MovementType{
		MovementType::Wheels
	}
	
	pub fn get_cost(&self, terrain : Terrain) -> Option<i32>{
		match terrain {
			Terrain::City(ref player) => Some(1),
			Terrain::Road => Some(1),
			Terrain::Fortress(ref player) => Some(1),
			Terrain::Plains => Some(2),
			Terrain::Woodland => Some(3),
			_ => None,			
		}
	}
}