use chrono::prelude::*;
use std::io;

#[derive(Debug)]
struct ToDo {
    title: String,
    completed: bool,
    create_at: String,
}

fn main() {
    let mut run = true;
    let mut todos: Vec<ToDo> = Vec::new();
    while run != false {
        println!("All to do: \n");
        if todos.is_empty() {
            println!("NO TO DO");
        } else {
            for (index, todo) in todos.iter().enumerate() {
                println!(
                    "{}. Status: {} | Created At: {} | To do title: {}",
                    &index + 1,
                    todo.completed,
                    todo.create_at,
                    todo.title
                );
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
            todos.push(ToDo {
                title: title,
                completed: false,
                create_at: Utc::now().to_string(),
            });
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
