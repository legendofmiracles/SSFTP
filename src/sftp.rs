use std::path::Path;
mod parseSftp;

pub fn checkIfFileExists(file: String) -> bool {
    Path::new(file).exists()
}

pub fn send(file: String){
    if checkIfFileExists(file){
        println!("Sending {0}", file);
    }
}

pub fn init(){

}