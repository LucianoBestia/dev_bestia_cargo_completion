//! automation_tasks_rs with_lib

use cargo_auto_lib::*;

/// automation_tasks_rs with_lib
fn main() {
    if is_not_run_in_rust_project_root_directory() {
        println!("Error: automation_tasks_rs must be called in the root directory of the rust project beside the Cargo.toml file and automation_tasks_rs directory.");
        // early exit
        std::process::exit(0);
    }

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("Running automation task: {}", &task);
                if &task == "build" || &task == "b" {
                    task_build();
                } else if &task == "release" || &task == "r" {
                    task_release();
                } else if &task == "docs" || &task == "doc" || &task == "d" {
                    task_docs();
                } else if &task == "publish_to_crates_io" {
                    task_publish_to_crates_io();
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!("");
    println!("User defined tasks in automation_tasks_rs:");
    println!("cargo auto build - builds the crate in debug mode, fmt");
    println!("cargo auto release - builds the crate in release mode, version from date, fmt");
    println!("cargo auto docs - builds the docs, copy to docs directory");
    println!("cargo auto publish_to_crates_io - publish to crates.io, git tag");
    println!("");
}

/// sub-command for bash auto-completion using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = &args[2];
    let sub_commands = vec!["build", "release", "doc", "publish_to_crates_io"];

    let mut sub_found = false;
    for sub_command in sub_commands.iter() {
        if sub_command.starts_with(word_being_completed) {
            println!("{}", sub_command);
            sub_found = true;
        }
    }
    if sub_found == false {
        // list all sub-commands
        for sub_command in sub_commands.iter() {
            println!("{}", sub_command);
        }
    }
}

// region: tasks

/// example how to call a list of shell commands
fn task_build() {
    #[rustfmt::skip]
    let shell_commands = [
        "cargo fmt", 
        "cargo build"];
    run_shell_commands(shell_commands.to_vec());
    println!("After `cargo auto build`, run the tests and the code. If ok, then `cargo auto release`");
}

/// example how to call one shell command and combine with rust code
fn task_release() {
    auto_version_from_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    println!("$ cargo fmt");
    run_shell_command("cargo fmt");
    println!("$ cargo build --release");
    run_shell_command("cargo build --release");
    println!("After `cargo auto release`, run the tests and the code. If ok, then `cargo auto doc`");
}

/// example how to call a list of shell commands and combine with rust code
fn task_docs() {
    auto_md_to_doc_comments();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items",        
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",package_name().replace("-","_")) ,        
    ];
    run_shell_commands(shell_commands.to_vec());
    // message to help user with next move
    println!(r#"After `cargo auto doc`, check `docs/index.html`. If ok, then `git commit -a -m"message"` and `git push`,"#);
    println!("then `cargo auto publish_to_crates_io`");
}

/// example hot to publish to crates.io and git tag
fn task_publish_to_crates_io() {
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = package_version()
    );
    run_shell_command(&shell_command);

    // cargo publish
    run_shell_command("cargo publish");
    println!(r#"After `cargo auto task_publish_to_crates_io', check `crates.io` page."#);
    println!(r#"If binary then install with `cargo install crate_name` and check how it works."#);
    println!(r#"If library then add dependency to your rust project and check how it works."#);
}

// endregion: tasks

// region: helper functions

/// check if run in rust project root directory error
/// there must be Cargo.toml and the directory automation_tasks_rs
fn is_not_run_in_rust_project_root_directory() -> bool {
    // return negation of exists
    !(std::path::Path::new("automation_tasks_rs").exists()
        && std::path::Path::new("Cargo.toml").exists())
}

// endregion: helper functions
