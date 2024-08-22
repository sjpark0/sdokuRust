pub use crate::solver::*;

pub struct NaiveSolver{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NaiveSolver{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<(usize, usize, usize, usize)> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push((j, i, (j / NUM_X) + (i / NUM_Y) * NUM_Y, 0 ));
                }
            }
        }

        self.solve_sdoku(sdoku, &mut empty_list, solve_list)
    }
}

impl NaiveSolver{
    fn solve_sdoku(&self, sdoku : &[usize], empty_list : &mut Vec<(usize, usize, usize, usize)>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        
        //self.print_emptylist(emptyList);
        //self.print_sdoku(&sdokuTemp);
        //self.print_sdoku(sdoku);

        if empty_list.len() == 0{
            solve_list.push(sdoku_temp);
            return 1;
        }
        let available_list = self.get_available_numbers(sdoku, empty_list[0].1, empty_list[0].0);
        if available_list.len() == 0{
            return 0;
        }
        
        let mut result = 0;
        //let mut emptylistTemp = (*emptyList[1..];
        let mut empty_list_temp : Vec<(usize, usize, usize, usize)> = Vec::new();
        for i in 1..empty_list.len(){
            empty_list_temp.push(empty_list[i].clone());
        }
        for (_, elem) in available_list.iter().enumerate(){
            sdoku_temp[empty_list[0].0 + empty_list[0].1 * NUM_X * NUM_Y] = *elem;
            let temp_result = self.solve_sdoku(&sdoku_temp, &mut empty_list_temp, solve_list);
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;
        }
        return result;
    }

}