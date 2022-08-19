use std::net::{SocketAddr, TcpListener, TcpStream};
use std::fs::{DirBuilder, self};

static LOG_CATEGORY_CENTER_WIDTH: usize = 15; //used to have the [  TEXT  ] effect in `log(..)`

static LOG_INFO : &str =            "INFO";
static LOG_STREAM_ERROR : &str =    "SREAM ERROR";
static LOG_FILE_SYSTEM : &str =     "FILE SYSTEM";
static LOG_SERVER : &str =          "SERVER";

fn handle_client(stream: TcpStream) {
    log(&format!("new client : [{}]", stream.local_addr().unwrap().ip()), &LOG_INFO);
}

fn log(msg: &str, category: &str){
    println!("[{:^w$}] > {}", category, msg, w=LOG_CATEGORY_CENTER_WIDTH);
}
fn main() {
    //setup multiple ip adress (if one of them is unavailable)
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 80)),
        SocketAddr::from(([127, 0, 0, 1], 8080)),
    ];
    
    const FILE_PATH: &str = "./files";

    //check if the folder exists :
    if fs::metadata(FILE_PATH).is_err() {
        //FILE_PATH isn't a folder or doesn't exists, try to create it :
        let dir_builder: DirBuilder = DirBuilder::new();
        dir_builder.create(FILE_PATH).unwrap();

        assert!(fs::metadata(FILE_PATH).unwrap().is_dir()); //stop if it didn't worked
        log(&format!("created folder : \"{}\"", FILE_PATH), &LOG_FILE_SYSTEM);
    }

    //run server
    let listener = TcpListener::bind(&addrs[..]).unwrap();
    log(&format!("running on : {}", listener.local_addr().unwrap().ip()), LOG_SERVER);


    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => { 
                log(&format!("error ! client couldn't be handled : {}", e.to_string()), &LOG_STREAM_ERROR);
            }
        }
    }
}