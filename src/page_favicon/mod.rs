use std::fs::File;
use std::io::Read;

pub fn get_http_frame_favicon() -> Vec<u8> {
    let mut ico: Vec<u8> = get_favicon();
    let mut tmp: Vec<u8> = get_favicon_headers(ico.len());
    tmp.append(&mut ico);
    return tmp;
}

fn get_favicon_headers(length: usize) -> Vec<u8> {
    let mut content: Vec<u8> = Vec::new();
    content.append(
        &mut "HTTP/1.1 200 OK\r\nContent-Length: CONTENT_LENGTH\r\nContent-Type: image/ico\r\n\r\n"
        .replace("CONTENT_LENGTH", &length.to_string())
        .as_bytes().to_vec()
    );
    return content;
}

fn get_favicon() -> Vec<u8> {
    let mut file = File::open("src/page_favicon/favicon.ico").expect("Unable to open favicon");
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect(&format!("ERROR READING FAVICON"));
    return content;
}
