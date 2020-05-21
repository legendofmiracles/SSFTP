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
    //let mut splitFile = file.split(".");
    let invalidExtensions = [".exe", ".bat", ".sh", ".ps1"];
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
    println!("{0} is valid. Parsing", file.to_string());
}


pub fn read(file: std::string::String) {
    // video files
    if file.contains(".mp4"){
        // use FFmpeg
    }
    if file.contains(".mp3"){
        // also use FFmpeg
    }
    // text files (such as text files) 
    if file.contains(".txt"){
        text_file(file.to_string());
    }
}


// recursivelyDownloadFile, well, recursively downloads files.
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
            }
            // i really don't know else-if statments in rust so...
            if choice == "download" {
                println!("got here");
                recursivelyDownloadFile(file);
            }
        }
        Err(e) => {
            println!("err: {0}", e);
        }
    }
}
