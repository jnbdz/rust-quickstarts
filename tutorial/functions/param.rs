fn main(){
   let num:i32 = 10;
   mutate(num);
   println!("The value of num is: {}", num);
}

fn mutate(mut param_num: i32) {
   param_num = param_num*0;
   println!("param_num value is: {}", param_num);
}
