use std::io;

fn main() {
    //so we want to execute main,
    //then the program allocates 0 tasks initially
    //while in program, type n to create new task,
    // and type d to delete a task
    // f to cross a task bcuz youre done with it

    let mut number_of_tasks = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mode_input = input.trim();

        if mode_input == "n" {
            create_task();
            number_of_tasks += 1;
            println!("Task {}: {}", number_of_tasks, input);
        } else if mode_input == "d" {
        } else if mode_input == "f" {
        } else if mode_input == "--help" {
            println!("Welome to Todolist version 1");
            println!("Available commands:");
            println!("\"n\": Create new ToDo");
            println!("\"d\": Delete ToDo");
            println!("\"f\": Cross out ToDo");
        } else {
            println!("Command not recognised. Enter --help for more info on usable commands.");
        }
    }
}

fn create_task() {
    println!("Enter ToDo");
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
}
