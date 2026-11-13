use std::process::Command;
use std::io;
use std::{thread, time};
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
            clear_screen();
            display_menu();
            if todo_list.is_empty() {
                
                println!("List Is empty");
                sleep(2);
                clear_screen();
                display_menu();
            }
            else{
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
            sleep(2);
            clear_screen();
            display_menu();
        }
        
        "3" => {
            let mut raw_inp = String::new();
            clear_screen();
            display_menu();
            if todo_list.is_empty(){
                println!("No Tasks in the List");
                sleep(2);
                clear_screen();
                display_menu();
            }
            else {
                println!("=============");
                for (index, item) in todo_list.iter().enumerate(){
                    println!("{}. {}", index+1, item);
                }
                println!("=============");
                println!("Which Task should be removed? (Enter Serial No.)");
                io::stdin()
                    .read_line(&mut raw_inp)
                    .expect("An Error occured while removing the task");
                let remove_task: usize = raw_inp.trim().parse().expect("Invalid number");
                let mut opt = 0;
                let mut removable = false;
                for index in todo_list.iter().enumerate(){
                    if index.0 + 1  == remove_task{
                        opt = index.0;
                        removable = true;
                    }
                }
                if removable == true{
                    todo_list.remove(opt);
                    println!("Task removed successfully");
                    sleep(2);
                    clear_screen();
                    display_menu();
                }
                else{
                    println!("No task found with that index");
                    sleep(2);
                    clear_screen();
                    display_menu();
                }
            }
        },

        "4" => break,

        _ => {
            println!("Invalid Input");
            sleep(2);
            clear_screen();
            display_menu();
        }
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

fn sleep(time:u64){
    thread::sleep(time::Duration::from_secs(time));
}