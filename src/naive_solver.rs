pub use crate::solver::*;

pub struct NaiveSolver{
    pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NaiveSolver{

}