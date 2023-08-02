fn sum(a:i32 , b:i32) -> i32{
    let mut count=0;

    for i in a..b{
    count=count+i;
    }
    return count;
}

fn main(){
    println!("{}",sum(1,4));
}