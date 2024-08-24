pub use crate::solver::*;

pub struct NewFastSolver2_1{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NewFastSolver2_1{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<(usize, usize, usize, usize)> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push((j, i, (j / NUM_X) + (i / NUM_Y) * NUM_Y, 0));
                }
            }
        }
        
        self.solve_sdoku(sdoku, &mut empty_list, solve_list)
    }

    
}

impl NewFastSolver2_1{
    fn assign_value(&self, assign_list : &mut Vec<(usize, usize, usize, usize)>, x : usize, y : usize, val : usize, empty_list : &mut Vec<(usize, usize, usize, usize)>, available_list : &mut Vec<Vec<usize>>) -> Vec<usize>{
        let length = assign_list.len();
        assign_list[length - 1].3 = val;
        
        //for elem in empty_list.iter(){
        let mut res = Vec::new();
        for i in 0..empty_list.len(){
            let pre_len = available_list[i].len();
            if empty_list[i].0 == x{
                available_list[i].retain(|&x| x != val);
            } else if empty_list[i].1 == y{
                available_list[i].retain(|&x| x != val);
            } else if empty_list[i].2 == (x/NUM_X + y/NUM_Y*NUM_Y){
                available_list[i].retain(|&x| x != val);
            }
            let post_len = available_list[i].len();
            if post_len != pre_len{
                res.push(i);
            }
        }
        return res;
    }
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<(usize, usize, usize, usize)>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut assign_list : Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut available_list : Vec<Vec<usize>> = Vec::new();
        let mut assign_list_list : Vec<Vec<(usize, usize, usize, usize)>> = Vec::new();
        for elem in empty_list.iter(){
            available_list.push(self.get_available_numbers(sdoku, elem.1, elem.0));
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
    
    fn solve_sdoku_r(&self, assign_list : &mut Vec<(usize, usize, usize, usize)>, empty_list : &mut Vec<(usize, usize, usize, usize)>, available_list : &mut Vec<Vec<usize>>, assign_list_list : &mut Vec<Vec<(usize, usize, usize, usize)>>) -> i32{
        let mut empty_list_temp: Vec<(usize, usize, usize, usize)> = empty_list.clone();
        let mut assign_list_temp: Vec<(usize, usize, usize, usize)> = assign_list.clone();
        let mut available_list_temp: Vec<Vec<usize>> = available_list.clone();
        let mut pos = 0;

        while pos < empty_list_temp.len(){
            
            if available_list_temp[pos].len() == 0{
                return 0;
            }
            if available_list_temp[pos].len() == 1{
                let x = empty_list_temp[pos].0;
                let y = empty_list_temp[pos].1;
                let val = available_list_temp[pos][0];

                assign_list_temp.push((x, y, empty_list_temp[0].2, val));
                empty_list_temp.remove(pos);
                available_list_temp.remove(pos);
                self.assign_value(&mut assign_list_temp, x, y, val, &mut empty_list_temp, &mut available_list_temp);
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
	    let x = empty_list_temp[0].0;
        let y = empty_list_temp[0].1;
        let group = empty_list_temp[0].2;
        let tmp_list = available_list_temp[0].clone();
        assign_list_temp.push((x, y, group, 0));
        
        empty_list_temp.remove(0);
        available_list_temp.remove(0);

        for i in 0..tmp_list.len(){
            let val_list = self.assign_value(&mut assign_list_temp, x, y, tmp_list[i], &mut empty_list_temp, &mut available_list_temp);
            let temp_result = self.solve_sdoku_r(&mut assign_list_temp, &mut empty_list_temp, &mut available_list_temp, assign_list_list);
            
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;

            for m in val_list.iter(){
                available_list_temp[*m].push(tmp_list[i]);
            }
        }
        
        return result;
        
    }

}