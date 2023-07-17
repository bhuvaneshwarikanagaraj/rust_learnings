use std::io::{stdin, self};

 fn main(){

    const PI:f32 =3.1412;
    const WANT:i32 =10000;
    let age:&str="21";
    let mut age : u32 = age.trim().parse().unwrap();
    age=age+1;
    let mut name:String=String::new();
    println!("what is your name?");
    io::stdin()
    .read_line(&mut name)
    .expect("Sorry the name is not entered");
    print!("Hi I'm {} and i'm {} old and i want to earn {}",name,age,WANT);
 }
