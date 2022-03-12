
fn main(){
    let mut hex:String  = String::new(); 
    loop {
        println!("Enter your name : ");
        std::io::stdin().read_line(&mut hex).unwrap();
        if hex.eq("exit") {
            break;
        }
        println!("Hello 👋 {} ",hex);

    }
}