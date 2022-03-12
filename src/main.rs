
fn main(){
    let mut name:String  = String::new(); 
    loop {
        println!("Enter your name : ");
        std::io::stdin().read_line(&mut name).unwrap();
        if name.eq("exit") {
            break;
        }
        println!("Hello 👋 {} ",name);

    }
}