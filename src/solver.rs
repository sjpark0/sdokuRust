
pub const NUM_X : usize = 3;
pub const NUM_Y : usize = 3;
pub trait Solver{
    fn print_sdoku(&self, sdoku : &[i32]){
        for i in 0..(NUM_X * NUM_Y){
            for j in 0..(NUM_X * NUM_Y){
                print!("{} ", sdoku[j+i*(NUM_X * NUM_Y)]);
            }
            println!("");
        }
    }
}
