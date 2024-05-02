struct Plot {
    width :u32,
    height:u32
}
impl Plot {
    fn area(&self)->u32{
       return self.width*self.height;
    }
}
fn main(){
    let plot1 = Plot {
        width:32,
        height:44
    };
    println!("width :{} ,height :{}, area : {}",plot1.width,plot1.height,plot1.area())
}
