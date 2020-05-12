use std::path::Path;
use ftp::ftpStream; // use the ftp crate
use ftp::openssl::ssl::{ SslContext, SslMethod }; // also use SSL for FTP.
use crate::parseFtp;

pub fn checkIfFileExists(&file) {
    let exists = Path::new(&file).exists();
    if exists == true {
        println!("{0} exists on machine.", file);
        send(file); // send the file.
    } else {
        println!("{0} doesn't exist. Please check your file name and try again", file);
    }
    Path::new(file).exists()
}

pub fn send(&file) {
    if checkIfFileExists(file){
        println!("Sending {0}", file!);
    }
}


// start the ftp server
pub fn startServer(){
    let ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    let ctx = SslContext::builder(SslMethod::tls()).unwrap().build();
    // Switch to the secure mode
    let mut ftp_stream = ftp_stream.into_secure(ctx).unwrap();
    ftp_stream.login("anonymous", "anonymous").unwrap();
    // Do other secret stuff
    // Switch back to the insecure mode (if required)
    let mut ftp_stream = ftp_stream.into_insecure().unwrap();
    // Do all public stuff
    let _ = ftp_stream.quit();
}

// initialize the FTP client
pub fn init(){
    startServer();
}