fn main(){
   let hello:(u8,String,f64)=(21,"Bhuvaneshwari".to_string(),50_000.00);
   print!("My name {},",hello.1);
   print!(" I'm {} years old",hello.0);
   println!(" and my salary is {}", hello.2);
   let (v1,v2,v3)=hello;
   println!("Age : {}",v1);  

}