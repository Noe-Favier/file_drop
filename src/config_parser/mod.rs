
fn get_file_path()->String {
    //try to check if path is specified in args

    //is the conf file in './'
}

fn get_authorized_protocol()->Vec<String>{
    //parse array from config
    return Vec::new();
}

fn get_field(field_name: String)-> String{

}

fn get_port() -> i8 {
    return get_field("port".to_string()).parse::<i8>().unwrap();;
}

