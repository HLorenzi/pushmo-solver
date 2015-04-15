extern crate std;

use cell_position::CellPosition;
use player_movement::PlayerMovement;
use puzzle_configuration::PuzzleConfiguration;


struct MoveUndo
{
	player_position: CellPosition,
	piece_index: i32,
	pull_out: bool
}


pub struct PuzzleSolver<'a>
{
	pub config: &'a PuzzleConfiguration,

	pub player_position: CellPosition,
	pub piece_pull_levels: Vec<i8>,
	
	/* unused; needs Hash trait below */
	/* pub already_checked_set: std::collections::HashSet<PuzzleSolver<'a>>, */
	
	pub solve_max_depth: i32,
	pub solve_movement_tests: i32
}


impl<'a> PartialEq for PuzzleSolver<'a>
{
	fn eq(&self, other: &PuzzleSolver) -> bool
	{
		/* should optimally only take into account
		the player's normalized/cannonical/reachability position */
		if self.player_position.x != other.player_position.x ||
			self.player_position.y != other.player_position.y
			{ return false; }
		
		for i in 0..self.piece_pull_levels.len()
		{
			if self.piece_pull_levels[i] != other.piece_pull_levels[i]
				{ return false; }
		}
		return true;
	}
}


impl<'a> Eq for PuzzleSolver<'a>
{

}


	/* state.write_* unstable at Rust 1.0? */
/*impl<'a> std::hash::Hash for PuzzleSolver<'a>
{
	fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher
	{
		/* should optimally only take into account
		the player's normalized/cannonical/reachability position */
		
		state.write_i32(self.player_position.x);
		state.write_i32(self.player_position.y);
		
		for i in 0..self.piece_pull_levels.len()
		{
			state.write_i8(self.piece_pull_levels[i]);
		}
	}
}*/


