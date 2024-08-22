pub use crate::solver::*;

pub struct NewFastSolver1{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NewFastSolver1{
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

impl NewFastSolver1{
    fn assign_value(&self, assign_list : &mut Vec<(usize, usize, usize, usize)>, x : usize, y : usize, group: usize,  val : usize, available_list : &mut [Vec<usize>], empty_list : &mut Vec<(usize, usize, usize)>){
        let length = assign_list.len();
        assign_list[length - 1].3 = val;

        for elem in empty_list.iter(){
            let index1 = elem.0 + elem.1 * NUM_X * NUM_Y;
            
            if elem.0 == x{
                available_list[index1].retain(|&x| x != val);
            } else if elem.1 == y{
                available_list[index1].retain(|&x| x != val);
            } else if elem.2 == group{
                available_list[index1].retain(|&x| x != val);
            }
            
        }
    }
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<(usize, usize, usize)>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut assign_list : Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut assign_list_list : Vec<Vec<(usize, usize, usize, usize)>> = Vec::new();
        let mut available_list : [Vec<usize>; NUM_X * NUM_Y * NUM_X * NUM_Y] = array_init::array_init(|_| Vec::new());
        for elem in empty_list.iter(){
            available_list[elem.0 + elem.1 * NUM_X * NUM_Y] = self.get_available_numbers(sdoku, elem.1, elem.0);
        }

        let result = self.solve_sdoku_r(&mut assign_list, empty_list, &mut available_list, &mut assign_list_list);
        
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        for elem in assign_list_list[0].iter(){
            sdoku_temp[elem.0 + elem.1 * NUM_X * NUM_Y] = elem.3;
        }
        solve_list.push(sdoku_temp);
        return result;
    }
    
    fn solve_sdoku_r(&self, assign_list : &mut Vec<(usize, usize, usize, usize)>, empty_list : &mut Vec<(usize, usize, usize)>, available_list : &mut [Vec<usize>], assign_list_list : &mut Vec<Vec<(usize, usize, usize, usize)>>) -> i32{
        let mut empty_list_temp = empty_list.clone();
        let mut assign_list_temp = assign_list.clone();
        
        let mut available_list_temp : [Vec<usize>; NUM_X * NUM_Y * NUM_X * NUM_Y] = array_init::array_init(|i| if available_list[i].len() > 0 { available_list[i].clone() } else {Vec::new()});
        


        let mut index;
        let mut pos = 0;

        while pos < empty_list_temp.len(){
            index = empty_list_temp[pos].0 + empty_list_temp[pos].1 * NUM_X * NUM_Y;
            if available_list_temp[index].len() == 0{
                return 0;
            }
            if available_list_temp[index].len() == 1{
                let (x, y, group) = empty_list_temp[pos];
	            let val = available_list_temp[index][0];

                assign_list_temp.push((x, y, group, val));
                empty_list_temp.remove(pos);
                self.assign_value(&mut assign_list_temp, x, y, group, val, &mut available_list_temp, &mut empty_list_temp);
                pos = 0;
            } else{
                pos += 1;
            }
        }

        if empty_list_temp.len() == 0{
            assign_list_list.push(assign_list_temp);
            return 1;
        }

        let mut result = 0;
        let (x, y, group) = empty_list_temp[0];
        assign_list_temp.push((x, y, group, 0));        
	    let index = x + y * NUM_X * NUM_Y;
        
        empty_list_temp.remove(0);
        
        let mut available_list_temp2 : [Vec<usize>; NUM_X * NUM_Y * NUM_X * NUM_Y] = array_init::array_init(|i| Vec::new());
        
        
        let mut modify_list : Vec<(usize, usize, usize)> = Vec::new();
        for elem in empty_list_temp.iter(){
            let index2 = elem.0 + elem.1 * NUM_X * NUM_Y;
            if elem.0 == x{
                available_list_temp2[index2] = available_list_temp[index2].clone();
                modify_list.push((elem.0, elem.1, elem.2));
            } else if elem.1 == y {
                available_list_temp2[index2] = available_list_temp[index2].clone();
                modify_list.push((elem.0, elem.1, elem.2));
            } else if elem.2 == (x/NUM_X + y/NUM_Y*NUM_Y){
                available_list_temp2[index2] = available_list_temp[index2].clone();
                modify_list.push((elem.0, elem.1, elem.2));
            }
        }

        //for elem in available_list_temp[index].iter(){
        for i in 0..available_list_temp[index].len(){
            self.assign_value(&mut assign_list_temp, x, y, group, available_list_temp[index][i], &mut available_list_temp, &mut empty_list_temp);
            let temp_result = self.solve_sdoku_r(&mut assign_list_temp, &mut empty_list_temp, &mut available_list_temp, assign_list_list);
            
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;

            
            for elem2 in modify_list.iter(){
                let index1 = elem2.0 + elem2.1 * NUM_X * NUM_Y;
                
                if elem2.0 == x{
                    available_list_temp[index1] = available_list_temp2[index1].clone();
                } else if elem2.1 == y{
                    available_list_temp[index1] = available_list_temp2[index1].clone();
                } else if elem2.2 == x / NUM_X + y / NUM_Y * NUM_Y{
                    available_list_temp[index1] = available_list_temp2[index1].clone();
                }
            }
        }
        
        return result;
        
    }

}