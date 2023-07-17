// Arrays in Rust

fn main(){
let arr =[1,2,3,4,5,6,7,8,9];
let mut index=0;

loop{
   if arr[index]%2==0 {
      index+=1;
      continue;
   }
   if arr[index]==9 {
      break;
   }
   println!("{}",arr[index]);
   index +=1;

}
}