use std::process::Command;
use std::io;
fn main() {
    clear_screen();
    display_menu();
    
    let mut todo_list:Vec<String>= Vec::new();

    loop {
     let mut raw_input  = String::new();
     io::stdin()
        .read_line(&mut raw_input)
        .expect("Could not take input");
    let trimmed_input = raw_input.trim();
    clear_screen();
    display_menu();
    
    match trimmed_input {
        "1" => {
            if todo_list.len() == 0 {
                clear_screen();
                display_menu();
                println!("List Is empty");
            }
            else{
                clear_screen();
                display_menu();
                println!("=============");
                for (index, item) in todo_list.iter().enumerate(){
                    println!("{}. {}", index+1, item);
                }
                println!("=============");
            }
        }

        "2" => {
            println!("Enter Task to add:");
            let mut raw_task = String::new();
            io::stdin()
                .read_line(&mut raw_task)
                .expect("Could not add task");
            let task = raw_task.trim().to_string();
            todo_list.push(task);
            clear_screen();
            display_menu();
            println!("Task added successfully");
        }
        
        "3" => {
            println!("Remove")
        },

        "4" => break,

        _ => println!("invalid input")
    }
    }
}

fn clear_screen(){
    Command::new("cmd")
        .args(["/C", "cls"])
        .status()
        .expect("Failed to clear screen");

}

fn display_menu(){
    println!("Rust Todo List");
    println!("===============");
    println!("Enter what you want to do:\n\n1)View Current Todo's\n2)Add Task\n3)Remove Task\n4)Exit\n");
}