mod copy_from_chatgpt;
mod reqwest_to_notion;

#[tokio::main]
async fn main() {
    let input = copy_from_chatgpt::run();
    match reqwest_to_notion::run(input).await {
        Ok(_) => {
            println!("OK");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
