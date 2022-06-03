enum Gender {
    Female,
    Male
}

fn print_gender(g:Gender) {
    match g {
        Gender::Female => {
            println!("It's a female!");
        }
        Gender::Male => {
            println!("It's a male!");
        }
    }
}

fn main() {
    print_gender(Gender::Female);
    print_gender(Gender::Male);
}
