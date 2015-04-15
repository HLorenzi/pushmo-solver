extern crate std;


#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct CellPosition
{
	pub x: i32,
	pub y: i32
}


impl CellPosition
{
	pub fn add(&self, x: i32, y: i32) -> CellPosition
	{
		return CellPosition{ x: self.x + x, y: self.y + y };
	}
}


impl std::fmt::Debug for CellPosition
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "({}, {})", self.x, self.y)
	}
}