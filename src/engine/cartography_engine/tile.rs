use engine::player::Player;
use engine::movement::terrain::Terrain;
use engine::movement::MovementClass;

#[derive(Clone)]
pub struct Tile<'a > {
	pub terrain: Terrain<'a>,
	pub unit: Option<&'a MovementClass>,
	pub capture_points: i8
}

impl<'a> Tile<'a>{
	pub fn defence_points(&self, unit_player: &Player) -> i8{
		match self.terrain {
			Terrain::Plains => 1,
			Terrain::Woodland => 2,
			Terrain::City(ref player) => 3,
			Terrain::Fortress(ref player) => {
				if *player == unit_player {
					5
				} else {
					2
				}
			},
			_ => 0,
		}
	}
}
