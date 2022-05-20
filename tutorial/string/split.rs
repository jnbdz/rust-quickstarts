fn main() {
   let user = "Bob, Smith, BigBurgers";

   for token in user.split(","){
      println!("token is {}",token);
   }

   // Vector (stored)
   println!("\n");
   let details:Vec<&str>= user.split(",").collect();
   println!("firstName is {}", details[0]);
   println!("lastname is {}", details[1]);
   println!("company is {}", details[2]);
}
