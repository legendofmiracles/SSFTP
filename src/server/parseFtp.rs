// parse the files that are sent through FTP.


// imports
// crate ffmpeg;
use std::fs;
use std::io;

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
    println!("File valid. Parsing");
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
        textFile(file.to_string());
    }
}


// functions for reading specific documents.
pub fn textFile(file: std::string::String) { 
    println!("In file {}", file);

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    
    // ask if want to download or keep on server:
    let mut choice = String::new();
    print!("Keep file on server or download it? ");
    match io::stdin().read_line(&mut choice){
        Ok(_) => {
            if choice == "server" {
                // ...
            }
        }
        Err(e) => {
            println!("err: {0}", e);
        }
    }
}