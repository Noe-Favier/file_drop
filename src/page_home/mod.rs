use std::net::{TcpStream};
use std::fs::{read_to_string, read_dir};

pub fn send_home(stream: TcpStream, pathToFiles: &str) -> String {
    return get_home_headers() + &get_home_html(pathToFiles);
}

fn get_home_headers() -> String {
    return "headers\n".to_string();
}

pub fn get_home_html(pathToFiles: &str) -> String {
    let filename = "page_home/home.noehtml";
    return read_to_string(filename)
        .expect(&format!("ERR ! FILE \"{}\" NOT FOUND", filename))
        .replace("<!--ELEMENTS_HERE-->", &get_elt_divs(pathToFiles));
}

fn get_elt_divs(path_to_files: &str) -> String {
    let to_be_returned: &str = "";
    if let Ok(entries) = read_dir(path_to_files) {
        for entry in entries {
            if let Ok(entry) = entry {
                to_be_returned.to_string().push_str(&format!("<p>{:?}</p>", entry.file_name()));
            }
        }
    }
    return to_be_returned.to_string();
}