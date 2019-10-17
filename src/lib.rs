use std::io;
pub fn input_signup () {
    let mut input_suform = String::new();
    println!("Enter Your Username");
    io::stdin().read_line(&mut input_suform);
    let mut input_suform01 = String::new();
    println!("Enter Your Mobile Number");
    io::stdin().read_line(&mut input_suform01);
    let mut input_suform02 = String::new();
    println!("Enter Your Password");
    io::stdin().read_line(&mut input_suform02);
    println!("Signup Successfully. !!\nYour Login ID is:\nUsername :{}\nNumber:{}\nPassword:{}",input_suform,input_suform01,input_suform02);
}