pub use crate::solver::*;
use std::collections::HashMap;
pub struct NewFastSolver3_0{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NewFastSolver3_0{
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

impl NewFastSolver3_0{
    fn assign_value(&self, x : usize, y : usize, group : usize, val : usize, empty_map : &mut HashMap<(usize, usize, usize), Vec<usize>>){
        
        for (coord, elem) in empty_map{
            if coord.0 == x{
                elem.retain(|&x| x != val);
            } else if coord.1 == y{
                elem.retain(|&x| x != val);
            } else if coord.2 == group{
                elem.retain(|&x| x != val);
            }
            
        }
    }
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<(usize, usize, usize, usize)>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut assign_map: HashMap<(usize, usize, usize), usize> = HashMap::new();
        let mut assign_map_list : Vec<HashMap<(usize, usize, usize), usize>> = Vec::new();
        let mut empty_map: HashMap<(usize, usize, usize), Vec<usize>> = HashMap::new();
        let mut empty_lists : Vec<(usize, usize, usize)> = Vec::new();

        for elem in empty_list.iter(){
            empty_map.insert((elem.0, elem.1, elem.2), self.get_available_numbers(sdoku, elem.1, elem.0));
            empty_lists.push((elem.0, elem.1, elem.2));
        }
        
        let result = self.solve_sdoku_r(&mut assign_map, &mut empty_map, &mut empty_lists, &mut assign_map_list);
        
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        for elem in assign_map_list[0].iter(){
            sdoku_temp[elem.0.0 + elem.0.1 * NUM_X * NUM_Y] = *(elem.1);
        }
        solve_list.push(sdoku_temp);
        return result;
    }
    
    fn solve_sdoku_r(&self, assign_map : &mut HashMap<(usize, usize, usize), usize>, empty_map : &mut HashMap<(usize, usize, usize), Vec<usize>>, empty_list : &mut Vec<(usize, usize, usize)>, assign_map_list : &mut Vec<HashMap<(usize, usize, usize), usize>>) -> i32{
        let mut empty_map_temp: HashMap<(usize, usize, usize), Vec<usize>> = empty_map.clone();
        let mut assign_map_temp: HashMap<(usize, usize, usize), usize> = assign_map.clone();
        let mut empty_list_temp = empty_list.clone();

        let mut pos = 0;
        while pos < empty_list_temp.len(){
            let available_list = empty_map_temp[&empty_list_temp[pos]].clone();
            match available_list.len() {
                0 => return 0,
                1 => {
                    let (x, y, group) = empty_list_temp[pos];
                    let val = available_list[0];
                    assign_map_temp.insert((x, y, group), val);
                    empty_map_temp.remove(&empty_list_temp[pos]);
                    empty_list_temp.remove(pos);
                    self.assign_value(x, y, group, val, &mut empty_map_temp);
                    pos = 0;
                },
                _ => pos += 1,
            };
        }

        if empty_map_temp.is_empty(){
            assign_map_list.push(assign_map_temp);
            return 1;
        }

        let mut result = 0;
        let (x, y, group) = empty_list_temp[0];
        let available_list = empty_map_temp[&empty_list_temp[0]].clone();
	    assign_map_temp.insert((x, y, group), 0);
        empty_map_temp.remove(&empty_list_temp[0]);
        empty_list_temp.remove(0);
        
        
        let mut empty_map_temp2: HashMap<(usize, usize, usize), Vec<usize>> = HashMap::new();
        for(k, v) in empty_map_temp.iter(){
            if k.0 == x{
                empty_map_temp2.insert(*k, v.clone());
            } else if k.1 == y{
                empty_map_temp2.insert(*k, v.clone());
            } else if k.2 == group{
                empty_map_temp2.insert(*k, v.clone());
            }
        }
        //for elem in available_list_temp[index].iter(){
        for i in 0..available_list.len(){
            let val = available_list[i];
            assign_map_temp.entry((x, y, group)).and_modify(|x| *x = val);
            self.assign_value(x, y, group, val, &mut empty_map_temp);
            let temp_result = self.solve_sdoku_r(&mut assign_map_temp, &mut empty_map_temp, &mut empty_list_temp, assign_map_list);
            
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;

            for (k, v) in empty_map_temp2.iter(){
                if k.0 == x{
                    empty_map_temp.entry(*k).and_modify(|x1| *x1 = v.clone());
                } else if k.1 == y{
                    empty_map_temp.entry(*k).and_modify(|x1| *x1 = v.clone());
                } else if k.2 == group{
                    empty_map_temp.entry(*k).and_modify(|x1| *x1 = v.clone());
                }

            }
            
        }
        
        return result;
        
    }

}