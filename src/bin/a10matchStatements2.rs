use std::cmp::Ordering;


fn main(){

   let age:i32=18;
   let voting_age:i32=18;
   match age.cmp(&voting_age)
    {
      Ordering::Less=>println!("Can't vote"),
      Ordering::Greater=>println!("Can Vote"),
      Ordering::Equal=>println!("Congrats you are ready to vote"),   
   };

}
