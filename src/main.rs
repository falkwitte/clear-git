use clap::Parser;
use std::fs::{remove_dir_all, read_dir, remove_file};
use std::path::Path;


/// Simple cli to remove unwanted files from a git folder
#[derive(Parser)]
#[command(author="https://github.com/falkwitte", version)]
struct Opts{
    /// removes custom dir entry
    #[arg(default_value="")]
    #[arg(long="remove")]
    #[arg(short='r')]
    dir:String,
}


fn main() {
    let mut blacklist = vec![".obsidian", ".idea"];
    let args = Opts::parse();


    if !args.dir.is_empty() { 
        // remove custom user dir
        blacklist.push(&args.dir);
    }

    if let Ok(entries) = read_dir(".") {
        for entry in entries.flatten(){
            
            let entry_string = entry.file_name()
                                    .into_string()
                                    .unwrap();  

            for i in blacklist.iter(){
                // remove dir
                if i == &entry_string.as_str() && Path::new(i).is_dir(){
                    remove_dir_all(&entry_string).unwrap_or_else(|_| println!("Error: No such file or directory"));

                // remove file
                }else if i == &entry_string && !Path::new(i).is_dir(){
                    remove_file(&entry_string).unwrap_or_else(|_| println!("Error: No such file or directory"));
                }
            }
        }
    }
    
}
