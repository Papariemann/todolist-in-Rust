use std::io;
use std::process::Command;

fn main() {
    //so we want to execute main,
    //then the program allocates 0 tasks initially
    //while in program, type n to create new task,
    // and type d to delete a task
    // f to cross a task bcuz youre done with it
    // init an array/vector for the tasks that have been created
    // I will jump off a bridge
    // kill me
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
            "esc" => break,
            "q" => break,
            "clear" => clear_screen(),
            "--help" => show_help(),
            _ => println!("Unknown command"),
        }
    }
}

// made this into a function because it was annoying to write the same shit every time I needed
// this functionality
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// creates a task
fn create_task(task_name: &str) -> Task {
    Task {
        task: task_name.to_string(),
        status: String::from("In progress"),
    }
}

// lists all tasks present in task_list
fn ls(task_list: &[Task]) {
    if task_list.is_empty() {
        println!("No tasks.");
    } else {
        for (i, task) in task_list.iter().enumerate() {
            println!("{}. {} [{}]", i + 1, task.task, task.status);
        }
    }
}

//...fucking deletes a task what more needst thou know
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

// marks task.status as "Finished"
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

// converts input into the location of the task in the task_list vec
fn parse_task_ids(input: String) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

// pretty self explanatory I guess
fn show_help() {
    println!("Welome to Todolist version 1");
    println!("Available commands:");
    println!("\"n\": Create new ToDo (takes more than 1 input with && between inputs)");
    println!("\"d\": Delete ToDo (takes more than 1 input with && between inputs)");
    println!("\"f\": Cross out ToDo (takes more than 1 input with && between inputs) \n");
}

// performs clear on the terminal in order to clear the current contents displayed
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

// task type for making my life both easier and more difficult
struct Task {
    task: String,
    status: String,
}
