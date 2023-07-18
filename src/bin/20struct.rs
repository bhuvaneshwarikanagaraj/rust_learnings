enum flavours {

chocolate,
vanilla,
rasberry_with_honey,
coffee
  
}

struct drink_info{

    Falvours:flavours,
    fluidcost:f64,
}

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

}


fn main(){

    let one{    

    };

    



}

