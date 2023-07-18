struct my_box{

    depth:i32,
    height:i32,
    width:i32,
    // we have three different fields here depth, height and width
}

fn main(){

    let Box = my_box{
        depth:3,
        height:5,
        width:6,
    };
    println!("The height of my box is {}",Box.height);
}