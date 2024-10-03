# task-cli

My solution to https://roadmap.sh/projects/task-tracker from https://roadmap.sh/

A command-line interface (CLI) application for managing tasks, written in Rust.

## Features

- Add new tasks
- List all tasks
- List tasks by status (todo, in-progress, done)
- Update task descriptions
- Delete tasks
- Mark tasks as in-progress or done
- Persistent storage using JSON

## Requirements

- Rust programming language
- Cargo package manager

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/Goitseone-Themba/backend_projects.git
   cd backend_projects/Task-Tracker/task-cli
   ```

2. Build the application:
   ```
   cargo build --release
   ```

3. The executable will be available in the `target/release` directory.

## Usage

### Add a new task
```
./task-cli add "Your task description"
```

### List all tasks
```
./task-cli list
```

### List tasks by status
```
./task-cli list todo
./task-cli list in-progress
./task-cli list done
```

### Update a task
```
./task-cli update <task_id> "New task description"
```

### Delete a task
```
./task-cli delete <task_id>
```

### Mark a task as in-progress
```
./task-cli mark-in-progress <task_id>
```

### Mark a task as done
```
./task-cli mark-done <task_id>
```

## Data Storage

Tasks are stored in a JSON file named `task_list.json` in the same directory as the executable.

## Dependencies

- chrono: For handling dates and times
- serde: For serializing and deserializing JSON data
- serde_json: For working with JSON

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).
