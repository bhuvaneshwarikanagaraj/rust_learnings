
fn main(){
   let age1 : i32 = 21;
   match age1 {
      1..=18 =>println!("Important Birthday"),
      21 |30 =>println!("Important Birthday"),
      65..=i32::MAX =>println!("Important birthday"),
      _=> println!("Not that much Important"),   
   };


}