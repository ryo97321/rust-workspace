use std::process::Command;
use std::io::{stdout, Write};

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    print!("fileName: ");
    stdout().flush().unwrap();
    let input = getline();
    let file_name = input.trim();

    print!("commit message: ");
    stdout().flush().unwrap();
    let input = getline();
    let commit_message = input.trim();

    let git_add_status = Command::new("git")
                    .arg("add")
                    .arg(file_name)
                    .status()
                    .expect("failed to execute add");

    if git_add_status.success() {
        println!("\n{} added.\n", file_name);
    }

    // let commit_message = format!("add {}", file_name);
    let git_commit_status = Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(commit_message)
                    .status()
                    .expect("failed to execute commit");

    if git_commit_status.success() {
        println!("\n{} commited.\n", file_name);
    }

    let git_push_status = Command::new("git")
                    .arg("push")
                    .status()
                    .expect("failed to execute push");
    
    if git_push_status.success() {
        println!("\n{} pushed.", file_name);
    }
}