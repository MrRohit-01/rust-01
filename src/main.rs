struct User {
    name:String,
    age:u8,
    vote:bool,
    email:String
}
fn main(){
    let user =User{
        name:String::from("rohit kumar barada"),
        vote:true,
        age:21,
        email:String::from("rohitkumar@gmail.com")
    };
    println!("{} is {} years old",user.name,user.age);
}
