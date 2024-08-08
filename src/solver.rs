
pub const NUM_X : usize = 3;
pub const NUM_Y : usize = 3;
pub trait Solver{
    fn print_sdoku(&self, sdoku : &[usize]){
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                print!("{} ", sdoku[j+i*(NUM_X * NUM_Y)]);
            }
            println!("");
        }
    }
    fn print_emptylist(&self, empty_list : &Vec<COORD1>){
        for (i, elem) in empty_list.iter().enumerate(){
            println!("{}, {}, {}, {}, {}", i, elem.x, elem.y, elem.group, elem.val);
        }
    }
    fn get_available_numbers(&self, sdoku : &[usize], y : usize, x : usize) -> Vec<usize>{
        let mut res : Vec<usize> = Vec::new();
        let mut is_avail : bool;
        for aa in 1 .. (NUM_X * NUM_Y + 1){
            is_avail = true;
            for m in 0 .. (NUM_X * NUM_Y){
                if sdoku[m + y * NUM_X * NUM_Y] == aa{
                    is_avail = false;
                    break;
                }
                if sdoku[x + m * NUM_X * NUM_Y] == aa{
                    is_avail = false;
                    break;
                }
            }

            if is_avail == true{
                let index1 = (y / NUM_Y) * NUM_X;
                let index2 = (x / NUM_X) * NUM_Y;
                for m in index1 .. (index1 + NUM_Y){
                    if is_avail == true{
                        for n in index2 .. (index2 + NUM_X){
                            if sdoku[n + m * NUM_X * NUM_Y] == aa{
                                is_avail = false;
                                break;
                            }
                        }
                    }
                }
            }

            if is_avail == true{
                res.push(aa);
            }
        }
        return res;
    }
    fn solve_sdoku(&self, sdoku : &mut [usize], solve_list : &mut Vec<[usize ; NUM_X * NUM_Y * NUM_X * NUM_Y]>) -> i32;
}

//#[derive(Clone, Copy)]
pub struct COORD1{
    pub x : usize,
    pub y : usize,
    pub group : usize,
    pub val : usize,
}