use crate::notion_json_template;
use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;

const NOTION_API_URL: &str = "https://api.notion.com/v1";

enum HttpMethod {
    Post,
    Patch,
}

async fn send_reqwest(
    client: &Client,
    url: &str,
    token: &str,
    meaning_block: &Value,
    method: HttpMethod,
) -> Result<Value, String> {
    let req_builder = match method {
        HttpMethod::Post => client.post(url),
        HttpMethod::Patch => client.patch(url),
    };

    let res = req_builder
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", "2021-08-16")
        .json(meaning_block)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let res_text = res.text().await.map_err(|e| e.to_string())?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text).map_err(|e| e.to_string())?;

    Ok(res_json)
}

pub async fn run(input: Vec<String>) -> Result<(), String> {
    dotenv().ok();
    let token = std::env::var("TOKEN").unwrap();
    let database_id = std::env::var("DBID").unwrap();
    let url = format!("{}/pages", NOTION_API_URL);

    let client = Client::new();

    let frequency = match input[1].as_str() {
        "非常によく使う(頻度9~10)" => "🥇目から鱗",
        "よく使う(頻度6~8)" | "そこそこ使う(頻度3~5)" => "🥈超使える",
        _ => "🥉使える",
    };

    // 新しいpageを作成(単語)
    let new_page_data = notion_json_template::generate_new_page(database_id, &input[0], &frequency);
    let res_json = send_reqwest(&client, &url, &token, &new_page_data, HttpMethod::Post).await?;

    // blockを作成
    if let Ok(results) = create_parent_block(&client, &token, res_json).await {
        let res = results["results"].as_array().expect("resultsでエラー");

        let index_mapping = [(0, 2), (1, 3), (2, 4), (3, 5), (5, 6)];

        for (child_block_index, input_index) in index_mapping.into_iter() {
            match create_children_block(
                &client,
                &token,
                &res,
                child_block_index,
                &input[input_index],
            )
            .await
            {
                Ok(_) => {
                    println!("create children");
                }
                Err(e) => {
                    println!("Err create children: {}", e);
                }
            }
        }
    } else {
        println!("create parent err")
    }

    Ok(())
}

async fn create_children_block(
    client: &Client,
    token: &str,
    parent_block_value: &[Value],
    number: usize,
    content: &str,
) -> Result<Value, String> {
    // 帰ってきた結果から'number番目のresult'を取得 -> そのresultから親のblock_idを取得
    let some_number_result = parent_block_value[number].clone();
    let parent_block_id = some_number_result["id"].as_str().expect("parentエラー");

    // jsonテンプレートを受け取り、notionに新しくblockを作成 -> 結果を受け取る
    let url = format!("{}/blocks/{}/children", NOTION_API_URL, parent_block_id);
    let children_block = notion_json_template::generate_children_block(content);
    let res_json = send_reqwest(&client, &url, &token, &children_block, HttpMethod::Patch).await?;

    Ok(res_json)
}

async fn create_parent_block(
    client: &Client,
    token: &str,
    res_json: Value,
) -> Result<Value, String> {
    // 帰ってきた結果から'result'を取得 -> そのresultから親のpage_idを取得
    let parent_page_id = res_json["id"].as_str().unwrap_or("");

    // jsonテンプレートを受け取り、notionに新しくblockを作成 -> 結果を受け取る
    let url = format!("{}/blocks/{}/children", NOTION_API_URL, parent_page_id);
    let parent_block = notion_json_template::generate_parent_block();
    let res_json = send_reqwest(&client, &url, &token, &parent_block, HttpMethod::Patch).await?;

    Ok(res_json)
}
