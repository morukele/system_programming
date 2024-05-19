use system_programming::networking;

fn main() {
    let res = networking::http::request();
    match res {
        Ok(_) => {
            println!("Request terminated normally")
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
