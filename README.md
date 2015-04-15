# pushmo-solver

Solves simple Pushmo levels using a depth-first search with depth cap and heuristic sorting. Should probably use iterative deepening, a better heuristic, and a hash set for keeping track of already-expanded moves. Currently only checks very basic jumping rules. It should be easy to add new mechanics by editing PuzzleSolver's get_reachable_at() and get_potential_moves_at().
