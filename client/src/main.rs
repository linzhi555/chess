use std::io::Read;

fn main() {
    let mut res = reqwest::blocking::get("http://localhost:8080/hello/client").unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
}
