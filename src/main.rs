mod solver;
mod naive_solver;
mod naive_solver_0;
mod naive_solver_func;
mod fast_solver;
mod fast_solver_func;
mod fast_solver_0;
mod fast_solver3;
mod new_fast_solver;
mod new_fast_solver1;
mod new_fast_solver2;
mod new_fast_solver2_1;
mod new_fast_solver2_2;
mod new_fast_solver3;
mod new_fast_solver3_0;

pub use crate::solver::*;
pub use crate::naive_solver::*;
pub use crate::naive_solver_0::*;
pub use crate::naive_solver_func::*;
pub use crate::fast_solver::*;
pub use crate::fast_solver_func::*;
pub use crate::fast_solver_0::*;
pub use crate::fast_solver3::*;
pub use crate::new_fast_solver::*;
pub use crate::new_fast_solver1::*;
pub use crate::new_fast_solver2::*;
pub use crate::new_fast_solver2_1::*;
pub use crate::new_fast_solver2_2::*;
pub use crate::new_fast_solver3::*;
pub use crate::new_fast_solver3_0::*;


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
    let solver : naive_solver_0::NaiveSolver_0 = NaiveSolver_0 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Naive Solver_0 Elapsed time: {:?}", elapsed_time);
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
    let solver : naive_solver_0::NaiveSolver_0 = NaiveSolver_0 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Naive Solver_0 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : naive_solver_func::NaiveSolverFunc = NaiveSolverFunc { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Naive Solver Func Elapsed time: {:?}", elapsed_time);
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

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : fast_solver_0::FastSolver_0 = FastSolver_0 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Fast Solver_0 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : fast_solver_func::FastSolverFunc = FastSolverFunc { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Fast Solver Func Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : fast_solver3::FastSolver3 = FastSolver3 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Fast3 Solver Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : new_fast_solver::NewFastSolver = NewFastSolver { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("New Fast Solver Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);
	
	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : new_fast_solver1::NewFastSolver1 = NewFastSolver1 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("New Fast Solver 1 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);


	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : new_fast_solver2::NewFastSolver2 = NewFastSolver2 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("New Fast Solver 2 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : new_fast_solver2_1::NewFastSolver2_1 = NewFastSolver2_1 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("New Fast Solver 2_1 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : new_fast_solver2_2::NewFastSolver2_2 = NewFastSolver2_2 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("New Fast Solver 2_2 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : new_fast_solver3::NewFastSolver3 = NewFastSolver3 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("New Fast Solver 3 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

	let mut solve_list : Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]> = Vec::new();
    let solver : new_fast_solver3_0::NewFastSolver3_0 = NewFastSolver3_0 { /*m_solver : Vec::new()*/ };
    sdoku = sdoku_original.clone();
	let start_time = Instant::now();
	solver.solve_sdoku(&mut sdoku, &mut solve_list);
	let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("New Fast Solver 3_0 Elapsed time: {:?}", elapsed_time);
	solver.print_sdoku(&solve_list[0]);

}
