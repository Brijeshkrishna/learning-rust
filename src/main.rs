use std::io;
use std::io::Write;

fn main() {

    let mut strings = String::from("hello, ");
    let mut name: String = String::new();
    loop {
		
        print!("What your name : ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut name)
            .expect("Unable to read input");

        if name.trim().eq("exit") {
            break;
        }
        strings.push_str(&name.trim());
		strings.push_str(" 👋");
        println!("{}", strings);
		strings.clear();
		name.clear();
    }
}
