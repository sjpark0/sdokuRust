pub use crate::solver::*;

pub struct NewFastSolver{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NewFastSolver{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<COORD1> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push(COORD1 { x: j, y: i, group: (j / NUM_X) + (i / NUM_Y) * NUM_Y, val: 0 });
                }
            }
        }
        self.solve_sdoku(sdoku, &mut empty_list, solve_list)
    }

    
}

impl NewFastSolver{
    fn get_available_numbers_1(&self, y : usize, x : usize, group : usize, original_empty_list : &Vec<COORD2>, assign_list : &Vec<COORD1>) -> Vec<usize>{
        let mut res : Vec<usize> = Vec::new();
        let mut num_list : [usize ; NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y];
        
        for elem in original_empty_list.iter(){
            if elem.x == x && elem.y == y {
                for elem2 in elem.available_list.iter(){
                    num_list[elem2 - 1] = 1;
                }
                break;
            }
        }
        for elem in assign_list.iter() {
            if elem.x == x {
                num_list[elem.val-1] = 0;
            } else if elem.y == y {
                num_list[elem.val-1] = 0;
            } else if elem.group == group {
                num_list[elem.val-1] = 0;
            }
        }
    
        
        for i in 0..NUM_X * NUM_Y {
            if num_list[i] > 0 {
                res.push(i+1);
            }
        }
        
        return res;
    }
    
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<COORD1>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut original_empty_list : Vec<COORD2> = Vec::new();
        
        for elem in empty_list.iter(){
            original_empty_list.push(COORD2 { x: elem.x, y: elem.y, group: elem.group, val: elem.val, available_list: self.get_available_numbers(sdoku, elem.y, elem.x) });
        }
        let mut assign_list : Vec<COORD1> = Vec::new();
        let mut assign_list_list : Vec<Vec<COORD1>> = Vec::new();
        let result = self.solve_sdoku_r(&mut original_empty_list, empty_list, &mut assign_list, &mut assign_list_list);
        
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        for elem in assign_list_list[0].iter(){
            sdoku_temp[elem.x + elem.y * NUM_X * NUM_Y] = elem.val;
        }
        solve_list.push(sdoku_temp);
        return result;
    }
    
    fn solve_sdoku_r(&self, original_empty_list : &mut Vec<COORD2>, empty_list : &mut Vec<COORD1>, assign_list : &mut Vec<COORD1>, assign_list_list : &mut Vec<Vec<COORD1>>) -> i32{
        
        let mut empty_list_temp: Vec<COORD1> = Vec::new();
        let mut assign_list_temp: Vec<COORD1> = Vec::new();

        for elem in empty_list.iter(){
            empty_list_temp.push(COORD1 { x: elem.x, y: elem.y, group: elem.group, val: elem.val });
        }
        for elem in assign_list.iter(){
            assign_list_temp.push(COORD1 { x: elem.x, y: elem.y, group: elem.group, val: elem.val });
        }
        

        let mut pos = 0;
        while pos < empty_list_temp.len(){
            let available_list = self.get_available_numbers_1(empty_list_temp[pos].y, empty_list_temp[pos].x, empty_list_temp[pos].group, &original_empty_list, &assign_list_temp);
            if available_list.len() == 0{
                return 0;
            }
            if available_list.len() == 1{
                assign_list_temp.push(COORD1 { x: empty_list_temp[pos].x, y: empty_list_temp[pos].y, group: empty_list_temp[pos].group, val: available_list[0] });
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
        let available_list = self.get_available_numbers_1(empty_list_temp[0].y, empty_list_temp[0].x, empty_list_temp[0].group, &original_empty_list, &assign_list_temp);    
        
        assign_list_temp.push(COORD1 { x: empty_list_temp[0].x, y: empty_list_temp[0].y, group: empty_list_temp[0].group, val: empty_list_temp[0].val });
        empty_list_temp.remove(0);
        
        let length = assign_list_temp.len();

        for (_, elem) in available_list.iter().enumerate(){
            assign_list_temp[length - 1].val = *elem;
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