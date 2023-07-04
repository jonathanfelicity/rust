pub fn req_status(status: i32){
    match status {
        200 => println!("Succes");
        404 => println!("Not Found");
        other => {
            println!("Request code failed {}", other)
        }
    }
}
