

enum Shapes{
  Circle(f32),
  Rectangle(f32,f32),
  Square(f32)

}
impl Shapes{
  fn area(&self)->f32 {
   match self {
        Shapes::Circle(radius) => 3.14 * radius * radius,
        Shapes::Rectangle(width,height) =>width * height,
        Shapes::Square(side) => side * side,

    }
   
}

   }

fn main(){
    let square = Shapes::Square(170.0);
    let rectangle = Shapes::Rectangle(170.0,10.0);
    let circle = Shapes::Circle(170.0);
    println!("{}",square.area());
    println!("{}",rectangle.area());
    println!("{}",circle.area())
}

