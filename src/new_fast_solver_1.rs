use std::io::empty;

pub use crate::solver::*;

pub struct NewFastSolver_1{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NewFastSolver_1{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<(usize, usize, usize)> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push((j, i, (j / NUM_X) + (i / NUM_Y) * NUM_Y));
                }
            }
        }
        self.solve_sdoku(sdoku, &mut empty_list, solve_list)
    }

    
}

impl NewFastSolver_1{
    fn get_available_numbers_1(&self, y : usize, x : usize, group : usize, original_empty_list : &Vec<(usize, usize, usize, Vec<usize>)>, assign_list : &Vec<(usize, usize, usize, usize)>) -> Vec<usize>{
        let mut res : Vec<usize> = Vec::new();
        let mut num_list : [usize ; NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y];
        
        for elem in original_empty_list.iter(){
            if elem.0 == x && elem.1 == y {
                for elem2 in elem.3.iter(){
                    num_list[elem2 - 1] = 1;
                }
                break;
            }
        }
        for elem in assign_list.iter() {
            if elem.0 == x {
                num_list[elem.3-1] = 0;
            } else if elem.1 == y {
                num_list[elem.3-1] = 0;
            } else if elem.2 == group {
                num_list[elem.3-1] = 0;
            }
        }
    
        
        for i in 0..NUM_X * NUM_Y {
            if num_list[i] > 0 {
                res.push(i+1);
            }
        }
        
        return res;
    }
    
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<(usize, usize, usize)>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut original_empty_list : Vec<(usize, usize, usize, Vec<usize>)> = Vec::new();
        
        for elem in empty_list.iter(){
            original_empty_list.push((elem.0, elem.1, elem.2, self.get_available_numbers(sdoku, elem.1, elem.0)));
        }
        let mut assign_list : Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut assign_list_list : Vec<Vec<(usize, usize, usize, usize)>> = Vec::new();
        let result = self.solve_sdoku_r(&mut original_empty_list, empty_list, &mut assign_list, &mut assign_list_list);
        
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        for elem in assign_list_list[0].iter(){
            sdoku_temp[elem.0 + elem.1 * NUM_X * NUM_Y] = elem.3;
        }
        solve_list.push(sdoku_temp);
        return result;
    }
    
    fn solve_sdoku_r(&self, original_empty_list : &mut Vec<(usize, usize, usize, Vec<usize>)>, empty_list : &mut Vec<(usize, usize, usize)>, assign_list : &mut Vec<(usize, usize, usize, usize)>, assign_list_list : &mut Vec<Vec<(usize, usize, usize, usize)>>) -> i32{
        
        let mut empty_list_temp: Vec<(usize, usize, usize)> = empty_list.clone();
        let mut assign_list_temp: Vec<(usize, usize, usize, usize)> = assign_list.clone();

        let mut pos = 0;
        while pos < empty_list_temp.len(){
            let available_list = self.get_available_numbers_1(empty_list_temp[pos].1, empty_list_temp[pos].0, empty_list_temp[pos].2, &original_empty_list, &assign_list_temp);
            if available_list.len() == 0{
                return 0;
            }
            if available_list.len() == 1{
                assign_list_temp.push((empty_list_temp[pos].0, empty_list_temp[pos].1, empty_list_temp[pos].2, available_list[0]));
                empty_list_temp.remove(pos);
                pos = 0;
            } else{
                pos += 1;
            }
        }

        if empty_list_temp.len() == 0{
            assign_list_list.push(assign_list_temp);
            return 1;
        }

        let mut result : i32 = 0;
        let available_list = self.get_available_numbers_1(empty_list_temp[0].1, empty_list_temp[0].0, empty_list_temp[0].2, &original_empty_list, &assign_list_temp);    
        
        assign_list_temp.push((empty_list_temp[0].0, empty_list_temp[0].1, empty_list_temp[0].2, 0));
        empty_list_temp.remove(0);
        
        let length = assign_list_temp.len();

        for (_, elem) in available_list.iter().enumerate(){
            assign_list_temp[length - 1].3 = *elem;
            let temp_result = self.solve_sdoku_r(original_empty_list, &mut empty_list_temp, &mut assign_list_temp, assign_list_list);
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;
        }
        return result;
    }

}