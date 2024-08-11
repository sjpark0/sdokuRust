pub use crate::solver::*;

pub struct FastSolver_0{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for FastSolver_0{
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

impl FastSolver_0{
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<COORD1>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        
        let mut assign_list : Vec<COORD1> = Vec::new();
        let mut index = 0;
        while index < empty_list.len(){
            let available_list = self.get_available_numbers(sdoku, empty_list[index].y, empty_list[index].x);
            if available_list.len() == 0{
                for elem in assign_list.iter(){
                    empty_list.push(COORD1 { x: elem.x, y: elem.y, group: elem.group, val: elem.val });
                    sdoku[elem.x + elem.y * NUM_X * NUM_Y] = 0;
                }
                return 0;
            }
            if available_list.len() == 1{
                assign_list.push(COORD1 { x: empty_list[index].x, y: empty_list[index].y, group: empty_list[index].group, val: empty_list[index].val });
                sdoku[empty_list[index].x + empty_list[index].y * NUM_X * NUM_Y] = available_list[0];
                empty_list.remove(index);
                
                index = 0;
            } else{
                index += 1;
            }
        }

        if empty_list.len() == 0{
            let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
            sdoku_temp.copy_from_slice(sdoku);
            
            solve_list.push(sdoku_temp);

            for elem in assign_list.iter(){
                empty_list.push(COORD1 { x: elem.x, y: elem.y, group: elem.group, val: elem.val });
                sdoku[elem.x + elem.y * NUM_X * NUM_Y] = 0;
            }

            return 1;
        }
        let tmp = COORD1{ x : empty_list[0].x, y : empty_list[0].y, group : empty_list[0].group, val : empty_list[0].val};
        assign_list.push(COORD1{ x : empty_list[0].x, y : empty_list[0].y, group : empty_list[0].group, val : empty_list[0].val});
        empty_list.remove(0);

        let available_list = self.get_available_numbers(sdoku, tmp.y, tmp.x);
        
        let mut result = 0;
        //let mut emptylistTemp = (*emptyList[1..];
        
        for (_, elem) in available_list.iter().enumerate(){
            sdoku[tmp.x + tmp.y * NUM_X * NUM_Y] = *elem;
            let temp_result = self.solve_sdoku(sdoku, empty_list, solve_list);
            //sdoku[tmp.x + tmp.y * NUM_X * NUM_Y] = 0;
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;
        }
        for elem in assign_list.iter(){
            empty_list.push(COORD1 { x: elem.x, y: elem.y, group: elem.group, val: elem.val }); 
            sdoku[elem.x + elem.y * NUM_X * NUM_Y] = 0;
        }
        return result;
    }

}