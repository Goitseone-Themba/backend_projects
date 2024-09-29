use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Read, Write};

// data model for each task
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
        let file_result: Result<File, Error> = File::open(&self.json_filename);

        match file_result {
            Ok(mut file) => {
                let mut file_content: String = String::new();
                file.read_to_string(&mut file_content).unwrap();
                serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new(),
        }
    }

    // write to the json file
    fn write_json(&self, tasks: &Vec<Task>) {
        let json_string: String = serde_json::to_string_pretty(&tasks).unwrap();
        let mut file: File = File::create(&self.json_filename).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
    }

    // add a new task
    fn add(&self, description: String) {
        let mut tasks: Vec<Task> = self.read_json();
        let this_time: DateTime<Utc> = Utc::now();

        println!("{}", this_time.format("%d-%b-%y %H:%M:%S"));

        let new_task: Task = Task {
            id: u32::try_from(tasks.len() + 1).unwrap(),
            description,
            status: String::from("todo"),
            created_at: this_time,
            updated_at: this_time,
        };
        tasks.push(new_task);
        self.write_json(&tasks);
        println!("Tasks added successfully (ID: {})", tasks.len());
    }

    // list all tasks
    fn list(&self) {
        let tasks: Vec<Task> = self.read_json();
        if tasks.len() == 0 {
            println!("There are currently no tasks");
        } else {
            for i in tasks.iter() {
                println!(
                    "{} {}, status: {}, {}, {}",
                    i.id,
                    i.description,
                    i.status,
                    i.created_at.format("%d-%b-%y %H:%M:%S"),
                    i.updated_at.format("%d-%b-%y %H:%M:%S")
                );
            }
        }
    }

    fn list_done(&self) {
        let tasks: Vec<Task> = self.read_json();
        if tasks.len() == 0 {
            println!("There are currently no tasks");
        } else {
            let mut done: Vec<Task> = Vec::new();
            for i in tasks.iter() {
                match i.status.as_str() {
                    "done" => {
                        done.push(i.clone());
                    }
                    _ => (),
                }
            }
            if done.len() > 0 {
                for i in done.iter() {
                    println!(
                        "{} {}, status: {}, {}, {}",
                        i.id,
                        i.description,
                        i.status,
                        i.created_at.format("%d-%b-%y %H:%M:%S"),
                        i.updated_at.format("%d-%b-%y %H:%M:%S")
                    );
                }
            } else {
                println!("There are no completed tasks");
            }
        }
    }

    fn list_todo(&self) {
        let tasks: Vec<Task> = self.read_json();
        if tasks.len() == 0 {
            println!("There are currently no tasks");
        } else {
            let mut todo: Vec<Task> = Vec::new();
            for i in tasks.iter() {
                match i.status.as_str() {
                    "todo" => {
                        todo.push(i.clone());
                    }
                    _ => (),
                }
            }
            if todo.len() > 0 {
                for i in todo {
                    println!(
                        "{} {}, status: {}, {}, {}",
                        i.id,
                        i.description,
                        i.status,
                        i.created_at.format("%d-%b-%y %H:%M:%S"),
                        i.updated_at.format("%d-%b-%y %H:%M:%S")
                    );
                }
            } else {
                println!("There are currently no pending tasks");
            }
        }
    }

    fn list_in_progress(&self) {
        let tasks: Vec<Task> = self.read_json();
        if tasks.len() == 0 {
            println!("There are currently no tasks");
        } else {
            let mut in_progress: Vec<Task> = Vec::new();
            for i in tasks.iter() {
                match i.status.as_str() {
                    "in-progress" => {
                        in_progress.push(i.clone());
                    }
                    _ => (),
                }
            }
            if in_progress.len() > 0 {
                for i in in_progress.iter() {
                    println!(
                        "{} {}, status: {}, {}, {}",
                        i.id,
                        i.description,
                        i.status,
                        i.created_at.format("%d-%b-%y %H:%M:%S"),
                        i.updated_at.format("%d-%b-%y %H:%M:%S")
                    );
                }
            } else {
                println!("There are currently no in progress tasks");
            }
        }
    }

    fn update(&self,id: &u32, description: &String) {
        let mut tasks: Vec<Task> = self.read_json();
        for i in &mut tasks {
            if i.id == *id {
                i.description = description.to_string();
            }
        }
        self.write_json(&tasks);
    }
}

fn main() {
    let task_cli: TaskCli = TaskCli::new();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: {} <command> <argument>", args[0]);
        return;
    }

    let command: &String = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() != 3 {
                println!("add usage: {} add <your task>", &args[0]);
                return;
            }
            task_cli.add(String::from(&args[2]))
        }
        "list" => {
            if args.len() < 3 {
                task_cli.list();
            } else {
                match args[2].as_str() {
                    "done" => task_cli.list_done(),
                    "todo" => task_cli.list_todo(),
                    "in-progress" => task_cli.list_in_progress(),
                    _ => task_cli.list(),
                }
            }
        },
        "update" => {
            if args.len() < 4 {
                println!("update usage: {} update <id> <new description>", &args[0]);
                return;
            } else {
                let id: u32 = match &args[2].trim().parse() {
                    Ok(num) => *num,
                    Err(_) => {
                        print!("Error! can't read id: {}", &args[2]);
                        return;
                    },
                };
                task_cli.update(&id, &args[3]);
            }
        },
        _ => println!("command: \"{}\" not found", command),
    }
}
