use engine::player::Player;
use engine::movement::wheels::Wheels;
use engine::movement::MovementClass;
use engine::movement::MovementType;
use engine::movement::terrain::Terrain;

#[test]
fn get_type_returns_wheels(){
	let wheels = Wheels{};
	assert_eq!(wheels.get_type(), MovementType::Wheels);
}

#[test]
fn get_cost_for_road_returns_1(){
	let wheels = Wheels{};
	assert_eq!(wheels.get_cost(Terrain::Road), Some(1));
}

#[test]
fn get_cost_for_city_returns_1(){
	let wheels = Wheels{};
	assert_eq!(wheels.get_cost(Terrain::City(&Player::Red)), Some(1));
}

#[test]
fn get_cost_for_fortress_returns_1(){
	let wheels = Wheels{};
	assert_eq!(wheels.get_cost(Terrain::Fortress(&Player::Blue)), Some(1));
}

#[test]
fn get_cost_for_plains_returns_2(){
	let wheels = Wheels{};
	assert_eq!(wheels.get_cost(Terrain::Plains), Some(2));
}

#[test]
fn get_cost_for_woodland_returns_3(){
	let wheels = Wheels{};
	assert_eq!(wheels.get_cost(Terrain::Woodland), Some(3));
}

#[test]
fn get_cost_for_sea_returns_none(){
	let wheels = Wheels{};
	assert_eq!(wheels.get_cost(Terrain::Sea), None);
}