use clap::{Arg, Command};
use std::process::Command as ProcessCommand;

fn main() {
    let matches = Command::new("CLI Game")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI game to manage users and groups")
        .subcommand(Command::new("create_user")
            .about("Creates a new user")
            .arg(Arg::new("username")
                .help("The username to create")
                .required(true)
                .index(1)))
        .subcommand(Command::new("delete_user")
            .about("Deletes an existing user")
            .arg(Arg::new("username")
                .help("The username to delete")
                .required(true)
                .index(1)))
        .subcommand(Command::new("create_group")
            .about("Creates a new group")
            .arg(Arg::new("groupname")
                .help("The group name to create")
                .required(true)
                .index(1)))
        .subcommand(Command::new("add_user_to_group")
            .about("Adds a user to a group")
            .arg(Arg::new("username")
                .help("The username to add")
                .required(true)
                .index(1))
            .arg(Arg::new("groupname")
                .help("The group name to add the user to")
                .required(true)
                .index(2)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create_user") {
        let username = matches.get_one::<String>("username").unwrap();
        create_user(username);
    } else if let Some(matches) = matches.subcommand_matches("delete_user") {
        let username = matches.get_one::<String>("username").unwrap();
        delete_user(username);
    } else if let Some(matches) = matches.subcommand_matches("create_group") {
        let groupname = matches.get_one::<String>("groupname").unwrap();
        create_group(groupname);
    } else if let Some(matches) = matches.subcommand_matches("add_user_to_group") {
        let username = matches.get_one::<String>("username").unwrap();
        let groupname = matches.get_one::<String>("groupname").unwrap();
        add_user_to_group(username, groupname);
    }
}

fn create_user(username: &str) {
    let output = ProcessCommand::new("sudo")
        .arg("useradd")
        .arg(username)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("User {} created successfully!", username);
    } else {
        eprintln!("Error creating user: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn delete_user(username: &str) {
    let output = ProcessCommand::new("sudo")
        .arg("userdel")
        .arg(username)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("User {} deleted successfully!", username);
    } else {
        eprintln!("Error deleting user: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn create_group(groupname: &str) {
    let output = ProcessCommand::new("sudo")
        .arg("groupadd")
        .arg(groupname)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Group {} created successfully!", groupname);
    } else {
        eprintln!("Error creating group: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn add_user_to_group(username: &str, groupname: &str) {
    let output = ProcessCommand::new("sudo")
        .arg("usermod")
        .arg("-aG")
        .arg(groupname)
        .arg(username)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("User {} added to group {} successfully!", username, groupname);
    } else {
        eprintln!("Error adding user to group: {}", String::from_utf8_lossy(&output.stderr));
    }
}
