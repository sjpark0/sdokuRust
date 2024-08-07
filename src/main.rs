mod solver;
mod naive_solver;

pub use crate::solver::*;
pub use crate::naive_solver::*;


fn main() {
    let mut sdoku : [i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
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

    let sdoku_original: [i32; NUM_X * NUM_Y * NUM_X * NUM_Y] = sdoku.clone();

    let solver : naive_solver::NaiveSolver = NaiveSolver { m_solver : Vec::new() };
    solver.print_sdoku(&sdoku);
    solver.print_sdoku(&sdoku_original);
    
}
