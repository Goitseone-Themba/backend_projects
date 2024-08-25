/// author: Goitseone Themba
///

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Customer {
    id: u32,
    name: String,
    status: String,
}

struct Custy {
    data_file: String,
}

impl Custy {
    fn new() -> Self {
        Custy {
            data_file: String::from("customers.json"),
        }
    }

    fn read_customers(&self) -> Vec<Customer> {
        match fs::File::open(&self.data_file) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new(),
        } 
    }

    fn write_customers(&self, customers: &Vec<Customer>) {
        let json = serde_json::to_string_pretty(&customers).unwrap();
        let mut file = fs::File::create(&self.data_file).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    fn add(&self, name: &str) {
        let mut customers = self.read_customers();
        let new_id = customers.iter().map(|c| c.id).max().unwrap_or(0) + 1;
        let new_customer = Customer {
            id: new_id,
            name: name.to_string(),
            status: String::from("waiting"),
        };
        customers.push(new_customer);
        self.write_customers(&customers);
        println!("{} added successfully, id: {:02}", name, new_id);
    }
}

fn main() {
    let custy = Custy::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <command> <argument>", args[0]);
        return;
    }

    let command = &args[1];
    let arg = &args[2];

    match command.as_str() {
        "add" => custy.add(arg),
        _=> println!("Unknown command"),
    }
}
