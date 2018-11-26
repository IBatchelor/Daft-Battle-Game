use engine::movement::terrain::Terrain;
use engine::movement::movement_type::MovementType;
use engine::movement::MovementClass;

struct Wheels{ //most likely will need to pub this in future
	
}

impl MovementClass for Wheels{
	fn get_type(&self) -> MovementType{ //ditto this
		MovementType::Wheels
	}
	
	fn get_cost(&self, terrain : Terrain) -> Option<i32>{ //ditto this
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