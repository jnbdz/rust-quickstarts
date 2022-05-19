fn main() {
   let msg = "Bob Smith is amazing".to_string();
   let mut i = 1;
   
   for token in msg.split_whitespace(){
      println!("token {} {}", i, token);
      i+=1;
   }
}
