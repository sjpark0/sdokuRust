pub use crate::solver::*;

pub struct NaiveSolver1{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NaiveSolver1{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<COORD1> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push(COORD1 { x: j, y: i, group: (j / NUM_X) + (i / NUM_Y) * NUM_Y, val: 0 });
                }
            }
        }

        self.solve_sdoku(sdoku, &mut empty_list.as_slice(), solve_list)
    }
}

impl NaiveSolver1{
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &[COORD1], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        if empty_list.len() == 0{
            let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
            sdoku_temp.copy_from_slice(sdoku);
            solve_list.push(sdoku_temp);
            return 1;
        }
        let available_list = self.get_available_numbers(sdoku, empty_list[0].y, empty_list[0].x);
        if available_list.len() == 0{
            return 0;
        }
        
        let mut result = 0;
        
        for (_, elem) in available_list.iter().enumerate(){
            sdoku[empty_list[0].x + empty_list[0].y * NUM_X * NUM_Y] = *elem;
            let temp_result = self.solve_sdoku(sdoku, &empty_list[1..], solve_list);
            sdoku[empty_list[0].x + empty_list[0].y * NUM_X * NUM_Y] = 0;
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;
        }
        return result;
    }

}