use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::fs::{DirBuilder, self};
use std::string::{String};
mod page_home; 


static AUTHORIZED_PROTOCOLS: [&str; 2] = ["POST", "GET"];
static LOG_CATEGORY_CENTER_WIDTH: usize = 15; //used to have the [  TEXT  ] effect in `log(..)`

static LOG_INFO : &str =            "INFO";
static LOG_STREAM_ERROR : &str =    "SREAM ERROR";
static LOG_FILE_SYSTEM : &str =     "FILE SYSTEM";
static LOG_SERVER : &str =          "SERVER";

//
static FILE_PATH: &str = "./files";
//

fn log(msg: &str, category: &str){
    println!("[{:^w$}] > {}", category, msg, w=LOG_CATEGORY_CENTER_WIDTH);
    return ();
}
fn main() {
    //setup multiple ip adress (if one of them is unavailable)
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 80)),
        SocketAddr::from(([127, 0, 0, 1], 8080)),
    ];

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

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0;512];
    let read_size = stream.read(&mut buffer[..]).unwrap_or_default();
    let req: String;

    let mut route: String = "".to_string();
    let mut first_arg: String = "".to_string();
    
    log(&format!("new client @{}", stream.local_addr().unwrap().ip()), &LOG_INFO);

    if read_size > 0 { //if the stream gave us some content to read:
        req = String::from_utf8_lossy(&buffer).to_string(); //then we get a string from it 

        for protocol in AUTHORIZED_PROTOCOLS {
            if req.starts_with(protocol) { //if the HTTP protocol used by the client is valid :
                //exemple of req = "POST /route/defined/by/client HTTP/1.1 ..."
                route = req.split_once(" HTTP").unwrap().0.to_string(); //get rid of " HTTP/1.1 ..."
                route = route.split_at(protocol.len()+1).1.to_string(); //get rid of "POST "

                let first_arg_index: usize = if route.starts_with("/") { 1 } else { 0 };
                let tmp: Vec<&str> = route.split("/").collect();
                first_arg = "/".to_string() + tmp.get(first_arg_index).unwrap();
            }
        }
    }

    if !&route.is_empty() { //if the client has a valid request
        controller(stream, first_arg,route);
    }

    //println!("{}", page_home::get_http_frame_home(FILE_PATH));
}

fn controller(stram: TcpStream, first_arg: String, route: String){
    match first_arg.as_str() {
        "/"     => println!("HOME"),
        "/file" => println!("DOWNLOAD"),
        _ => println!("404")
    };
}
