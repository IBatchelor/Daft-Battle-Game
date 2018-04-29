use engine::player::Player;

#[derive(Clone)]
pub enum Terrain<'a> {
	Plains,
	Woodland,
	City(&'a Player),
	Fortress(&'a Player),
	Road,
	Sea
}