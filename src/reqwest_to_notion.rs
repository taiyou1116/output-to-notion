use dotenv::dotenv;
use reqwest;
use serde_json::json;

pub async fn run() -> Result<(), String> {
    dotenv().ok();
    let token = std::env::var("TOKEN").unwrap();
    let database_id = std::env::var("DBID").unwrap();
    let url = format!("https://api.notion.com/v1/pages",);

    let client = reqwest::Client::new();

    let new_page_data = json!({
        "parent": { "database_id": database_id },
        "properties": {
            "Vocabulary": {
                "title": [
                    {
                        "text": {
                            "content": "test"
                        }
                    }
                ]
            },
            "頻出度": {
                "select": {
                    "name": "🥈超使える"
                }
            },
            "習得度": {
                "status": {
                    "name": "インプット中"
                }
            }
        }
    });

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", "2021-08-16")
        .json(&new_page_data)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let res_text = res.text().await.map_err(|e| e.to_string())?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text).map_err(|e| e.to_string())?;
    println!("New Page Created: {:?}", res_text);
    let parent_page_id = res_json["id"].as_str().unwrap_or("");
    println!("parent_id: {:?}", parent_page_id);

    Ok(())
}

// pub async fn add_sub_page(res_text: String) -> Result<(), String> {
//     let res_json: serde_json::Value = serde_json::from_str(&res_text).map_err(|e| e.to_string())?;
//     let parent_id = res_json["id"].as_str().unwrap();

//     Ok(())
// }
