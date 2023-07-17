
fn main(){
   let mut _age=47;
   let canvote:bool =if _age>=18{
      true
   }
   else{
      false
   };
   println!("Can vote {}",canvote);
}