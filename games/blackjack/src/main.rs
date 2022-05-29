/*use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use std::io;
use std::cmp::Ordering;*/
//use rand::Rng;
use std::convert::TryInto;
use rand::rngs::mock::StepRng;

fn main() {
    println!("Blackjack!");

	let mut arr_num: [String; 10] = Default::default();
	let mut i = 0;
	let mut n = 2;

    while n <= 11 {
		arr_num[i] = n.to_string();
        n += 1;
		i += 1;
    }

    let arr: [i32; 5] = (1..=5).collect::<Vec<_>>()
            .try_into().expect("wrong size iterator");
    let mut first = StepRng::new(2, 11);
    //let mut first: [&str; 10] = ["2"; "11"];
    let mut ranks: [&str; 4] = ["JACK", "QUEEN", "KING", "ACE"];
    let mut suits: [&str; 4] = ["SPADE", "HEART", "DIAMOND", "CLUB"];

	println!("arr_num is {:?}", arr_num);
    println!("arr is {:?}", arr);
    println!("Ranks is {:?}", first);
    println!("Ranks is {:?}", ranks);
    println!("Suits is {:?}", suits);
}
