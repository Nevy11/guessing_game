#[allow(dead_code)]
pub enum Gender {
    Male,
    Female,
    RatherNotSay,
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn take_n_print_gender(gender: Gender) {
    match gender {
        Gender::Male => println!("You are Male"),
        Gender::Female => println!("You are Female"),
        Gender::RatherNotSay => println!("Rather not say"),
    }
}
