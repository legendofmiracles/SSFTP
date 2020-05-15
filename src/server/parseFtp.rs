// parse the files that are sent through FTP.


// imports
crate ffmpeg;

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
    if file.contains(".mp4"){
        // use FFmpeg
    }
    if file.contains(".mp3"){
        // also use FFmpeg
    }
}
