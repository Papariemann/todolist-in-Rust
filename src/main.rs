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
        let mode_input = read_input().trim().to_string();

        match mode_input.as_str() {
            "n" => {
                let task = create_task();
                let task_clone = task.task.clone();
                task_list.push(task);
                println!("Task {}: {}", task_list.len(), task_clone);
            }
            "d" => {
                ls(&task_list);
                let todo_to_del = read_input().trim().to_string();
                delete_task(&mut task_list, &todo_to_del);
            }
            "ls" => ls(&task_list),
            "f" => {
                ls(&task_list);
                let change_status_input = read_input().trim().to_string();
                finish_task(&mut task_list, &change_status_input);
            }
            "--help" => {
                show_help();
            }
            _ => println!("Command not recognised. Enter --help for more info."),
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Error reading input.");
    }
    input
}

fn create_task() -> Task {
    println!("Enter ToDo: ");
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
    Task {
        task: task.trim().to_string(),
        status: String::from("In progress"),
    }
}

fn ls(task_list: &Vec<Task>) {
    if task_list.is_empty() {
        println!("Task list is empty. ");
    } else {
        for (i, task) in task_list.iter().enumerate() {
            println!("{}. {} [{}]", i + 1, task.task, task.status);
        }
    }
}

fn delete_task(task_list: &mut Vec<Task>, todo_to_del: &str) {
    if let Ok(index) = todo_to_del.parse::<usize>() {
        if index > 0 && index <= task_list.len() {
            task_list.remove(index - 1);
        } else {
            println!("Invalid index.");
        }
    } else if let Some(pos) = task_list.iter().position(|x| x.task == todo_to_del) {
        task_list.remove(pos);
    } else {
        println!("Task not found.");
    }
}

fn finish_task(task_list: &mut Vec<Task>, todo_to_mark: &str) {
    if let Ok(index) = todo_to_mark.parse::<usize>() {
        if index > 0 && index <= task_list.len() {
            task_list[index - 1].status = String::from("Finished");
        } else {
            println!("Invalid index.");
        }
    } else if let Some(pos) = task_list.iter_mut().position(|x| x.task == todo_to_mark) {
        task_list[pos].status = String::from("Finished");
    } else {
        println!("Task not found.");
    }
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
