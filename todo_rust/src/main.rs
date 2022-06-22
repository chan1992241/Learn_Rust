/*
 * What is the difference between
 * (todo: &ToDo) and (todos: &mut Vec<ToDo>) and (mut file: &File)?
 *  */
use chrono::prelude::*;
use std::{
    fs::{write, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
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

fn clean_todo_data_txt(path: &Path) {
    write(path, "").expect("Failed to write to file");
}

fn update_todo_to_file(todos: &Vec<ToDo>, mut file: &File, path: &Path) {
    clean_todo_data_txt(path);
    for (index, todo) in todos.iter().enumerate() {
        write!(file, "{}\n", convert_todo_to_string(index as u8, todo))
            .expect("Failed to write to file");
    }
}

fn convert_todo_string_to_todo(todo_string: &str) -> ToDo {
    let todo_string_split: Vec<&str> = todo_string.split("|").collect();
    let todo_title = todo_string_split[2]
        .replace("To do title:", "")
        .trim()
        .to_string();
    let todo_completed = todo_string_split[0]
        .replace("1. Status:", "")
        .trim()
        .to_string()
        == "DONE";
    let todo_create_at = todo_string_split[1]
        .replace("Created At: ", "")
        .trim()
        .to_string();
    return ToDo {
        title: todo_title,
        completed: todo_completed,
        create_at: todo_create_at.to_string(),
    };
}

fn load_data_to_vec(file: &mut File, todos: &mut Vec<ToDo>) {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let tododetail = line.unwrap();
        todos.push(convert_todo_string_to_todo(&tododetail));
    }
}

fn main() {
    let mut run = true;
    let path = Path::new(".\\src\\todos.txt");
    if !path.exists() {
        write(path, "").expect("Failed to write to file");
    }
    let mut todos: Vec<ToDo> = Vec::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");
    load_data_to_vec(&mut file, &mut todos);
    while run != false {
        println!("All to do: \n");
        for (index, todo) in todos.iter().enumerate() {
            println!("{}", convert_todo_to_string(index as u8, todo));
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
                title,
                completed: false,
                create_at: Utc::now().to_string(),
            };
            todos.push(new_to_do);
            update_todo_to_file(&todos, &file, &path);
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
                update_todo_to_file(&todos, &file, &path);
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
                update_todo_to_file(&todos, &file, &path);
            }
            println!("")
        } else {
            println!("Invalid action");
        }
    }
}
