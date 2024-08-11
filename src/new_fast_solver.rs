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
        /*while !empty_list.is_empty(){
            empty_list.swap_remove(0);
            println!("{}", empty_list.len());
        }*/
        self.solve_sdoku(sdoku, &mut empty_list, solve_list)
    }
}

impl NewFastSolver{
    fn solve_sdoku(&self, sdoku : &[usize], empty_list : &mut Vec<COORD1>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        
        //let mut empty_list_temp : Vec<COORD1> = Vec::with_capacity(empty_list.len());
        //&empty_list_temp.clone_from(empty_list);
        let mut empty_list_temp: Vec<COORD1> = Vec::new();
        for i in 0..empty_list.len(){
            empty_list_temp.push(COORD1 { x: empty_list[i].x, y: empty_list[i].y, group: empty_list[i].group, val: empty_list[i].val });
        }
        

        let mut index = 0;
        while index < empty_list_temp.len(){
            let available_list = self.get_available_numbers(&sdoku_temp, empty_list_temp[index].y, empty_list_temp[index].x);
            if available_list.len() == 0{
                return 0;
            }
            if available_list.len() == 1{
                sdoku_temp[empty_list_temp[index].x + empty_list_temp[index].y * NUM_X * NUM_Y] = available_list[0];
                empty_list_temp.swap_remove(index);
                index = 0;
            } else{
                index += 1;
            }
        }

        if empty_list_temp.len() == 0{
            solve_list.push(sdoku_temp);
            return 1;
        }
        let tmp = COORD1{ x : empty_list_temp[0].x, y : empty_list_temp[0].y, group : empty_list_temp[0].group, val : empty_list_temp[0].val};
        //empty_list_temp[0].clone();
        empty_list_temp.swap_remove(0);

        let available_list = self.get_available_numbers(sdoku, tmp.y, tmp.x);
        if available_list.len() == 0{
            return 0;
        }
        
        let mut result = 0;
        //let mut emptylistTemp = (*emptyList[1..];
        
        for (_, elem) in available_list.iter().enumerate(){
            sdoku_temp[tmp.x + tmp.y * NUM_X * NUM_Y] = *elem;
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