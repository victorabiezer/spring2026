use std::process::Command; // to run system commands 
use std::io::Write; 

enum FileOperation 
{
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}



fn perform_operation(operation: FileOperation) 
{
    match operation
    {
        FileOperation::List(path) => 
        {
            Command::new("ls").arg(path).status().expect("Failed to execute ls"); 
        }
        FileOperation::Display(path) =>
        {
            Command::new("cat").arg(path).status().expect("Failed to execute cat");
        }
        FileOperation::Create(file_path, content) =>
        {
            let command = format!("echo '{}' > {}", content, file_path);
            Command::new("sh").arg("-c").arg(command).status().expect("Failed to create a file");
        }
        FileOperation::Remove(path) =>
        {
            Command::new("rm").arg(path).status().expect("Failed to remove file");
        }
        FileOperation::Pwd =>
        {
            Command::new("pwd").status().expect("Failed to execute pwd"); 
        }
    } 
}

fn main() 
{
    println!("Welcome to the File Operations Program!");
    
    loop
    {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit"); 
        print!("\nEnter your choice (0-5): "); 
        std::io::stdout().flush().unwrap(); 
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim()
        {
            "1" => 
            {
                print!("Enter directory path: ");
                std::io::stdout().flush().unwrap();
                let mut path = String::new();
                std::io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::List(path.trim().to_string())); 
            }

            "2" =>
            {
                print!("Enter file path: ");
                std::io::stdout().flush().unwrap();
                let mut path = String::new();
                std::io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::Display(path.trim().to_string()));
            }

            "3" =>
            {
                print!("Enter file path: ");
                std::io::stdout().flush().unwrap();
                let mut path = String::new(); 
                std::io::stdin().read_line(&mut path).unwrap();
                print!("Enter content: ");
                std::io::stdout().flush().unwrap();
                let mut content = String::new();
                std::io::stdin().read_line(&mut content).unwrap();
                perform_operation(FileOperation::Create(path.trim().to_string(), content.trim().to_string()));
            }

            "4" =>
            {
                print!("Enter file path: "); 
                std::io::stdout().flush().unwrap();
                let mut path = String::new(); 
                std::io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::Remove(path.trim().to_string()));
            }

            "5" =>
            {
                perform_operation(FileOperation::Pwd);
            }

            "0" =>
            {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."), 
        }
    }
}