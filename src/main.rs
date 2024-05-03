

enum Shapes{
  Circle(f32),
  Rectangle(f32,f32),
  Square(f32)

}

   fn area(shape : Shapes)->f32 {
   match shape {
        Shapes::Circle(radius) => 3.14 * radius * radius,
        Shapes::Rectangle(width,height) =>width * height,
        Shapes::Square(side) => side * side,

    }
   }

fn main(){
    let square = Shapes::Square(170.0);
    let rectangle = Shapes::Rectangle(170.0,10.0);
    let circle = Shapes::Circle(170.0);
    println!("{}",area(square));
    println!("{}",area(rectangle));
    println!("{}",area(circle))
}

