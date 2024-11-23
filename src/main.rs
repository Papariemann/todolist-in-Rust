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
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mode_input = input.trim();

        if mode_input == "n" {
            let task = create_task();
            let task_clone = task.task.clone();
            task_list.push(task); // adding it to the array/vector of tasks
            println!("Task {}: {}", task_list.len(), task_clone);
        } else if mode_input == "d" {
            for (i, task) in task_list.iter().enumerate() {
                println!("{}. {} [{}]", i + 1, task.task, task.status);
            }

            println!("Enter ToDo to delete: (Enter ToDo contents or ToDo index)");
            let mut todo_to_del = String::new();
            io::stdin().read_line(&mut todo_to_del).unwrap();
            let del_todo = todo_to_del.trim();

            if let Ok(index) = del_todo.parse::<usize>() {
                if index > 0 && index <= task_list.len() {
                    // Adjust cuz list begins at task 1 but task_list begins at 0
                    let pos = index - 1;
                    task_list.remove(pos);
                } else {
                    println!("Invalid index. ");
                }
            } else {
                if let Some(pos) = task_list.iter().position(|x| x.task == del_todo) {
                    task_list.remove(pos);
                } else {
                    println!("Task not found. ");
                }
            }
        } else if mode_input == "ls" {
            ls(&task_list);
        } else if mode_input == "f" {
            // if input in "normal" mode is f,
            // a task like "go shower: in progress" -> "finished"
            println!("Choose task to mark as completed: (Enter ToDo contents or ToDo index)");
            ls(&task_list);
            let mut change_status_input = String::new();
            io::stdin().read_line(&mut change_status_input).unwrap();

            if let Ok(index) = change_status_input.trim().parse::<usize>() {
                if index > 0 && index <= task_list.len() {
                    let pos = index - 1;
                    task_list[pos].status = String::from("Finished");
                    ls(&task_list);
                } else {
                    println!("Invalid index. ");
                }
            } else {
                if let Some(pos) = task_list
                    .iter_mut()
                    .position(|x| x.task == change_status_input.trim())
                {
                    task_list[pos].status = String::from("Finished");
                    ls(&task_list);
                }
            }
        } else if mode_input == "--help" {
            println!("Welome to Todolist version 1");
            println!("Available commands:");
            println!("\"n\": Create new ToDo");
            println!("\"d\": Delete ToDo");
            println!("\"f\": Cross out ToDo\n");
        } else {
            println!("Command not recognised. Enter --help for more info on usable commands.");
        }
    }
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

struct Task {
    task: String,
    status: String,
}
