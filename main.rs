extern crate sign_up;
use sign_up::sign_up::input_signup;
use std::io;

fn main(){
    println!("Press 1 to Sign Up");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input:i32 = input.trim().parse().unwrap();
    if input ==1 {
        println!("Wellcome To Sign Up");
    use sign_up::sign_up::input_signup;
    input_signup();
    } else {
        println!("Something Do Wrong");
   }
    fn signup01 () {
        let mut SU01 = String::new();
        io::stdin().read_line(&mut SU01);
    }
    fn signup02 () {
        let mut SU02 = String::new();
        io::stdin().read_line(&mut SU02);
    }
    fn signup03 () {
        let mut SU03 = String::new();
        io::stdin().read_line(&mut SU03);
}
}