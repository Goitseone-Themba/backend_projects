use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]

// data model for each task
struct Task {
    id: u32,
    description: String,
    status: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}

// data model for the json file
struct TaskCli {
    json_filename: String,
}

impl TaskCli {
    fn new() -> Self {
        TaskCli {
            json_filename: String::from("task_list.json"),
        }
    }

    // read from the json file
    fn read_json(&self) -> Vec<Task> {
        let file_result = File::open(&self.json_filename);

        match file_result {
            Ok(mut file) => {
                let mut file_content: String = String::new();
                file.read_to_string(&mut file_content).unwrap();
                serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new(),
        }
    }

    //write to the json file
    fn write_json(&self, tasks: &Vec<Task>) {
        let json_string = serde_json::to_string_pretty(&tasks).unwrap();
        let mut file = File::create(&self.json_filename).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
    }

    //adding a new task
    fn add(&self, description: String) {
        let mut tasks = self.read_json();
        let new_task: Task = Task {
            id: u32::try_from(tasks.len() + 1).unwrap(),
            description,
            status: String::from("to_do"),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        tasks.push(new_task);
        self.write_json(&tasks);
        println!("Tasks added successfully (ID: {})", tasks.len());
    }
}

fn main() {
    let task_cli: TaskCli = TaskCli::new();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("usage: {} <command> <argument>", args[0]);
        return;
    }

    let command = &args[1];
    let argument = &args[2];

    match command.as_str() {
        "add" => {
            if args.len() != 3 {
                println!("add usage: {} add <your task>", &args[0]);
                return;
            }
            task_cli.add(String::from(argument))
        }
        _ => println!("command: \"{}\" not found", command),
    }
}
