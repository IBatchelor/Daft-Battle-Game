//Map Tests module
use engine::cartography_engine::tile::Tile;
use engine::cartography_engine::map::Coordinates;
use engine::cartography_engine::map::Location;
use engine::cartography_engine::map::Map;
use engine::movement::MovementDetails;
use engine::movement::movement_type::MovementType;
use engine::movement::MovementClass;
use engine::player::Player;
use engine::movement::terrain::Terrain;
use engine::movement::wheels::Wheels;
use engine::unit::Unit;

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

#[test]
fn check_unit_movement_when_movement_is_1_across_all_terrain_returns_diamond_of_appropriate_size(){
	//unit has 2 movement points
	//start unit on tile 3-3 of 5x5 map
	//expect tiles (1,3),(2,2),(2,3),(2,4),(3,1),(3,2),(3,3),(3,4),(3,5),(4,2),(4,3),(4,4),(5,3)
	
}

#[test]
fn check_unit_movement_when_movement_is_2_across_all_terrain_returns_diamond_of_appropriate_size(){
//2 movement points
//start on 3-3
//expect (2,3),(3,2),(3,3),(3,4),(4,3)
}

#[test]
fn check_unit_movement_when_movement_cost_higher_than_points_returns_current_tile(){
	//2 movement points, 3 movement cost, expect (3,3)
}

#[test]
fn check_unit_movement_when_movement_prohibited_returns_current_tile(){
	//unit surrounded by impassable terrain, expect (3,3)
}

#[test]
fn check_unit_movement_calculates_correct_shape_for_general_terrain(){
	//1,1,2,2,1 -> y,y,y,n,n
	//2,1,1,2,1 -> y,y,y,y,y
	//1,1,x,2,1 -> y,y,y,y,y
	//2,2,3,3,1 -> y,y,y,n,y
	//1,1,1,1,1 -> n,y,y,n,n
	//unit has 4 movement points
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

struct MovementTestUnit {
	movement_points : i32
}

impl Unit for MovementTestUnit {
	fn get_movement_details<Wheels>(&self) -> MovementDetails<Wheels> {
		MovementDetails{distance : self.movement_points, details : Wheels{}}
	}
}