use std::env;
use std::fs;
use reqwest::Client;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

async fn post_data() -> Result<()> {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file: {}", filename);

    let data = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("File content: {}", data);

    let client = Client::new();
    let req = client
        .post("https://httpbin.org/post")
        .header("Accepts", "text/plain")
        .body(data);

    let res = req.send().await?;
    let status_code = res.status().clone();

    let body = res.bytes().await?;

    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("response: {} ", s);
    println!("{}", status_code);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    post_data().await?;
    Ok(())
}