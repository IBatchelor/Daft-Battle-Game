//Tile Tests module
use engine::cartography_engine::tile::Tile;
use engine::cartography_engine::tile::Terrain;
use engine::player::Player;
use engine::unit::Unit;

#[test]
fn defence_points_for_plains_returns_one(){
	let terrain = Terrain::Plains;
	let tile = build_tile(terrain, None);
	
	assert_eq!(tile.defence_points(&Player::Red), 1);
}

#[test]
fn defence_points_for_woodland_returns_two(){
	let terrain = Terrain::Woodland;
	let tile = build_tile(terrain, None);
	
	assert_eq!(tile.defence_points(&Player::Red), 2);
}

#[test]
fn defence_points_for_city_returns_three(){
	let city_player = Player::Red;
	let terrain = Terrain::City(&city_player);
	let tile = build_tile(terrain, None);
	
	assert_eq!(tile.defence_points(&city_player), 3);
	assert_eq!(tile.defence_points(&Player::Blue), 3);
}

#[test]
fn defence_points_for_fortress_returns_two_if_different_player(){
	let terrain = Terrain::Fortress(&Player::Green);
	let tile = build_tile(terrain, None);
	
	assert_eq!(tile.defence_points(&Player::Yellow), 2);
}

#[test]
fn defence_points_for_fortress_returns_five_if_same_player(){
	let player = Player::Black;
	let terrain = Terrain::Fortress(&player);
	let tile = build_tile(terrain, None);
	
	assert_eq!(tile.defence_points(&player), 5);
}

#[test]
fn defence_points_for_road_returns_zero(){
	let terrain = Terrain::Road;
	let tile = build_tile(terrain, None);
	
	assert_eq!(tile.defence_points(&Player::Blue), 0);
}

fn build_tile<'a>(terrain: Terrain<'a>, unit: Option<&'a Unit>) -> Tile<'a>{
	Tile{
		terrain,
		unit,
		capture_points: 20,
	}
}