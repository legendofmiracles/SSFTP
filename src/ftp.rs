use std::path::Path;
use ftp::ftpStream; // use the ftp crate
use ftp::openssl::ssl::{ SslContext, SslMethod }; // also use SSL for FTP.
use crate::parseFtp;

pub fn checkIfFileExists(file: String) -> bool {
    let exists = Path::new(file).exists();
    if exists == true {
        println!("{0} exists on machine.", file);
        send(file); // send the file.
    } else {
        println!("{0} doesn't exist. Please check your file name and try again", file);
    }
    Path::new(file).exists()
}

pub fn send(file: String){
    if checkIfFileExists(file){
        println!("Sending {0}", file);
    }
}


// start the ftp server
pub fn startServer(){
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap_or_else(|err|
        // if error show here
        panic!("error! {}", err)
    );
    let ctx = SslContext::builder(SslMethod::tls()).unwrap().build(); // unwrap SSL with TLS and build.

    // Switch to secure mode
    let mut ftp_stream = ftp_stream.into_secure(ctx).unwrap();
    ftp_stream.login("anonymous", "anonymous").unwrap(); // login with anonymous
}

// initialize the FTP client
pub fn init(file: String){
    startServer();
}