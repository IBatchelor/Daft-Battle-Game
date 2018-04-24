use engine::player::Player;
use engine::unit::Unit;

#[derive(Clone)]
pub enum Terrain<'a> {
	Plains,
	Woodland,
	City(&'a Player),
	Fortress(&'a Player),
	Road
}

#[derive(Clone)]
pub struct Tile<'a> {
	pub terrain: Terrain<'a>,
	pub unit: Option<&'a Unit>,
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
