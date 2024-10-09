use std::env;

    fn get_activity(username: &String) {
        println!("fetching {}'s activity...",username); 
    }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        println!("Usage: github-activity <username>");
        return;
    }

    let username: &String = &args[1];
    
    get_activity(username);
}
