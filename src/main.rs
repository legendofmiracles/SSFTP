// server imports
#[path = "server/ftp.rs"] mod ftp;
//#[path = "server/sftp.rs"] mod sftp;
#[path = "server/parseFtp.rs"] mod parseFtp;

// client imports


fn main() {
    // just a place holder
    // // the real code will come soon.
    ftp::init();
}
