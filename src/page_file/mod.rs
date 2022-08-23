use std::fs::{File};
use std::io::Read;
use std::path::Path;

pub fn get_http_frame_file(file_path: String) -> Vec<u8> {
    if Path::new("/etc/hosts").exists(){
        let mut ico: Vec<u8> = get_file(file_path);
        let mut tmp: Vec<u8> = get_file_headers(ico.len());
        tmp.append(&mut ico);
        return tmp;
    }
    return [0;0].to_vec(); //we return nothing if the file doesn't exist
}

fn get_file_headers(length: usize) -> Vec<u8> {
    let mut content: Vec<u8> = Vec::new();
    content.append(
        &mut "HTTP/1.1 200 OK\r\nContent-Length: CONTENT_LENGTH\r\nContent-Type: image/ico\r\n\r\n"
        .replace("CONTENT_LENGTH", &length.to_string())
        .as_bytes().to_vec()
    );
    return content;
}

fn get_file(file_path: String) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect(&format!("ERROR READING file"));
    return content;
}
