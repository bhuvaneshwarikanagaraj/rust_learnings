enum colors{
    red,
    yellow,
    green,
    blue
}

fn main(){
    let which_color=colors::red;
    match which_color{

        colors::blue=>println!("Its Blue colour"),
        colors::green=>println!("Its green colour"),
        colors::yellow=>println!("Its yellow colour"), 
        colors::red=>println!("Its red Colour"),
        _ =>println!("Other colour"),  
    }


}