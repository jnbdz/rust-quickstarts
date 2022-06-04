#[derive(Debug)]
enum User {
   ID(i32),
   Name(String)
}

fn main() {
   let u1 = User::Name(String::from("Jack Smith"));
   let u2 = User::ID(100);
   println!("{:?}", u1);
   println!("{:?}", u2);

   match u1 {
      User::Name(val) => {
         println!("{}", val);
      }
      User::ID(val) => {
         println!("{}", val);
      }
   }
}
