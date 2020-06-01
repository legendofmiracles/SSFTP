// imports
mod parseFtp;

// public functions (other then init)


// startServer starts the FTP server
pub fn startServer(){
    println!("Work in progress FTP server init.");
    parseFtp::checkForMal("etc.sh".to_owned());
}


// init function
pub fn init(){
    startServer();
}
