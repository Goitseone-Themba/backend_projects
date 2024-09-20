use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
// data model for each task
struct Task {
    id: u32,
    description: String,
    status: String,
    created_at: String,
    updated_at: String,
}

// data model for the json file
struct TaskList {
    json_filename: String,
}

impl TaskList {
    fn new() -> Self {
        TaskList {
            json_filename: String::from("task_list.json"),
        }
    }

    // read from the json file
    fn read(&self) -> Vec<Task> {
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
    fn write(&self, tasks: &Vec<Task>) {
        let json_string = serde_json::to_string_pretty(&tasks).unwrap();
        let mut file = File::create(&self.json_filename).unwrap();
        file.write_all(json_string.as_bytes()).unwrap(); 
    }

    // fn add(&self, description: String) {}
}

fn main() {
    let task_list: TaskList = TaskList::new();

    println!("{}", task_list.json_filename);

}
