extern crate std;

use cell_position::CellPosition;


#[derive(Debug, Clone)]
pub struct Piece
{
	pub cells: Vec<CellPosition>
}


pub struct PuzzleConfiguration
{
	pub width: i32,
	pub height: i32,
	pub pieces: Vec<Piece>,
	pub cell_to_piece_matrix: Vec<Vec<i32>>,
	pub goal: CellPosition
}


impl PuzzleConfiguration
{
	pub fn new_from_string(puzzle_str: &str) -> Option<PuzzleConfiguration>
	{	
		/* set up the resulting puzzle */
		let mut result = PuzzleConfiguration
		{
			width: 0,
			height: 0,
			pieces: Vec::new(),
			cell_to_piece_matrix: Vec::new(),
			goal: CellPosition{ x: -1, y: -1 }
		};
		
		
		let mut x_current = 0;
		
		let mut cells = Vec::new();
		cells.push(Vec::new());
		result.height = 1;
		
		/* fill in matrix with block colors */
		for c in puzzle_str.chars()
		{
			if c >= 'A' && c <= 'Z'
			{
				cells[result.height as usize - 1].push(c as i8 - 'A' as i8);
				x_current += 1;
			}
			else if c == '.'
			{
				cells[result.height as usize - 1].push(-1);
				x_current += 1;
			}
			else if c == '*'
			{
				result.goal = CellPosition{ x: x_current - 1, y: result.height - 2 };
			}
			else if c == '\n'
			{
				if x_current > result.width { result.width = x_current; }
				x_current = 0;
				
				cells.push(Vec::new());
				result.height += 1;
			}
		}
		
		
		/* trim bottom space */
		for y in (cells.len() - 1)..0
		{
			let mut is_empty = true;
			for block in cells[y].iter()
				{ is_empty = is_empty || *block == -1; }
			if is_empty
			{
				result.height -= 1;
				cells.pop();
			}
		}
		
		
		/* find connected blocks */
		for row in 0..cells.len()
		{
			for column in 0..cells[row].len()
			{
				let block_current = cells[row][column];
				if block_current == -1 { continue; }
				
				let mut piece = Piece { cells: Vec::new() };
			
				let mut blocks_to_check = Vec::new();
				blocks_to_check.push(CellPosition{ x: column as i32, y: row as i32 });
				
				let mut check_index = 0;
				while check_index < blocks_to_check.len()
				{
					let pos = blocks_to_check[check_index].clone();
					if pos.y >= 0 && pos.y < cells.len() as i32
					{				
						if pos.x >= 0 && pos.x < cells[pos.y as usize].len() as i32
						{
							if cells[pos.y as usize][pos.x as usize] == block_current
							{
								piece.cells.push(pos);
								cells[pos.y as usize][pos.x as usize] = -1;
								blocks_to_check.push(CellPosition{ x: pos.x - 1, y: pos.y });
								blocks_to_check.push(CellPosition{ x: pos.x + 1, y: pos.y });
								blocks_to_check.push(CellPosition{ x: pos.x, y: pos.y - 1 });
								blocks_to_check.push(CellPosition{ x: pos.x, y: pos.y + 1 });
							}
						}
					}
					
					check_index += 1;
				}
				
				result.pieces.push(piece);
			}
		}
		
		
		/* prepare cell matrix with empty piece indices */
		for _ in 0..result.height
		{
			let mut row = Vec::with_capacity(result.width as usize);
			for _ in 0..result.width
			{
				row.push(-1);
			}
			result.cell_to_piece_matrix.push(row);
		}
		
		
		/* fill in cell matrix with piece indices */
		for piece_index in 0..result.pieces.len()
		{
			for cell_pos in result.pieces[piece_index].cells.iter()
			{
				result.cell_to_piece_matrix[cell_pos.y as usize][cell_pos.x as usize] = piece_index as i32;
			}
		}
		
		
		return Some(result);
	}
}