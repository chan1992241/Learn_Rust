use chrono::prelude::*;
use std::{
    fmt::write,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Error, Read, Write},
    path::{self, Path},
};

#[derive(Debug)]
struct ToDo {
    title: String,
    completed: bool,
    create_at: String,
}

fn convert_todo_to_string(index: u8, todo: &ToDo) -> String {
    let mut status = "DONE";
    if todo.completed == false {
        status = "NOT DONE";
    }
    return format!(
        "{}. Status: {} | Created At: {} | To do title: {}",
        index + 1,
        status,
        todo.create_at,
        todo.title
    );
}

fn update_todo_to_file(todos: &Vec<ToDo>, mut file: &File) {
    file.write_all("".as_bytes())
        .expect("Failed to write to file");
    for (index, todo) in todos.iter().enumerate() {
        write!(file, "{}\n", convert_todo_to_string(index as u8, todo))
            .expect("Failed to write to file");
    }
}

fn main() {
    let mut run = true;
    let path = Path::new(".\\src\\todos.txt");
    let mut todos: Vec<ToDo> = Vec::new();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");
    while run != false {
        println!("All to do: \n");
        if todos.is_empty() {
            println!("NO TO DO");
        } else {
            for (index, todo) in todos.iter().enumerate() {
                println!("{}", convert_todo_to_string(index as u8, todo));
            }
        }
        println!("\nAll action\n");
        println!("1. Add To Do");
        println!("2. Delete To Do");
        println!("3. Mark To Do as completed");
        println!("4. Exit");
        println!("Enter your action: ");
        let mut action: String = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        if action.trim() == "4" {
            run = false;
        } else if action.trim() == "1" {
            println!("Enter To Do title: ");
            let mut title: String = String::new();
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");
            title = title.trim().to_string();
            let new_to_do = ToDo {
                title: title,
                completed: false,
                create_at: Utc::now().to_string(),
            };
            // write!(
            //     &file,
            //     "{}\n",
            //     convert_todo_to_string(todos.len() as u8, &new_to_do)
            // )
            // .expect("Failed to write to file");
            todos.push(new_to_do);
            update_todo_to_file(&todos, &file);
            println!("")
        } else if action.trim() == "2" {
            println!("Enter To Do index: ");
            let mut index: String = String::new();
            io::stdin()
                .read_line(&mut index)
                .expect("Failed to read line");
            let index: usize = index.trim().parse().expect("Please enter a number");
            if index > todos.len() {
                println!("Index out of range");
            } else {
                todos.remove(index - 1);
                update_todo_to_file(&todos, &file);
            }
            println!("")
        } else if action.trim() == "3" {
            println!("Enter To Do index: ");
            let mut index: String = String::new();
            io::stdin()
                .read_line(&mut index)
                .expect("Failed to read line");
            let index: usize = index.trim().parse().expect("Please enter a number");
            if index > todos.len() {
                println!("Index out of range");
            } else {
                todos[index - 1].completed = true;
            }
            println!("")
        } else {
            println!("Invalid action");
        }
    }
}
