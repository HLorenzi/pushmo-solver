mod cell_position;
mod player_movement;
mod puzzle_configuration;
mod puzzle_solver;

#[cfg(test)]
use cell_position::CellPosition;

use player_movement::PlayerMovement;
use puzzle_configuration::PuzzleConfiguration;
use puzzle_solver::PuzzleSolver;


pub fn solve(puzzle_str: &str) -> Option<Vec<PlayerMovement>>
{
	let puzzle_config = PuzzleConfiguration::new_from_string(puzzle_str).unwrap();
	let mut puzzle_solver = PuzzleSolver::new(&puzzle_config);
	
	let solution = puzzle_solver.solve();
	
	match solution
	{
		None =>
		{
			println!("#### Could not find a solution. Solver checked {} moves. ####", puzzle_solver.solve_movement_tests);
			return None;
		}
		
		Some(steps) =>
		{
			println!("======= Initial Configuration =======");
			puzzle_solver.print();
			println!("");
			
			for step_index in 0..steps.len()
			{
				puzzle_solver.apply_move(&steps[step_index]);
				println!("======= Step {} =======", step_index + 1);
				puzzle_solver.print();
				println!("");
			}
			
			println!("======= Solved in {} steps. Solver checked {} moves. =======", steps.len(), puzzle_solver.solve_movement_tests);
		}
	}
	
	return None
}


#[test]
fn test_has_foothold()
{
	let puzzle_config = PuzzleConfiguration::new_from_string("AA*\nBB").unwrap();
	let puzzle_solver = PuzzleSolver::new(&puzzle_config);
	
	assert_eq!(puzzle_solver.position_has_foothold(CellPosition{ x: 0, y: 1 }), true);
	assert_eq!(puzzle_solver.position_has_foothold(CellPosition{ x: 1, y: 1 }), true);
	assert_eq!(puzzle_solver.position_has_foothold(CellPosition{ x: 0, y: 0 }), false);
	assert_eq!(puzzle_solver.position_has_foothold(CellPosition{ x: 1, y: 0 }), false);
}

#[test]
fn test_get_pull_level_at()
{
	let puzzle_config = PuzzleConfiguration::new_from_string("AA*\nBB").unwrap();
	let puzzle_solver = PuzzleSolver::new(&puzzle_config);
	
	assert_eq!(puzzle_solver.get_pull_level_at(CellPosition{ x: 0, y: 1 }), 0);
	assert_eq!(puzzle_solver.get_pull_level_at(CellPosition{ x: 1, y: 1 }), 0);
	assert_eq!(puzzle_solver.get_pull_level_at(CellPosition{ x: 0, y: 0 }), 0);
	assert_eq!(puzzle_solver.get_pull_level_at(CellPosition{ x: 1, y: 0 }), 0);
}