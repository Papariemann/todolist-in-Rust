use dirs::home_dir;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Initialize the task list
    let mut task_list: Vec<Task> = Vec::new();

    loop {
        println!("Enter input: (see --help for commands)");
        let input = read_input();

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let tasks = parts[1..].join(" ");

        match command {
            "n" => {
                if tasks.is_empty() {
                    println!("No task provided.");
                } else {
                    let new_tasks: Vec<&str> = tasks.split("&&").collect();
                    let new_task_clone = new_tasks.clone();
                    for task in new_task_clone {
                        let task = create_task(task.trim());
                        task_list.push(task);
                    }
                    println!("Created {} task(s).", new_tasks.len());
                }
            }
            "f" => finish_task(&mut task_list, tasks),
            "d" => delete_task(&mut task_list, tasks),
            "ls" => ls(&task_list),
            "esc" | "q" => break,
            "clear" => clear_screen(),
            "--help" => show_help(),
            "save" => save_file(&task_list).expect("REASON"),
            "load" => {
                if let Ok(loaded_tasks) = load_file() {
                    task_list = loaded_tasks;
                } else {
                    println!("Error loading tasks.");
                }
            }
            _ => println!("Unknown command"),
        }
    }
}

// Function to read input from user
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Create a new task
fn create_task(task_name: &str) -> Task {
    Task {
        task: task_name.to_string(),
        status: String::from("In progress"),
    }
}

// List all tasks
fn ls(task_list: &[Task]) {
    if task_list.is_empty() {
        println!("No tasks.");
    } else {
        for (i, task) in task_list.iter().enumerate() {
            println!("{}. {} [{}]", i + 1, task.task, task.status);
        }
    }
}

// Delete a task
fn delete_task(task_list: &mut Vec<Task>, task_input: String) {
    if task_input == "all" {
        task_list.clear();
        println!("All tasks deleted.");
    } else {
        let task_ids: Vec<usize> = parse_task_ids(task_input);
        for task_id in task_ids {
            if task_id <= task_list.len() {
                task_list.remove(task_id - 1);
                println!("Task {} deleted.", task_id);
            }
        }
    }
}

// Mark task as finished
fn finish_task(task_list: &mut Vec<Task>, task_input: String) {
    if task_input == "all" {
        for task in task_list {
            task.status = String::from("Finished");
        }
    } else {
        let task_ids: Vec<usize> = parse_task_ids(task_input);
        for task_id in task_ids {
            if task_id <= task_list.len() {
                task_list[task_id - 1].status = String::from("Finished");
                println!("Task {} marked as finished.", task_id);
            }
        }
    }
}

// Parse task IDs from input
fn parse_task_ids(input: String) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

// Show help information
fn show_help() {
    println!("Welome to Todolist version 1");
    println!("Available commands:");
    println!("\"n\": Create new ToDo (takes more than 1 input with && between inputs)");
    println!("\"d\": Delete ToDo (takes more than 1 input with && between inputs)");
    println!("\"f\": Cross out ToDo (takes more than 1 input with && between inputs) \n");
}

// Clear the screen
fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cls")
            .status()
            .expect("Failed to execute 'cls' command.");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to execute 'clear' command.");
    }
}

// Define Task struct
#[derive(Debug, Clone)]
struct Task {
    task: String,
    status: String,
}

// Ensure the task list directory exists
pub fn ensure_tasklist_dir() -> io::Result<PathBuf> {
    let home = home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;
    let mut dir = home;
    dir.push("tasks");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

// Write tasks to file
pub fn write_tasks_to_file(filename: Option<&str>, tasks: &[Task]) -> io::Result<()> {
    let dir = ensure_tasklist_dir()?;

    let file_name = filename.unwrap_or("tasks.whateverthefuck");
    let mut file_path = dir;
    file_path.push(file_name);

    let mut file = fs::File::create(file_path)?;
    for task in tasks {
        writeln!(file, "{}:{}", task.status, task.task)?;
    }
    Ok(())
}

// Read tasks from file
pub fn read_tasks_from_file(filename: Option<&str>) -> io::Result<Vec<Task>> {
    let dir = ensure_tasklist_dir()?;

    let file_name = filename.unwrap_or("tasks.whateverthefuck");
    let mut file_path = dir;
    file_path.push(file_name);

    if file_path.exists() {
        let content = fs::read_to_string(file_path)?;
        let tasks = content
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, ":");
                let status = parts.next()?.to_string();
                let task = parts.next()?.to_string();
                Some(Task { status, task })
            })
            .collect();
        Ok(tasks)
    } else {
        Ok(Vec::new())
    }
}

// Save tasks to file
fn save_file(task_list: &[Task]) -> std::io::Result<()> {
    write_tasks_to_file(None, task_list)?;
    println!("Tasks saved successfully!");
    Ok(())
}

// Load tasks from file
fn load_file() -> std::io::Result<Vec<Task>> {
    let loaded_tasks = read_tasks_from_file(None)?;
    println!("Loaded tasks: {:?}", loaded_tasks);

    Ok(loaded_tasks)
}
