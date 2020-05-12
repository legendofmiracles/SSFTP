use std::path::Path;
mod parseSftp;

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

pub fn init(file: String){

}