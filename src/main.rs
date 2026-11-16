use std::process::Command;
use std::io;
use std::{thread, time};
fn main() {
    let t = 1;
    clear_screen();
    display_menu();
    
    let mut todo_list:Vec<String>= Vec::new();

    loop {
     let mut main_screen_inp  = String::new();
     io::stdin()
        .read_line(&mut main_screen_inp)
        .expect("Could not take input");
    let trimmed_input = main_screen_inp.trim();
    clear_screen();
    display_menu();
    
    match trimmed_input {
        "1" => {
            view_list(&todo_list, t);
        }

        "2" => {
            add_task(&mut todo_list, t);
        }
        
        "3" => {
            remove_task(&mut todo_list, t);
        }

        "4" => break,

        _ => {
            println!("Invalid Input");
            sleep(t);
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

fn view_list(todo_list: &Vec<String>, t: u64){
    clear_screen();
    display_menu();
    if todo_list.is_empty() {      
        println!("List Is empty");
        sleep(t);
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

fn add_task(todo_list: &mut Vec<String>, t: u64){
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
    sleep(t);
    clear_screen();
    display_menu();
}

fn remove_task(todo_list: &mut Vec<String>, t: u64){
    let mut raw_inp = String::new();
        clear_screen();
        display_menu();
        if todo_list.is_empty(){
            println!("No Tasks in the List");
            sleep(t);
            clear_screen();
            display_menu();
        }
        else {
            loop{
                println!("=============");
                for (index, item) in todo_list.iter().enumerate(){
                    println!("{}. {}", index+1, item);
                    }
                println!("=============");
                println!("Which Task should be removed? (Enter Serial No.)");
                raw_inp.clear();
                io::stdin()
                    .read_line(&mut raw_inp)
                    .expect("An Error occured while removing the task");
                
                match raw_inp.trim().parse::<usize>(){
                    Ok(number) => {
                        if number > 0 && number <= todo_list.len(){
                            todo_list.remove(number-1);
                            clear_screen();
                            display_menu();
                            break;
                        }
                        else{
                            clear_screen();
                            display_menu();
                            println!("Invalid index number, Try Again");
                        }
                    }

                    Err(error) => {
                        clear_screen();
                        display_menu();
                        println!("An error occured: {}, Enter a Valid Value", error)
                    }
                }
            }
        }
}