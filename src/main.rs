use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs,
    io::{self, Read, Write},
    vec,
};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

const FILE: &str = "data.json";

// Load task from file
fn load_tasks() -> Vec<Task> {
    fs::read_to_string(FILE)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| vec![])
}

fn save_tasks(tasks: &Vec<Task>) {
    if let Ok(json) = serde_json::to_string_pretty(tasks) {
        fs::write(FILE, json).expect("Failed to save tasks")
    }
}

fn display_tasks(tasks: &Vec<Task>) {
    println!("\n=== To-Do List ===");
    for task in tasks {
        let status = if task.done { "[✔]" } else { "[]" };
        println!("{} {} - {}", task.id, task.done, task.description);
    }
    println!();
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = tasks.len() + 1;
    tasks.push(Task {
        id,
        description,
        done: false,
    });
    save_tasks(tasks);
}

fn mark_done(tasks: &mut Vec<Task>, id: usize) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
        save_tasks(tasks);
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: usize) {
    tasks.retain(|t| t.id != id);
    save_tasks(tasks);
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("\n1. Add Task\n2. View Tasks\n3. Mark Done\n4. Delete Task\n5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                add_task(&mut tasks, desc.trim().to_string());
            }
            "2" => display_tasks(&tasks),
            "3" => {
                print!("Enter task ID to mark as done: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    mark_done(&mut tasks, id);
                }
            }
            "4" => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    delete_task(&mut tasks, id);
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, try again!"),
        }
    }
}
