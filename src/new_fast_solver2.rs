use std::io::empty;

pub use crate::solver::*;

pub struct NewFastSolver2{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NewFastSolver2{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<(usize, usize, usize, usize, Vec<usize>)> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push((j, i, (j / NUM_X) + (i / NUM_Y) * NUM_Y, 0, self.get_available_numbers(sdoku, i, j) ));
                }
            }
        }
        
        self.solve_sdoku(sdoku, &mut empty_list, solve_list)
    }

    
}

impl NewFastSolver2{
    fn assign_value(&self, assign_list : &mut Vec<(usize, usize, usize, usize, Vec<usize>)>, x : usize, y : usize, val : usize, empty_list : &mut Vec<(usize, usize, usize, usize, Vec<usize>)>) -> Vec<usize>{
        let length = assign_list.len();
        assign_list[length - 1].3 = val;
        let mut res = Vec::new();
        //for elem in empty_list.iter(){
        for i in 0..empty_list.len(){
            let pre_len = empty_list[i].4.len();
            if empty_list[i].0 == x{
                empty_list[i].4.retain(|&x| x != val);
            //    break;
            } else if empty_list[i].1 == y{
                empty_list[i].4.retain(|&x| x != val);
            //    break;
            //} else if(empty_list[i].x / NUM_X == x / NUM_X) && (empty_list[i].y / NUM_Y == y / NUM_Y){
            } else if empty_list[i].2 == (x/NUM_X + y/NUM_Y*NUM_Y){
                empty_list[i].4.retain(|&x| x != val);
            //    break;
            }
            let post_len = empty_list[i].4.len();
            if post_len != pre_len{
                res.push(i);
            }
        }
        return res;
    }
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<(usize, usize, usize, usize, Vec<usize>)>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut assign_list : Vec<(usize, usize, usize, usize, Vec<usize>)> = Vec::new();
        let mut assign_list_list : Vec<Vec<(usize, usize, usize, usize, Vec<usize>)>> = Vec::new();
        
        let result = self.solve_sdoku_r(&mut assign_list, empty_list, &mut assign_list_list);
        
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        for elem in assign_list_list[0].iter(){
            sdoku_temp[elem.0 + elem.1 * NUM_X * NUM_Y] = elem.3;
        }
        solve_list.push(sdoku_temp);
        return result;
    }
    
    fn solve_sdoku_r(&self, assign_list : &mut Vec<(usize, usize, usize, usize, Vec<usize>)>, empty_list : &mut Vec<(usize, usize, usize, usize, Vec<usize>)>, assign_list_list : &mut Vec<Vec<(usize, usize, usize, usize, Vec<usize>)>>) -> i32{
        let mut empty_list_temp: Vec<(usize, usize, usize, usize, Vec<usize>)> = empty_list.clone();
        let mut assign_list_temp: Vec<(usize, usize, usize, usize, Vec<usize>)> = assign_list.clone();
        
        let mut pos = 0;

        while pos < empty_list_temp.len(){
            if empty_list_temp[pos].4.len() == 0{
                return 0;
            }
            if empty_list_temp[pos].4.len() == 1{
                let x = empty_list_temp[pos].0;
                let y = empty_list_temp[pos].1;
                let val = empty_list_temp[pos].4[0];

                assign_list_temp.push(empty_list_temp[pos].clone());
                empty_list_temp.remove(pos);
                self.assign_value(&mut assign_list_temp, x, y, val, &mut empty_list_temp);
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
        let tmp_list = empty_list_temp[0].4.clone();
        assign_list_temp.push(empty_list_temp[0].clone());
        
        empty_list_temp.remove(0);
        
        /*let mut modify_list : Vec<usize> = Vec::new();
        let mut available_list_temp = Vec::new();
        for i in 0..empty_list_temp.len(){
            let elem = &empty_list_temp[i];
            if elem.0 == x{
                available_list_temp.push(elem.4.clone());
                //empty_list_temp2.push(elem.clone());
                modify_list.push(i);
            } else if elem.1 == y{
                available_list_temp.push(elem.4.clone());
                //empty_list_temp2.push(elem.clone());
                modify_list.push(i);
            } else if elem.2 == group{
                available_list_temp.push(elem.4.clone());
                //empty_list_temp2.push(elem.clone());
                modify_list.push(i);
            } else{
                available_list_temp.push(elem.4.clone());
                //empty_list_temp2.push(elem.clone());
                
            }
        }*/

        
        
        //for elem in available_list_temp[index].iter(){
        for i in 0..tmp_list.len(){
            let val_list = self.assign_value(&mut assign_list_temp, x, y, tmp_list[i], &mut empty_list_temp);
            let temp_result = self.solve_sdoku_r(&mut assign_list_temp, &mut empty_list_temp, assign_list_list);
            
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;

            for m in val_list.iter(){
                empty_list_temp[*m].4.push(tmp_list[i]);
            }
            /*for m in modify_list.iter(){
                if empty_list_temp[*m].0 == x {
                    empty_list_temp[*m].4 = available_list_temp[*m].clone();
                } else if empty_list_temp[*m].1 == y {
                    empty_list_temp[*m].4 = available_list_temp[*m].clone();
                } else if empty_list_temp[*m].2 == (x/NUM_X + y/NUM_Y*NUM_Y){
                    empty_list_temp[*m].4 = available_list_temp[*m].clone();
                }
            }*/
        }
        
        return result;
        
    }

}