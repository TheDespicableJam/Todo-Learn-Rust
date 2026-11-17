use std::process::Command;
use std::io;
use std::{thread, time};
fn main() {
    let t = 1;
    clear_screen();
    display_menu();
    
    let mut todo_list:Vec<String>= Vec::new();

    loop {
    let raw = input();
    let main_screen_input = raw.parse().unwrap();
    clear_screen();
    display_menu();
    
    match main_screen_input{
        1 => {
            view_list(&todo_list, t);
        }

        2 => {
            add_task(&mut todo_list, t);
        }
        
        3 => {
            remove_task(&mut todo_list, t);
        }

        4 => break,

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
    loop {
        println!("Enter Task to add:");
        let task = input();
        if task.len() != 0{
            todo_list.push(task);
            clear_screen();
            display_menu();
            println!("Task added successfully");
            sleep(t);
            clear_screen();
            display_menu();
            break;
        }
        else{
            println!("Cannot take an empty task!")
        }    
    }
    
    
}

fn remove_task(todo_list: &mut Vec<String>, t: u64){
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
                let raw_inp = input();
                match raw_inp.parse::<usize>(){
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

fn input() -> String{
    let mut input_var = String::new();
    io::stdin()
        .read_line(&mut input_var)
        .expect("Error occured while taking input");
    let return_val = input_var.trim().to_string();
    return_val
}