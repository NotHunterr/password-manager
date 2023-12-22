use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};


/* -- TO-DO --
    - CREATE A BETTER WAY TO SEE IF THE USER HAS USED THIS PROGRAM BEFORE (PREF WITHOUT A DATABASE)
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
        println!("Welcome back! What is the full directory AND file of your master password JSON file");
      
        loop {
            io::stdin().read_line(&mut full_file_name).expect("Bad bad bad");
            let trimmed_file_name = full_file_name.trim();
           
           if Path::new(&trimmed_file_name).exists() {
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
    println!("What would you like to do\n\t- Create new entry\n\t- Delete entry\n\t- Edit entry\n\t- View entries\n\t- Update entries ");
    //  Add what the user can do!

   
}