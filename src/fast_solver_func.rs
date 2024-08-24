use std::io::empty;

pub use crate::solver::*;

pub struct FastSolverFunc{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for FastSolverFunc{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<(usize, usize, usize, usize, usize)> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push((j, i, j + i * NUM_X * NUM_Y, (j / NUM_X) + (i / NUM_Y) * NUM_Y, 0 ));
                }
            }
        }
        
        self.solve_sdoku(sdoku, &mut empty_list.as_slice(), solve_list)
    }
}

impl FastSolverFunc{
    fn remove_trivial(&self, sdoku : &mut [usize], empty_list : &[(usize, usize, usize, usize, usize)], mut res : Vec<(usize, usize, usize, usize, usize)>) -> Option<(Vec<(usize, usize, usize, usize, usize)>, usize)>{
        if empty_list.len() == 0{ 
            let s = res.len();
            return Some((res, s)); 
        }
        let available_list = self.get_available_numbers(sdoku, empty_list[0].1, empty_list[0].0);
        match available_list.len() {
            0 => None,
            1 => {
                sdoku[empty_list[0].2] = available_list[0];
                res.extend_from_slice(&empty_list[1..]);
                self.remove_trivial(sdoku, &res, Vec::new())
            },
            _ => {
                //res.push(empty_list[0].clone());
                res.push(empty_list[0]);
                self.remove_trivial(sdoku, &empty_list[1..], res)
            }
        }
    }
    fn solve_sdoku(&self, sdoku : &[usize], empty_list : &[(usize, usize, usize, usize, usize)], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);

        let empty_list_temp = self.remove_trivial(&mut sdoku_temp, empty_list, Vec::new());
        
        match empty_list_temp  {
            Some((l, 0)) => {
                solve_list.push(sdoku_temp);
                1
            },
            Some((l, _)) => 
                self.get_available_numbers(&sdoku_temp, l[0].1, l[0].0).iter().fold(0, |b, avail| {
                    sdoku_temp[l[0].2] = *avail;
                    b + self.solve_sdoku(&sdoku_temp, &l.as_slice()[1..], solve_list)
                }),
            None => return 0,
        }
        
        
        
    }

}