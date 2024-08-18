pub use crate::solver::*;

pub struct NewFastSolver2{
    //pub m_solver : Vec<[i32 ; NUM_X * NUM_Y * NUM_X * NUM_Y]>,
}

impl Solver for NewFastSolver2{
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut empty_list : Vec<COORD2> = Vec::new();
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                if sdoku[j + i * NUM_X * NUM_Y] == 0{
                    empty_list.push(COORD2 { x: j, y: i, group: (j / NUM_X) + (i / NUM_Y) * NUM_Y, val: 0, available_list: self.get_available_numbers(sdoku, i, j) });
                }
            }
        }
        
        self.solve_sdoku(sdoku, &mut empty_list, solve_list)
    }

    
}

impl NewFastSolver2{
    fn assign_value(&self, assign_list : &mut Vec<COORD2>, x : usize, y : usize, val : usize, empty_list : &mut Vec<COORD2>){
        let length = assign_list.len();
        assign_list[length - 1].val = val;

        //for elem in empty_list.iter(){
        for i in 0..empty_list.len(){
            if empty_list[i].x == x{
                empty_list[i].available_list.retain(|&x| x != val);
            //    break;
            } else if empty_list[i].y == y{
                empty_list[i].available_list.retain(|&x| x != val);
            //    break;
            //} else if(empty_list[i].x / NUM_X == x / NUM_X) && (empty_list[i].y / NUM_Y == y / NUM_Y){
            } else if empty_list[i].group == (x/NUM_X + y/NUM_Y*NUM_Y){
                empty_list[i].available_list.retain(|&x| x != val);
            //    break;
            }
        }
    }
    fn solve_sdoku(&self, sdoku : &mut [usize], empty_list : &mut Vec<COORD2>, solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32{
        let mut assign_list : Vec<COORD2> = Vec::new();
        let mut assign_list_list : Vec<Vec<COORD2>> = Vec::new();
        
        let result = self.solve_sdoku_r(&mut assign_list, empty_list, &mut assign_list_list);
        
        let mut sdoku_temp : [usize; NUM_X * NUM_Y * NUM_X * NUM_Y] = [0 ; NUM_X * NUM_Y * NUM_X * NUM_Y];
        sdoku_temp.copy_from_slice(sdoku);
        for elem in assign_list_list[0].iter(){
            sdoku_temp[elem.x + elem.y * NUM_X * NUM_Y] = elem.val;
        }
        solve_list.push(sdoku_temp);
        return result;
    }
    
    fn solve_sdoku_r(&self, assign_list : &mut Vec<COORD2>, empty_list : &mut Vec<COORD2>, assign_list_list : &mut Vec<Vec<COORD2>>) -> i32{
        let mut empty_list_temp: Vec<COORD2> = Vec::new();
        let mut assign_list_temp: Vec<COORD2> = Vec::new();
        for elem in empty_list.iter(){
            empty_list_temp.push(COORD2 { x: elem.x, y: elem.y, group: elem.group, val: elem.val, available_list: elem.available_list.clone() });
        }
        for elem in assign_list.iter(){
            assign_list_temp.push(COORD2 { x: elem.x, y: elem.y, group: elem.group, val: elem.val, available_list: elem.available_list.clone() });
        }
        let mut pos = 0;

        while pos < empty_list_temp.len(){
            if empty_list_temp[pos].available_list.len() == 0{
                return 0;
            }
            if empty_list_temp[pos].available_list.len() == 1{
                let x = empty_list_temp[pos].x;
                let y = empty_list_temp[pos].y;
                let val = empty_list_temp[pos].available_list[0];

                assign_list_temp.push(COORD2 { x: x, y: y, group: empty_list_temp[pos].group, val: empty_list_temp[pos].val, available_list : empty_list_temp[pos].available_list.clone() });
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
	    let x = empty_list_temp[0].x;
        let y = empty_list_temp[0].y;
        let group = empty_list_temp[0].group;
        let tmp_list = empty_list_temp[0].available_list.clone();
        assign_list_temp.push(COORD2 { x: empty_list_temp[0].x, y: empty_list_temp[0].y, group: empty_list_temp[0].group, val: empty_list_temp[0].val, available_list : empty_list_temp[0].available_list.clone() });
        
        empty_list_temp.remove(0);
        
        let mut modify_list : Vec<usize> = Vec::new();
        let mut empty_list_temp2: Vec<COORD2> = Vec::new();
        for i in 0..empty_list_temp.len(){
            let elem = &empty_list_temp[i];
            if elem.x == x{
                empty_list_temp2.push(COORD2 { x: elem.x, y: elem.y, group: elem.group, val: elem.val, available_list: elem.available_list.clone() });
                modify_list.push(i);
            } else if elem.y == y{
                empty_list_temp2.push(COORD2 { x: elem.x, y: elem.y, group: elem.group, val: elem.val, available_list: elem.available_list.clone() });
                modify_list.push(i);
            } else if elem.group == group{
                empty_list_temp2.push(COORD2 { x: elem.x, y: elem.y, group: elem.group, val: elem.val, available_list: elem.available_list.clone() });
                modify_list.push(i);
            } else{
                empty_list_temp2.push(COORD2 { x: elem.x, y: elem.y, group: elem.group, val: elem.val, available_list: Vec::new() });
            }
        }

        
        
        //for elem in available_list_temp[index].iter(){
        for i in 0..tmp_list.len(){
            self.assign_value(&mut assign_list_temp, x, y, tmp_list[i], &mut empty_list_temp);
            let temp_result = self.solve_sdoku_r(&mut assign_list_temp, &mut empty_list_temp, assign_list_list);
            
            if temp_result > 1{
                result = 2;
                break;
            }
            result += temp_result;

            /*for m in 0..empty_list_temp.len() {
                if empty_list_temp[m].x == x {
                    empty_list_temp[m].available_list = empty_list_temp2[m].available_list.clone();
                } else if empty_list_temp[m].y == y {
                    empty_list_temp[m].available_list = empty_list_temp2[m].available_list.clone();
                } else if empty_list_temp[m].group == (x/NUM_X + y/NUM_Y*NUM_Y){
                    empty_list_temp[m].available_list = empty_list_temp2[m].available_list.clone();
                }
            }*/
            for m in modify_list.iter(){
                if empty_list_temp[*m].x == x {
                    empty_list_temp[*m].available_list = empty_list_temp2[*m].available_list.clone();
                } else if empty_list_temp[*m].y == y {
                    empty_list_temp[*m].available_list = empty_list_temp2[*m].available_list.clone();
                } else if empty_list_temp[*m].group == (x/NUM_X + y/NUM_Y*NUM_Y){
                    empty_list_temp[*m].available_list = empty_list_temp2[*m].available_list.clone();
                }
            }
        }
        
        return result;
        
    }

}