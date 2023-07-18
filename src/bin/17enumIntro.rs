
//  **Enum Notes**

enum enum_name{

    Variance1,
    variance2,
    Variance3,
    variance4
    

    //These are the variance of the enum_name
    
    //and the last variance unlike match expression does not nessecarly need to end with ","
}

fn main(){
    let find_best=enum_name::variance2;
    match find_best
     {

       enum_name::Variance1 =>println!("one1"),
       enum_name::Variance3 =>println!("three3"),
       enum_name::variance4 =>println!("four4"),
       enum_name::variance2 =>println!("two2"),
       _ => println!("Its other variance"),
        
    }
}


// fn main(){

// }