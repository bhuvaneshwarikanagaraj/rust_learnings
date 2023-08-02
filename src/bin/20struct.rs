enum flavours {

chocolate,
vanilla,
rasberry_with_honey,
coffee
  
}
//created a list called 4 diiferent variance with four different flavours

struct drink_info{

    Falvours:flavours,
    fluidcost:f64,
}
 //Structure with two diffrent fields are created 

fn printing_function(drink:drink_info){

    match drink.Falvours
     { 
        flavours::chocolate =>println!("Chocolate"),
        flavours::coffee =>println!("coffee"),
        flavours::vanilla =>println!("Vanilla flavour"),
        flavours::rasberry_with_honey =>println!("Raspberry_with_honey"),
        _ =>println!("Hey this is all the drinks that we have"),
     }  

     println!("The cost of the drinks are here :{}",drink.fluidcost);
     println!("")
    }
//Using the enums

fn main(){

    let one= drink_info{
        Falvours:flavours::chocolate,
        fluidcost:32.02,
    } ;

    let two =drink_info{
        Falvours:flavours::vanilla,
        fluidcost:67.32,
    };

    printing_function(one);
    printing_function(two);

//calling the functions
    }

    

