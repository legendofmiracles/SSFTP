// imports
mod parseFtp;

// public functions (other then init)



pub fn startServer(){
    parseFtp::init(); // init.
}


// init function
pub fn init(){
    startServer();
}
