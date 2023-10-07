mod reqwest_to_notion;

#[tokio::main]
async fn main() {
    match reqwest_to_notion::run().await {
        Ok(_) => {
            println!("OK");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
