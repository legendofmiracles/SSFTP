// 5/14/2020 - Remodled ftp.rs (too many errors, i also don't
// really understand my own code at this point...)
//


// imports
mod parseFtp;

// public functions (other then init)



pub fn startServer(){
    // ...
}


// init function
pub fn init(){
    println!("Initializing SSFTP.");
    parseFtp::checkForMal("homework.mp4".to_string());
    parseFtp::checkForMal("etc.sh".to_string());
}
