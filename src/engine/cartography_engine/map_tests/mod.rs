//Map Tests module
use engine::cartography_engine::tile::Tile;
use engine::cartography_engine::tile::Terrain;
use engine::cartography_engine::map::Coordinates;
use engine::cartography_engine::map::Location;
use engine::cartography_engine::map::Map;
use engine::player::Player;

#[test]
fn move_down_increments_the_current_tile_y_value_by_1(){
	let mut map = create_basic_map();
	let orig_x = map.current_coordinates.x;
	let orig_y = map.current_coordinates.y;
	map.move_down();
	let expected_y = orig_y + 1;
	assert_eq!(map.current_coordinates.x, orig_x);
	assert_eq!(map.current_coordinates.y, expected_y);
}

#[test]
fn move_up_decrements_the_current_tile_y_value_by_1(){
	let mut map = create_basic_map();
	map.current_coordinates = Coordinates{x : 1, y : 2};
	let orig_x = map.current_coordinates.x;
	let orig_y = map.current_coordinates.y;
	let expected_y = orig_y - 1;
	map.move_up();
	assert_eq!(map.current_coordinates.x, orig_x);
	assert_eq!(map.current_coordinates.y, expected_y);
}

#[test]
fn move_right_increments_the_current_tile_x_value_by_1(){
	let mut map = create_basic_map();
	let orig_x = map.current_coordinates.x;
	let orig_y = map.current_coordinates.y;
	let expected_x = orig_x + 1;
	map.move_right();
	assert_eq!(map.current_coordinates.x, expected_x);
	assert_eq! (map.current_coordinates.y, orig_y);
}

#[test]
fn move_left_decrements_the_current_tile_x_value_by_1(){
	let mut map = create_basic_map();
	map.current_coordinates = Coordinates{x : 2, y : 1};
	let orig_x = map.current_coordinates.x;
	let orig_y = map.current_coordinates.y;
	let expected_x = orig_x - 1;
	map.move_left();
	assert_eq!(map.current_coordinates.x, expected_x);
	assert_eq!(map.current_coordinates.y, orig_y);
}

fn create_basic_map<'a>() -> Map<'a> {
	let players = &[Player::Blue, Player::Green][..];
	let tiles = &[
		Location{coordinates:Coordinates{x : 1,y : 1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 1,y : 2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 1,y : 3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 1,y : 4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}}, 
		Location{coordinates:Coordinates{x : 1,y : 5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 2,y : 1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 2,y : 2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 2,y : 3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}}, 
		Location{coordinates:Coordinates{x : 2,y : 4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 2,y : 5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 3,y : 1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 3,y : 2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 3,y : 3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 3,y : 4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 3,y : 5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 4,y : 1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 4,y : 2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 4,y : 3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 4,y : 4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 4,y : 5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 5,y : 1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 5,y : 2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 5,y : 3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 5,y : 4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{x : 5,y : 5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		][..];
	
	Map::init(players, tiles)
}