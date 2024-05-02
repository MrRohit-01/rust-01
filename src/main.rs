

enum Direction{
  up,
  down,
  left,
  right
}
impl Direction {
   fn default(&self)->isize{
    match self {
        Direction::up => 1,
        Direction::down => 2,
        Direction::left => 3,
        Direction::right => 4
    }
   }
}
fn main(){
    let north =Direction::up;
let default_value = north.default();
println!("{}",default_value)
}

