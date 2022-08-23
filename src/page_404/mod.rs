use std::fs::read_to_string;

pub fn get_http_frame_404() -> String {
    let html_content: String = get_404_html();
    return get_404_headers(html_content.len()) + &html_content;
}

fn get_404_headers(length: usize) -> String {
    return "HTTP/1.1 200 OK\r\nContent-Length: CONTENT_LENGTH\r\nContent-Type: text/html\r\n\r\n".replace("CONTENT_LENGTH", &length.to_string()).to_string();
}

fn get_404_html() -> String {
    let filename = "src/page_404/404.html";
    return read_to_string(filename)
        .expect(&format!("ERR ! FILE \"{}\" NOT FOUND", filename))
}
