//Cartography Engine Map Module
use std::collections::HashMap;
use engine::cartography_engine::tile::Tile;
use engine::player::Player;



pub struct Map<'a> {
	pub players: &'a [Player],
	pub map : HashMap<Coordinates, Tile<'a>>,
	pub current_coordinates : Coordinates
}

impl <'b> Map<'b>{
	pub fn init<'g>(players : &'g [Player], tiles : &'g [Location]) -> Map<'g> {
		let map : HashMap<Coordinates, Tile> = tiles.iter().map(|tile| (tile.coordinates.clone(), tile.tile.clone())).collect();
		let coordinates = Coordinates {x : 1, y : 1};
		
		Map{players : players, map : map, current_coordinates : coordinates}
	}
	
	pub fn move_right(&mut self) {
		self.current_coordinates.x += 1;
	}
	
	pub fn move_left(&mut self) {
		self.current_coordinates.x -= 1;
	}
	
	pub fn move_up(&mut self){
		self.current_coordinates.y -= 1;
	}
	
	pub fn move_down(&mut self){
		self.current_coordinates.y += 1;
	}
	
	pub fn check_unit_movement(&mut self){
	    //Want to make this return list of tiles or co-ordinates where unit can move to
		//If no unit on tile, presumably return NONE and let GUI/Engine deal with that
		//If unit on tile, return SOME(tile[] or coordinates[])
		//
		//So, What do I need to do?
		//See which tiles adjacent to the unit can be moved into (and for each one calculate what the remaining movement is)
		//See which tiles adjacent to those (non-checked) the remaining movement can move to (and take the one with the highest remaining movement)
		//Repeat until get to layer where none are viable
		//Return the lot
		
	}
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Coordinates {
	pub x : i32,
	pub y : i32
}

pub struct Location<'a> {
	pub coordinates : Coordinates,
	pub tile : Tile<'a>
}

