use std::path::Path;
use ftp::ftpStream; // use the ftp crate
use ftp::openssl::ssl::{ SslContext, SslMethod }; // also use SSL for FTP.
// mod parseFtp;
use crate::parseFtp;

pub fn checkIfFileExists(&file) -> bool {
    let exists = Path::new(&file!).exists();
    if exists == true {
        println!("{0} exists on machine.", file!);
        send(file!); // send the file.
    } else {
        println!("{0} doesn't exist. Please check your file name and try again", file!);
    }
    Path::new(file!).exists()
}

pub fn send(&file) {
    if checkIfFileExists(file!){
        println!("Sending {0}", file!);
    }
}

// pub fn get_user_input(){
// }


// start the ftp server
pub fn startServer(){
    let ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    let ctx = SslContext::builder(SslMethod::tls()).unwrap().build();
    // Switch to the secure mode
    let mut ftp_stream = ftp_stream.into_secure(ctx).unwrap();
    // get input so we can sign in
    // get_user_input();
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Enter username ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct user.");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    // get password
    let mut s2=String::new();
    print!("Enter password: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct pass.");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    

    ftp_stream.login(s, s2).unwrap(); // log in with credentials.
    // Do other secret stuff
    // Switch back to the insecure mode (if required)
    let mut ftp_stream = ftp_stream.into_insecure().unwrap();
    // Do all public stuff
    let _ = ftp_stream.quit();
}

// initialize the FTP server
pub fn init(){
    startServer();
}