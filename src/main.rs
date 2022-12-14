use std::thread;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::fs::{DirBuilder, self};
use std::string::String;

mod page_home; 
mod page_404;
mod page_favicon;
mod page_file;

static AUTHORIZED_PROTOCOLS: [&str; 2] = ["POST", "GET"];
static LOG_CATEGORY_CENTER_WIDTH: usize = 20; //used to have the [  TEXT  ] effect in `log(..)`

static LOG_INFO : &str =                "INFO";
static LOG_STREAM_INFO : &str =         "INFO STREAM";
static LOG_STREAM_ERROR : &str =        "SREAM ERROR";
static LOG_FILE_SYSTEM : &str =         "FILE SYSTEM";
static LOG_SERVER : &str =              "SERVER";

//
static FILE_PATH: &str = "./files";
//

fn log(msg: &str, category: &str){
    println!("[{:^w$}] > {}", category, msg, w=LOG_CATEGORY_CENTER_WIDTH);
    return ();
}
fn main() {
    //setup multiple ip adress (if one of them is unavailable)
    let addrs:[SocketAddr; 2] = [
        SocketAddr::from(([0, 0, 0, 0], 80)),
        SocketAddr::from(([0, 0, 0, 0], 8080)),
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
    let listener: TcpListener = TcpListener::bind(&addrs[..]).unwrap();
    log(&format!("running on : {}", listener.local_addr().unwrap().ip()), LOG_SERVER);


    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => { 
                log(&format!("error ! client couldn't be handled : {}", e.to_string()), &LOG_STREAM_ERROR);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0;512];
    let read_size: usize = stream.read(&mut buffer[..]).unwrap_or_default();
    let req: String;

    let mut route: String = "".to_string();
    let mut first_arg: String = "".to_string();

    if read_size > 0 { //if the stream gave us some content to read:
        req = String::from_utf8_lossy(&buffer).to_string(); //then we get a string from it 

        for protocol in AUTHORIZED_PROTOCOLS {
            if req.starts_with(protocol) ||AUTHORIZED_PROTOCOLS.len() == 0 { //if the HTTP protocol used by the client is valid : (or if no protocol were specified)
                //exemple of req = "POST /route/defined/by/client HTTP/1.1 ..."
                route = req.split_once(" HTTP").unwrap().0.to_string(); //get rid of " HTTP/1.1 ..."
                route = route.split_at(protocol.len()+1).1.to_string(); //get rid of "POST "

                let first_arg_index: usize = if route.starts_with("/") { 1 } else { 0 };
                let tmp: Vec<&str> = route.split("/").collect();
                first_arg = "/".to_string() + tmp.get(first_arg_index).unwrap();
                break; //we've successfully found the protocol and sent the page to our client. We can stop here 
            }
        }
    }

    if !&route.is_empty() { //if the client has a valid request
        controller(stream, first_arg,route);
    }
}

fn controller(mut stream: TcpStream, first_arg: String, mut route: String){
    let client_ip = stream.local_addr().unwrap().ip();
    log(&format!("new client @{}", client_ip), &LOG_INFO);

    match first_arg.as_str() {
        "/"     => {
            // HOME //
            match stream.write(page_home::get_http_frame_home(FILE_PATH).as_bytes()) {
                Ok(result) => log(&format!("sent home ({} bytes) to @{}", result, client_ip), &LOG_STREAM_INFO),
                Err(_err) => log(&format!("can't send home to @{}", client_ip), &LOG_STREAM_ERROR)
            };
            //\\
        },
        
        "/file" => {
            // DOWNLOAD FILE //
            let mut buffer: Vec<u8> = page_file::get_http_frame_file(FILE_PATH.to_string(),route.replace("/file", "").to_string());
            if buffer.len() <=0 {
                buffer = page_404::get_http_frame_404().as_bytes().to_vec();
                route = "404".to_string();
            }
            match stream.write(&buffer) {
                Ok(result) => log(&format!("sent {} ({} bytes) to @{}", route, result, client_ip), &LOG_STREAM_INFO),
                Err(_err) => log(&format!("can't send file to @{}", client_ip), &LOG_STREAM_ERROR)
            };
            //\\
        },

        "/favicon.ico" => {
            // FAVICON //
            match stream.write(&page_favicon::get_http_frame_favicon()) {
                Ok(result) => log(&format!("sent favicon ({} bytes) to @{}", result, client_ip), &LOG_STREAM_INFO),
                Err(_err) => log(&format!("can't send favicon to @{}", client_ip), &LOG_STREAM_ERROR)
            };
            //\\
        },

        _ => {
            // ERROR 404 //
            match stream.write(page_404::get_http_frame_404().as_bytes()) {
                Ok(result) => log(&format!("sent 404 ({} bytes) to @{}", result, client_ip), &LOG_STREAM_INFO),
                Err(_err) => log(&format!("can't send 404 to @{}", client_ip), &LOG_STREAM_ERROR)
            };
            //\\
        }
    };
}
