extern crate pushmo;

fn main()
{
	let puzzle_str = "D*DD.\n D.E.\n D.EB\n DEEA";
	pushmo::solve(puzzle_str);
}