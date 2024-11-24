use std::io;

fn main() {
    //so we want to execute main,
    //then the program allocates 0 tasks initially
    //while in program, type n to create new task,
    // and type d to delete a task
    // f to cross a task bcuz youre done with it
    // init an array/vector for the tasks that have been created
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
            "ls" => ls(&mut task_list),
            "esc" => break,
            "q" => break,
            "--help" => show_help(),
            _ => println!("Unknown command"),
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn create_task(task_name: &str) -> Task {
    Task {
        task: task_name.to_string(),
        status: String::from("In progress"),
    }
}

fn ls(task_list: &[Task]) {
    if task_list.is_empty() {
        println!("Not tasks.");
    } else {
        for (i, task) in task_list.iter().enumerate() {
            println!("{}. {} [{}]", i + 1, task.task, task.status);
        }
    }
}

fn delete_task(task_list: &mut Vec<Task>, task_input: String) {
    let task_ids: Vec<usize> = parse_task_ids(task_input);
    for task_id in task_ids {
        if task_id <= task_list.len() {
            task_list.remove(task_id - 1);
            println!("Task {} deleted.", task_id);
        }
    }
}

fn finish_task(task_list: &mut Vec<Task>, task_input: String) {
    let task_ids: Vec<usize> = parse_task_ids(task_input);
    for task_id in task_ids {
        if task_id <= task_list.len() {
            task_list[task_id - 1].status = String::from("Finished");
            println!("Task {} marked as finished.", task_id);
        }
    }
}

fn parse_task_ids(input: String) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn show_help() {
    println!("Welome to Todolist version 1");
    println!("Available commands:");
    println!("\"n\": Create new ToDo");
    println!("\"d\": Delete ToDo");
    println!("\"f\": Cross out ToDo\n");
}

struct Task {
    task: String,
    status: String,
}
