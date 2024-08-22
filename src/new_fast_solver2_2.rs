pub use crate::solver::*;
use std::collections::HashMap;
pub struct NewFastSolver2_2{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NewFastSolver2_2{
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

impl NewFastSolver2_2{
    fn assign_value(&self, assign_list : &mut Vec<(usize, usize, usize, usize)>, x : usize, y : usize, val : usize, empty_list : &mut Vec<(usize, usize, usize, usize)>, available_map : &mut HashMap<(usize, usize), Vec<usize>>){
        let length = assign_list.len();
        assign_list[length - 1].3 = val;

        //for elem in empty_list.iter(){
        /*for i in 0..empty_list.len(){
            let index = (empty_list[i].0, empty_list[i].1);
            if empty_list[i].0 == x{
                available_map.get_mut(&index).unwrap().retain(|&x| x != val);
            //    break;
            } else if empty_list[i].1 == y{
                available_map.get_mut(&index).unwrap().retain(|&x| x != val);
            //    break;
            //} else if(empty_list[i].x / NUM_X == x / NUM_X) && (empty_list[i].y / NUM_Y == y / NUM_Y){
            } else if empty_list[i].2 == (x/NUM_X + y/NUM_Y*NUM_Y){
                available_map.get_mut(&index).unwrap().retain(|&x| x != val);
            //    break;
            }
        }*/
        for (k, v) in available_map.iter_mut(){
            if k.0 == x{
                v.retain(|&x| x != val);
            } else if k.1 == y{
                v.retain(|&x| x != val);
            } else if k.0 / NUM_X == x / NUM_X && k.1 / NUM_Y == y / NUM_Y{
                v.retain(|&x| x != val);
            }
        }
    }
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<(usize, usize, usize, usize)>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut assign_list : Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut available_map : HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        let mut assign_list_list : Vec<Vec<(usize, usize, usize, usize)>> = Vec::new();
        for elem in empty_list.iter(){
            available_map.insert((elem.0, elem.1), self.get_available_numbers(sdoku, elem.1, elem.0));
            //available_list.push(self.get_available_numbers(sdoku, elem.1, elem.0));
        }
        let result = self.solve_sdoku_r(&mut assign_list, empty_list, &mut available_map, &mut assign_list_list);
        
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        for elem in assign_list_list[0].iter(){
            sdoku_temp[elem.0 + elem.1 * NUM_X * NUM_Y] = elem.3;
        }
        solve_list.push(sdoku_temp);
        return result;
    }
    
    fn solve_sdoku_r(&self, assign_list : &mut Vec<(usize, usize, usize, usize)>, empty_list : &mut Vec<(usize, usize, usize, usize)>, available_map : &mut HashMap<(usize, usize), Vec<usize>>, assign_list_list : &mut Vec<Vec<(usize, usize, usize, usize)>>) -> i32{
        let mut empty_list_temp: Vec<(usize, usize, usize, usize)> = empty_list.clone();
        let mut assign_list_temp: Vec<(usize, usize, usize, usize)> = assign_list.clone();
        let mut available_map_temp : HashMap<(usize, usize), Vec<usize>> = available_map.clone();
        let mut pos = 0;

        while pos < empty_list_temp.len(){
            if available_map_temp[&(empty_list_temp[pos].0, empty_list_temp[pos].1)].len() == 0{
                return 0;
            }
            if available_map_temp[&(empty_list_temp[pos].0, empty_list_temp[pos].1)].len() == 1{
                let x = empty_list_temp[pos].0;
                let y = empty_list_temp[pos].1;
                let val = available_map_temp[&(empty_list_temp[pos].0, empty_list_temp[pos].1)][0];

                assign_list_temp.push((x, y, empty_list_temp[0].2, val));
                empty_list_temp.remove(pos);
                available_map_temp.remove(&(x, y));
                self.assign_value(&mut assign_list_temp, x, y, val, &mut empty_list_temp, &mut available_map_temp);
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
        let tmp_list = available_map_temp[&(x, y)].clone();
        assign_list_temp.push((x, y, group, 0));
        
        empty_list_temp.remove(0);
        available_map_temp.remove(&(x, y));
        let mut modify_list : Vec<usize> = Vec::new();
        //let mut empty_list_temp2: Vec<(usize, usize, usize, usize, Vec<usize>)> = Vec::new();
        let mut available_list_temp: Vec<(usize, usize, Vec<usize>)> = Vec::new();
        for (k, v) in available_map_temp.iter(){
            if k.0 == x{
                available_list_temp.push((k.0, k.1, v.clone()));
            } else if k.1 == y{
                available_list_temp.push((k.0, k.1, v.clone()));
            } else if k.0 / NUM_X == x / NUM_X && k.1 / NUM_Y == y / NUM_Y{
                available_list_temp.push((k.0, k.1, v.clone()));
            }
        }
        /*for i in 0..empty_list_temp.len(){
            let elem = &empty_list_temp[i];
            if elem.0 == x{
                available_list_temp.push(available_map_temp[&(elem.0, elem.1)].clone());
                //empty_list_temp2.push(elem.clone());
                modify_list.push(i);
            } else if elem.1 == y{
                available_list_temp.push(available_map_temp[&(elem.0, elem.1)].clone());
                //empty_list_temp2.push(elem.clone());
                modify_list.push(i);
            } else if elem.2 == group{
                available_list_temp.push(available_map_temp[&(elem.0, elem.1)].clone());
                //empty_list_temp2.push(elem.clone());
                modify_list.push(i);
            } else{
                available_list_temp.push(available_map_temp[&(elem.0, elem.1)].clone());
                //empty_list_temp2.push(elem.clone());
                
            }
        }*/

        
        
        //for elem in available_list_temp[index].iter(){
        for i in 0..tmp_list.len(){
            self.assign_value(&mut assign_list_temp, x, y, tmp_list[i], &mut empty_list_temp, &mut available_map_temp);
            let temp_result = self.solve_sdoku_r(&mut assign_list_temp, &mut empty_list_temp, &mut available_map_temp, assign_list_list);
            
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;

            /*for m in modify_list.iter(){
                let index = (empty_list_temp[*m].0,empty_list_temp[*m].1);
                if empty_list_temp[*m].0 == x {
                    available_map_temp.entry(index).and_modify(|e| *e = available_list_temp[*m].clone());
                } else if empty_list_temp[*m].1 == y {
                    available_map_temp.entry(index).and_modify(|e| *e = available_list_temp[*m].clone());
                } else if empty_list_temp[*m].2 == (x/NUM_X + y/NUM_Y*NUM_Y){
                    available_map_temp.entry(index).and_modify(|e| *e = available_list_temp[*m].clone());
                }
            }*/
            for m in available_list_temp.iter(){
                if m.0 == x{
                    available_map_temp.entry((m.0, m.1)).and_modify(|e| *e = m.2.clone());
                } else if m.1 == y{
                    available_map_temp.entry((m.0, m.1)).and_modify(|e| *e = m.2.clone());
                } else if m.0 / NUM_X == x / NUM_X && m.1 / NUM_Y == y / NUM_Y{
                    available_map_temp.entry((m.0, m.1)).and_modify(|e| *e = m.2.clone());
                }
            }
        }
        
        return result;
        
    }

}