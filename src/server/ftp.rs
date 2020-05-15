// 5/14/2020 - Remodled ftp.rs (too many errors, i also don't
// really understand my own code at this point...)
//


// imports
mod parseFtp;

// public functions (other then init)
pub fn sendFile(){
    // we need to parse the file, so we are going to check for malware
    parseFtp::checkForMal("eatmynuggets.bat".to_string()); // should return true
    parseFtp::checkForMal("latexdocumentation.txt".to_string()); // should return false
}


pub fn startServer(){
    // ...
}


// init function
pub fn init(){
    println!("Initializing SSFTP.");
    sendFile();
}
