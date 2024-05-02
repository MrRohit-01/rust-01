use std::ffi::c_void;


fn main() {
    
    let mut greeting =String::from("hello world!");
    change(&mut greeting);
    change1(&mut greeting);
    change2(&mut greeting);
    println!("{}",greeting);
    


}
fn change(new_value : &mut String){
    new_value.push_str("hiiii");
    println!("{}",new_value);
}
fn change1( new_value : &mut String){
    new_value.push_str("hiiii2");
   println!("{}",new_value)
}
fn change2( new_value : &mut String){
    new_value.push_str("hiiii33");
   println!("{}",new_value);

}
