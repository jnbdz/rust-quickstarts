fn main() {
   let mut num:i32 = 10;
   mutate_num(&mut num);
   println!("The value of num is: {}", num);
}

fn mutate_num(param_num:&mut i32) {
   *param_num = 0; // reference
}
