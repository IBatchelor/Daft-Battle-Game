use engine::player::Player;
use engine::unit::Unit;

pub enum Terrain {
	Plains,
	Woodland,
	City(Player),
	Fortress(Player),
	Road
}

pub struct Tile<'a> {
	terrain: Terrain,
	unit: Option<&'a Unit>,
	capture_points: i8
}

impl<'a> Tile<'a>{
	fn defence_points(&self, unit_player: &Player) -> i8{
		match self.terrain {
			Terrain::Plains => 1,
			Terrain::Woodland => 2,
			Terrain::City(ref player) => 3,
			Terrain::Fortress(ref player) => {
				if player == unit_player {
					5
				} else {
					2
				}
			},
			_ => 0,
		}
	}
}