impl<'a> PuzzleSolver<'a>
{
	pub fn new(config: &'a PuzzleConfiguration) -> PuzzleSolver<'a>
	{
		let mut p = PuzzleSolver
		{
			config: config,
			
			player_position: CellPosition{ x: 0, y: config.height - 1 },
			piece_pull_levels: Vec::new(),
			
			/* unused; needs the Hash trait above */
			/* already_checked_set: std::collections::HashSet::new(), */
			
			solve_max_depth: 15,
			solve_movement_tests: 0
		};
		
		/* zero out every piece's pull level */
		for _ in 0..config.pieces.len()
			{ p.piece_pull_levels.push(0); }
			
		return p;
	}


	pub fn apply_move(&mut self, m: &PlayerMovement) -> MoveUndo
	{
		let undo = MoveUndo { player_position: m.player_position, piece_index: m.piece_index, pull_out: m.pull_out };
	
		/* move player */
		self.player_position = m.player_position;
		/* push/pull piece */
		self.piece_pull_levels[m.piece_index as usize] +=
			if m.pull_out { 1 } else { -1 };
			
		return undo;
	}
	
	
	pub fn undo_move(&mut self, m: &MoveUndo)
	{
		/* unmove player */
		self.player_position = m.player_position;
		/* unpush/unpull piece */
		self.piece_pull_levels[m.piece_index as usize] -=
			if m.pull_out { 1 } else { -1 };
	}
	
	
	pub fn position_has_foothold(&self, pos: CellPosition) -> bool
	{
		/* if at ground level... */
		if pos.y == self.config.height - 1 && pos.x >= 0 && pos.x < self.config.width
			{ return true; }
		/* else, check for appropriate blocks */
		else
			{ return self.get_pull_level_at(pos) < self.get_pull_level_at(pos.add(0, 1)); }
	}
	
	
	pub fn get_piece_at(&self, pos: CellPosition) -> i32
	{
		/* if out of puzzle bounds... */
		if pos.x < 0 || pos.x >= self.config.width || pos.y < 0 || pos.y >= self.config.height
			{ -1 }
		/* else, get piece index from config's matrix */
		else
			{ self.config.cell_to_piece_matrix[pos.y as usize][pos.x as usize] }
	}
	
	
	pub fn get_pull_level_at(&self, pos: CellPosition) -> i8
	{
		/* if at ground level... */
		if pos.y == self.config.height && pos.x >= 0 && pos.x < self.config.width
			{ 4 }
		/* if out of puzzle bounds... */
		else if pos.x < 0 || pos.x >= self.config.width || pos.y < 0 || pos.y >= self.config.height
			{ 0 }
		/* if inside puzzle bounds... */
		else
		{
			let piece = self.config.cell_to_piece_matrix[pos.y as usize][pos.x as usize];
			/* a hole */
			if piece == -1
				{ 0 }
			/* a piece */
			else
				{ self.piece_pull_levels[piece as usize] }
		}
	}
	
	
	pub fn get_reachable_at(&self, outvec: &mut Vec<CellPosition>, pos: CellPosition)
	{
		/* test for moving left */
		if self.position_has_foothold(pos.add(-1, 0))
			{ outvec.push(pos.add(-1, 0)); }
			
		/* test for moving right */
		if self.position_has_foothold(pos.add(1, 0))
			{ outvec.push(pos.add(1, 0)); }
			
		/* test for jumping atop a block at the same horizontal position */
		if self.position_has_foothold(pos.add(0, -1)) /* should also test for ceilings */
			{ outvec.push(pos.add(0, -1)); }
			
		/* test for jumping atop a block to the left */
		if self.position_has_foothold(pos.add(-1, -1)) /* should also test for ceilings */
			{ outvec.push(pos.add(-1, -1)); }
			
		/* test for jumping atop a block to the right */
		if self.position_has_foothold(pos.add(1, -1)) /* should also test for ceilings */
			{ outvec.push(pos.add(1, -1)); }
	}
	
	
	pub fn get_reachable_recursive(&self, pos: CellPosition) -> Vec<CellPosition>
	{
		let mut to_check = Vec::new();
		to_check.push(pos);
		
		let mut reachable_temp_vec = Vec::new();
		
		let mut check_index = 0;
		while check_index < to_check.len()
		{
			self.get_reachable_at(&mut reachable_temp_vec, to_check[check_index]);
			
			for r in reachable_temp_vec.iter()
			{
				let mut duplicate = false;
				for p in to_check.iter()
				{
					if r.x == p.x && r.y == p.y
						{ duplicate = true; break; }
				}
			
				if !duplicate
					{ to_check.push(*r); }
			}
			
			reachable_temp_vec.clear();
			check_index += 1;
		}
		
		return to_check;
	}
	
	
	pub fn get_potential_moves_at(&self, outvec: &mut Vec<PlayerMovement>, pos: CellPosition)
	{
		let pull_level_at_pos = self.get_pull_level_at(pos);
		let pull_level_under = self.get_pull_level_at(pos.add(0, 1));
		
		/* test for pulling right in front */
		if self.get_piece_at(pos) != -1
		{
			if pull_level_at_pos < pull_level_under - 1
				{ outvec.push(PlayerMovement{ player_position: pos, piece_index: self.get_piece_at(pos), pull_out: true, score: 0f32 }); }
		}
		
		/* test for pulling sideways */
		if pull_level_under - pull_level_at_pos >= 2
		{
			/* right side */
			if self.get_piece_at(pos.add(1, 0)) != -1
			{
				let pull_level_at_side = self.get_pull_level_at(pos.add(1, 0));
				
				if pull_level_at_side > 0 && pull_level_at_side < 3
					{ outvec.push(PlayerMovement{ player_position: pos, piece_index: self.get_piece_at(pos.add(1, 0)), pull_out: true, score: 0f32 }); }
			}
			
			/* left side */
			if self.get_piece_at(pos.add(-1, 0)) != -1
			{
				let pull_level_at_side = self.get_pull_level_at(pos.add(-1, 0));
				
				if pull_level_at_side > 0 && pull_level_at_side < 3
					{ outvec.push(PlayerMovement{ player_position: pos, piece_index: self.get_piece_at(pos.add(-1, 0)), pull_out: true, score: 0f32 }); }
			}
		}
	}
	
	
	pub fn rate_after_move(&self, m: &PlayerMovement) -> f32
	{
		/* create a heuristic score for the state after the given move is applied */
		/* the higher value, the better and closer to the goal */
		let mut result = 0f32;
		result -= (self.config.goal.x - m.player_position.x).abs() as f32;
		result -= (self.config.goal.y - m.player_position.y).abs() as f32;
		return result;
	}
	
	
	pub fn solve(&mut self) -> Option<Vec<PlayerMovement>>
	{
		let mut moves_temp = Vec::new();
		return self.solve_search(0, &mut moves_temp);
	}
	
	
	fn solve_search(&mut self, depth: i32, mut moves_temp: &mut Vec<Vec<PlayerMovement>>) -> Option<Vec<PlayerMovement>>
	{
		if depth >= self.solve_max_depth
			{ return None; }
			
		if depth as usize >= moves_temp.len()
			{ moves_temp.push(Vec::new()); }
			
		/* gather potential moves */
		let reach = self.get_reachable_recursive(self.player_position);		
		moves_temp[depth as usize].clear();
		
		for r in reach
		{
			if r.x == self.config.goal.x && r.y == self.config.goal.y
				{ return Some(Vec::new()); }
		
			self.get_potential_moves_at(&mut moves_temp[depth as usize], r);
		}
		
		/* sort by heuristic */
		for m in moves_temp[depth as usize].iter_mut()
			{ m.score = self.rate_after_move(m); }
		
		moves_temp[depth as usize].sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
		
			
		/* search deeper */
		for m_index in 0..moves_temp[depth as usize].len()
		{
			self.solve_movement_tests += 1;
			
			let undo = self.apply_move(&moves_temp[depth as usize][m_index]);
			let solution = self.solve_search(depth + 1, &mut moves_temp);
			self.undo_move(&undo);
			
			if let Some(mut solution_vec) = solution
			{
				solution_vec.insert(0, moves_temp[depth as usize][m_index]);
				return Some(solution_vec);
			}
		}
		
		return None;
	}
	
	
	pub fn print(&self)
	{
		for y in -1..self.config.height
		{
			for x in 0..self.config.width
			{
				print!("{} ", self.get_pull_level_at(CellPosition{ x: x, y: y }));
			}
			println!("");
			for x in 0..self.config.width
			{
				if self.player_position.x == x && self.player_position.y == y
					{ print!("@"); }
				else if self.config.goal.x == x && self.config.goal.y == y
					{ print!("*"); }
				else
					{ print!(" "); }
					
				print!(" ");
			}
			println!("");
		}
	}
}