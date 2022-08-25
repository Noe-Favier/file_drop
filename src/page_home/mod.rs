use std::fs::{read_to_string, read_dir, ReadDir};

pub fn get_http_frame_home(path_to_files: &str) -> String {
    let html_content: String = get_home_html(path_to_files);
    return get_home_headers(html_content.len()) + &html_content;
}

fn get_home_headers(length: usize) -> String {
    return "HTTP/1.1 200 OK\r\nContent-Length: CONTENT_LENGTH\r\nContent-Type: text/html\r\n\r\n".replace("CONTENT_LENGTH", &length.to_string()).to_string();
}

fn get_home_html(path_to_files: &str) -> String {
    let filename: &str = "src/page_home/home.html";
    return read_to_string(filename)
        .expect(&format!("ERR ! FILE \"{}\" NOT FOUND", filename))
        .replace("<!--ELEMENTS_HERE-->", &get_elt_divs(path_to_files));
}

fn get_elt_divs(path_to_files: &str) -> String {
    let mut to_be_returned: String = "".to_string();

    //get files in the "files" dir 
    let paths: ReadDir = read_dir(path_to_files).unwrap();

    //next, we'll inject them into home.html
    let default_filename: String = "<em>Untilted</em>".to_string();
    for path in paths {
        let opt: [String; 3]  = match path {
            Err(_x) => {
                    [
                        "span".to_string(), 
                        default_filename.to_owned(),
                        "id".to_string()
                    ]
            },

            Ok(f) => {
                    [
                        "a".to_string(), 
                        f.file_name().to_os_string().to_str().unwrap_or(&default_filename).to_string(),
                        "href".to_string()
                    ]
            }
        };

        to_be_returned.push_str(
            &get_div_content(
                opt[0].to_owned(),
                opt[1].to_owned(),
                opt[2].to_owned()
        ));
    }
    return to_be_returned.to_string();
}

fn get_div_content(node: String, labl: String, attr: String) -> String{
    return format!("<{n} {a}=\"/file/{l}\"> {l} </{n}>", n=node, l=labl, a=attr);
}