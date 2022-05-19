fn main() {
    let name = " Bob Smith \r\n";
    println!("Before trim ");
    println!("length is {}", name.len());
    println!();
    println!("After trim ");
    println!("length is {}", name.trim().len());
}
