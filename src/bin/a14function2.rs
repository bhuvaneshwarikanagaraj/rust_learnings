use std::result;

fn sum(a1:i32 ,b1:i32 ) -> i32{
 a1+b1

}


fn display_function(result:i32){
    println!("result: {}",result);
}

fn main(){
    let result=sum(4, 5);
    display_function(result);

}