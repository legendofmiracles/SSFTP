// parse the files that are sent through FTP.


// imports
// crate ffmpeg;
use std::fs;
use std::io;
use std::process::Command;

// global scoped, immutable variables


// global scoped, mutable variables

// public functions

// check for malware checks if the extension is malicious
// example:
// checkForMal(".exe"); true
// checkForMal(".pdf"); false
pub fn checkForMal(file: std::string::String) {
    let invalidExtensions = [".exe", ".bat", ".sh", ".ps1"];
    let weirdExtensions = [".exe.mp4", ".exe.mp3", ".exe.txt", ".exe.docx", // im tired i dont want to write more extensions then this
                           ".bat.mp4", ".bat.mp3", ".bat.txt", ".bat.docx",
                           ".sh.mp4", ".sh.mp3", ".sh.txt", ".sh.docx",
                           ".ps1.mp4", ".ps1.mp3", ".ps1.txt", ".ps1.docx"]; // they will come soon
    for i in 0..weirdExtensions.len() {
        if file.ends_with(weirdExtensions[i]) {
            println!("File {0} is not allowed due to extension: {1}", file, weirdExtensions[i]);
            break;
        } else {
            if file.contains(invalidExtensions[0]) {
                println!("Because {0} has the extension {1}, we cannot allow this file through FTP", file, invalidExtensions[0]);
            }
            if file.contains(invalidExtensions[1]){
                println!("Because {0} has the extension {1}, we cannot allow this file through FTP", file, invalidExtensions[1]);
            }
            if file.contains(invalidExtensions[2]){
                println!("Because {0} has the extension {1}, we cannot allow this file through FTP", file, invalidExtensions[2]);
            }
            if file.contains(invalidExtensions[3]){
                println!("Because {0} has the extension {1}, we cannot allow this file through FTP", file, invalidExtensions[3]);
            }
            break;
        }
    }
    
    // println!("{0} is valid. Parsing", file.to_string());
}

pub fn read(file: std::string::String) {
    // video files
    if file.contains(".mp4"){
        // process it.
    }
    if file.contains(".mp3"){
       audioProcess(file.to_string());
    }
    // text files (such as text files) 
    if file.contains(".txt"){
        text_file(file.to_string());
    }
}

pub fn audioProcess(file: std::string::String){
    // foo bar lol
}

// recursivelyDownloadFile, well, recursively downloads text files.
pub fn recursivelyDownloadFile(file: std::string::String) {
    let CurrentOS = std::env::consts::OS;
    let checkOS = CurrentOS.to_string();
    if checkOS == "windows" { // checks for windows
        Command::new("poweshell cat ".to_owned() + &file + " >> temp.txt")
            .spawn()
            .expect("WARNING: You may not have specific permissions");
        // once this is done, we'll submit the data onto a database
        // then we run that on the client side and something.
    }
    if checkOS == "mac" {
        // we can run it natively.
        // no needing to run it in a sub-subshell
        Command::new("cat ".to_owned() + &file + " >> temp.txt")
            .spawn()
            .expect("WARNING. Please run with sudo if you have not.");
        // now we add the raw data to a database,
        // encrypt it with an algorithm
        // and do other stuff.
    }
    if checkOS == "linux" {
        Command::new("cat ".to_owned() + &file + " >> temp.txt")
            .spawn()
            .expect("WARNING. Please run with sudo if you have not.");
        // this is like mac
    }
}

// functions for reading specific documents.
pub fn text_file(file: std::string::String) { 
    let fileName: std::string::String = "".to_string();
    println!("In file {}", file);

    let _contents = fs::read_to_string(fileName)
        .expect("Something went wrong reading the file");
    
    // ask if want to download or keep on server:
    let mut choice = String::new();
    print!("Keep file on server or download it? ");
    match io::stdin().read_line(&mut choice){
        Ok(_) => {
            if choice == "server" {
                println!("Keeping on server.");
                print!("Do you want to save Raw Data to cloud or into a database?");
                let choice2: std::string::String = "".to_string();
                let _contents2 = fs::read_to_string(choice2)
                    .expect("Something went wrong.");
                let mut _choice2 = String::new();
                match io::stdin().read_line(&mut _choice2){
                    Ok(_) => {
                        if _choice2 == "cloud" {
                            // run the gui application
                            let CurrentOS = std::env::consts::OS;
                            let checkOS = CurrentOS.to_string();
                            if checkOS == "windows" {
                                // usually it runs with py, so, let's try that
                                Command::new("py extScripts/server.py")
                                    .spawn() // create the instance
                                    .expect("Error ocurred, re attempting");
                                // attempt 2?
                                Command::new("python3 extScripts/server.py")
                                    .spawn() // make instance
                                    .expect("Error occcured, make an issue on the github"); // shrug :)
                            } else if checkOS == "mac" {
                                // usually it runs with py, so, let's try that
                                Command::new("py extScripts/server.py")
                                    .spawn() // create the instance
                                    .expect("Error ocurred, re attempting");
                                // attempt 2?
                                Command::new("python3 extScripts/server.py")
                                    .spawn() // make instance
                                    .expect("Error occcured, make an issue on the github"); // shrug :)
                            } else if checkOS == "linux" {
                                // usually it runs with py, so, let's try that
                                Command::new("py extScripts/server.py")
                                    .spawn() // create the instance
                                    .expect("Error ocurred, re attempting");
                                // attempt 2?
                                Command::new("python3 extScripts/server.py")
                                    .spawn() // make instance
                                    .expect("Error occcured, make an issue on the github"); // shrug :)
                            } else {
                                print!("unknown os lol!");
                            }
                        } else if _choice2 == "database"{
                            // use diesel
                        }
                    }
                    Err(e) => {
                        print!("Error occured! Details: {}", e);
                    }
                }
            }
            // i really don't know else-if statments in rust so...
            if choice == "download" {
                println!("got here");
                recursivelyDownloadFile(file);
            }
        }
        Err(e) => {
            println!("err: {}", e);
        }
    }
}
