use std::fs::File;
use std::io;
use std::path::PathBuf;

fn main() {
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
            eprintln!("It seems as if this file directory and name already exist, please try again.");
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


    println!("What would you like to do\n\t- Create ");
    //  Add what i need to do :D

   
}