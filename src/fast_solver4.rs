
pub use crate::solver::*;
use array_init;

pub struct FastSolver4{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for FastSolver4{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<(usize, usize, usize, usize)> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push((j, i, (j / NUM_X) + (i / NUM_Y) * NUM_Y, 0 ));
                }
            }
        }
        let mut available_list : [Vec<usize>; NUM_X * NUM_Y * NUM_X * NUM_Y] = array_init::array_init(|_| Vec::new());
        for elem in empty_list.iter(){
            available_list[elem.0 + elem.1 * NUM_X * NUM_Y] = self.get_available_numbers(sdoku, elem.1, elem.0);
        }

        return self.solve_sdoku(sdoku, &mut empty_list, &available_list, solve_list);
    }
}

impl FastSolver4{
    fn assign_value(&self, sdoku : &mut [usize], x : usize, y : usize, val : usize, available_list : &mut [Vec<usize>], empty_list : &Vec<(usize, usize, usize, usize)>) {
        let index = x + y * NUM_X * NUM_Y;
        sdoku[index] = val;

        for elem in empty_list.iter(){
            let index1 = elem.0 + elem.1 * NUM_X * NUM_Y;
            if elem.0 == x{
                available_list[index1].retain(|&x| x != val);
            } else if elem.1 == y{
                available_list[index1].retain(|&x| x != val);
            } else if(elem.0 / NUM_X == x / NUM_X) && (elem.1 / NUM_Y == y / NUM_Y){
                available_list[index1].retain(|&x| x != val);
            }
        }
    }
    fn solve_sdoku(&self, sdoku : &[usize], empty_list : &mut Vec<(usize, usize, usize, usize)>, available_list : &[Vec<usize>], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);

        let mut empty_list_temp: Vec<(usize, usize, usize, usize)> = empty_list.clone();
        let mut available_list_temp : [Vec<usize>; NUM_X * NUM_Y * NUM_X * NUM_Y] = array_init::array_init(|i| if available_list[i].len() > 0 { available_list[i].clone() } else {Vec::new()});
        
        let mut index;
        let mut pos = 0;
        
        while pos < empty_list_temp.len(){
            index = empty_list_temp[pos].0 + empty_list_temp[pos].1 * NUM_X * NUM_Y;
            if available_list_temp[index].len() == 0{
                return 0;
            }
            if available_list_temp[index].len() == 1{
                let tmp = empty_list_temp[pos].clone();
                empty_list_temp.remove(pos);
                self.assign_value(&mut sdoku_temp, tmp.0, tmp.1, available_list_temp[tmp.0 + tmp.1 * NUM_X * NUM_Y][0], &mut available_list_temp, &mut empty_list_temp);
                pos = 0;
            } else{
                pos += 1;
            }
        }

        if empty_list_temp.len() == 0{
            solve_list.push(sdoku_temp);
            return 1;
        }
        
        let tmp = empty_list_temp[0].clone();
        empty_list_temp.remove(0);
        let mut result = 0;
	    index = tmp.0 + tmp.1 * NUM_X * NUM_Y;
        let tmp_list = available_list_temp[index].clone();
        let available_list_temp2 : [Vec<usize>; NUM_X * NUM_Y * NUM_X * NUM_Y] = array_init::array_init(|i| available_list_temp[i].clone());
        
        
        for elem in tmp_list.iter(){
            self.assign_value(&mut sdoku_temp, tmp.0, tmp.1, *elem, &mut available_list_temp, &empty_list_temp);
            let temp_result = self.solve_sdoku(&sdoku_temp, &mut empty_list_temp, &mut available_list_temp, solve_list);
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;

            for m in 0..empty_list_temp.len() {
                let index1 = empty_list_temp[m].0 + empty_list_temp[m].1 * NUM_X * NUM_Y;
                if empty_list_temp[m].0 == tmp.0 {
                    available_list_temp[index1] = available_list_temp2[index1].clone();
                } else if empty_list_temp[m].1 == tmp.1 {
                    available_list_temp[index1] = available_list_temp2[index1].clone();
                //} else if(empty_list_temp[m].x / NUM_X == tmp.x / NUM_X) && (empty_list_temp[m].y / NUM_Y == tmp.y / NUM_Y){
                } else if empty_list_temp[m].2 == (tmp.0/NUM_X + tmp.1/NUM_Y*NUM_Y){
                    available_list_temp[index1] = available_list_temp2[index1].clone();
                }
                
                /*else if empty_list_temp[m].group == (tmp.x/NUM_X + tmp.y/NUM_Y*NUM_Y) {
                    available_list_temp[index1] = available_list_temp2[index1].clone();
                }*/
            }
        }
        return result;
    }

}