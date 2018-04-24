//Map Tests module

#[test]
fn move_down_increments_the_current_tile_y_value_by_1(){
	let map = create_basic_map();
	let orig_x = map.current_coordinates.x;
	let orig_y = map.current_coordinates.y;
	map.move_down();
	let expected_y = orig_y + 1;
	assert_eq!(map.current_coordinates.x, orig_x);
	assert_eq!(map.current_coordinates.y, expected_y);
}

#[test]
fn move_up_decrements_the_current_tile_y_value_by_1(){
	let map = create_basic_map();
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
	let map = create_basic_map();
	let orig_x = map.current_coordinates.x;
	let orig_y = map.current_coordinates.y;
	let expected_x = orig_x + 1;
	map.move_right();
	assert_eq!(map.current_coordinates.x, expected_x);
	assert_eq! (map.current_coordinates.y, orig_y);
}

#[test]
fn move_left_decrements_the_current_tile_x_value_by_1(){
	let_map = create_basic_map();
	map.current_coordinates = Coordinates{x : 2, y : 1};
	let orig_x = map.current_coordinates.x;
	let orig_y = map.current_coordinates.y;
	let expected_x = orig_x - 1;
	map.move_left();
	assert_eq!(map.current_coordinates.x, expected_x);
	assert_eq!(map.current_coordinates.y, orig_y);
}

fn create_basic_map() -> Map {
	let players = [&Player::Blue, &Player::Green];
	let tiles = {
		Location{coordinates:Coordinates{1,1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{1,2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{1,3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{1,4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}}, 
		Location{coordinates:Coordinates{1,5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{2,1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{2,2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{2,3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}}, 
		Location{coordinates:Coordinates{2,4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{2,5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{3,1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{3,2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{3,3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{3,4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{3,5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{4,1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{4,2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{4,3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{4,4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{4,5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{5,1},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{5,2},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{5,3},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{5,4},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		Location{coordinates:Coordinates{5,5},tile:Tile{terrain:Terrain::Plains, unit:None, capture_points:0}},
		};
	
	Map::New(players, tiles);
}