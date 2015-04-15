extern crate std;

use cell_position::CellPosition;


#[derive(Clone, Copy)]
pub struct PlayerMovement
{
	pub player_position: CellPosition,
	pub piece_index: i32,
	pub pull_out: bool,
	pub score: f32
}


impl std::fmt::Debug for PlayerMovement
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "player at {:?}, piece {}, pull? {}", self.player_position, self.piece_index, self.pull_out)
	}
}