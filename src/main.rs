use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use colored::Colorize;

/* -- TO-DO --
    - CREATE A BETTER WAY TO SEE IF THE USER HAS USED THIS PROGRAM BEFORE (PREF WITHOUT A DATABASE)
    - FIRST CHECK IF THE PATH IS CORRECT WHEN CREATING A NEW ORIGIN FILE, AND LOOP EARLIER AS TO NOT WASTE TIME
*/

fn main() {
    let mut full_file_name = String::new();

    // Ask user if they have used the program before

    println!("Have you used this program before: [1 - yes] [2 - no]");
    let mut used_before = String::new();

    io::stdin()
        .read_line(&mut used_before)
        .expect("Invalid input");

    used_before = format!("{}", used_before.trim());

    if &used_before == "1" {
        println!(
            "Welcome back! What is the full directory AND file of your master password JSON file"
        );

        loop {
            io::stdin()
                .read_line(&mut full_file_name)
                .expect("Bad bad bad");
            let trimmed_file_name = full_file_name.trim();

            if Path::new(&trimmed_file_name).exists() {
                println!("File has been found! Continuing..\n");
                break;
            } else {
                println!("This is not a valid directory and file, please try again.");
                full_file_name.clear();
            }
        }

        // Search to see if the file that they input exists.
    } else if &used_before == "2" {
        loop {
            // Prompt for the directory
            println!("Welcome to the CLI password manager! If this is your first time using this program,\nplease enter the directory to the main password file that you would like to create.");
            let mut directory = String::new();
            io::stdin()
                .read_line(&mut directory)
                .expect("Invalid input");

            // Prompt for the file name
            println!("Perfect, now please enter what you would like to name the file");
            let mut file_name = String::new();
            io::stdin()
                .read_line(&mut file_name)
                .expect("Invalid input");

            // Trim leading and trailing whitespaces from directory and file_name
            let directory = directory.trim();
            let full_file_name = format!("{}.JSON", file_name.trim());

            let mut file_path = PathBuf::from(&directory);
            file_path.push(&full_file_name);

            if file_path.exists() {
                eprintln!("It seems as if this file directory and name already exist, please try again.\n");
            } else {
                match File::create(&file_path) {
                    Ok(_) => {
                        println!("Origin file successfully created!");
                        // Break out of the loop if the file creation is successful
                        break;
                    }
                    Err(e) => eprintln!("Error creating file, {}", e),
                }
            }
        }
    } else {
        println!("You entered an invalid number, please restart the program.")
    }

    println!("\n\nWelcome to the only CLI password manager you'll ever need!");
    println!("What would you like to do\n\t- Create new entry [1]\n\t- Delete entry [2]\n\t- Edit entry [3]\n\t- View entries [4]\n\t- Update entries [5] ");
    let mut action = String::new();

    io::stdin().read_line(&mut action).expect("Invalid input.");
    action = action.trim().to_string();

    match &action[..] {
        "1" => create_pwds(),
        // "2" => delete_pwds(),
        // "3" => edit_pwds(),
        // "4" => view_pwds(),
        // "5" => update_pwds(),
        _ => println!("Please enter a valid number."),
    }
    //  Add what the user can do!


}


fn create_pwds(){

    loop {
    // -- TO-DO -- Be able to add multiple usernames and passwords for one website! 
    println!("Please enter the website that this password will be used for");
    let mut website_name = String::new();
    io::stdin().read_line(&mut website_name).expect("Invalid input.");
    website_name = website_name.trim().to_string();

    println!("Great! Now what is the username for {}", website_name);
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Invalid input.");
    username = username.trim().to_string();

    println!("Awesome, and now the password!");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Invalid input.");
    password = password.trim().to_string();
 
 println!("ta-da! the for the website {}, you have entered a username of {}, and a password of {}. Is all of this information correct?\n\t- yes [1]\n\t- no", website_name.bold().red(), username.bold().red(), password.bold().red());
 let mut save_info = String::new();
 io::stdin().read_line(&mut save_info).expect("Invalid input.");
    save_info = save_info.trim().to_string();

    if save_info == "1" {
        println!("Perfect! Saving now..");
        break;
    } 
    
    }

   
}
