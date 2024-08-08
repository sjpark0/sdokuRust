mod solver;
mod naive_solver;
mod naive_solver1;
mod fast_solver;

pub use crate::solver::*;
pub use crate::naive_solver::*;
pub use crate::naive_solver1::*;
pub use crate::fast_solver::*;

use std::time::Instant;


fn main() {
    let mut sdoku : [usize ; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
    //let mut sdokuOriginal : [i32 ; NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y];
    sdoku[1] = 9;
	sdoku[5] = 6;
	sdoku[8] = 5;
	sdoku[10] = 3;
	sdoku[12] = 4;
	sdoku[13] = 5;
	sdoku[16] = 8;
	sdoku[18] = 4;
	sdoku[23] = 2;
	sdoku[32] = 4;
	sdoku[36] = 3;
	sdoku[39] = 7;
	sdoku[40] = 9;
	sdoku[44] = 2;
	sdoku[46] = 8;
	sdoku[51] = 1;
	sdoku[54] = 7;
	sdoku[57] = 5;
	sdoku[58] = 3;
	sdoku[62] = 9;
	sdoku[67] = 6;
	sdoku[74] = 9;
	sdoku[79] = 2;

    let sdoku_original: [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = sdoku.clone();
	
	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : naive_solver1::NaiveSolver1 = NaiveSolver1 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Naive Solver1 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : naive_solver::NaiveSolver = NaiveSolver { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Naive Solver Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : naive_solver1::NaiveSolver1 = NaiveSolver1 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Naive Solver1 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : fast_solver::FastSolver = FastSolver { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Fast Solver Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);
	
    
}
