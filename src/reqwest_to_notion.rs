use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};

pub async fn run() -> Result<(), String> {
    dotenv().ok();
    let token = std::env::var("TOKEN").unwrap();
    let database_id = std::env::var("DBID").unwrap();
    let url = format!("https://api.notion.com/v1/pages",);

    let client = Client::new();

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
    // let res = client
    //     .post(&url)
    //     .header("Authorization", format!("Bearer {}", token))
    //     .header("Notion-Version", "2021-08-16")
    //     .json(&new_page_data)
    //     .send()
    //     .await
    //     .map_err(|e| e.to_string())?;
    // let res_text = res.text().await.map_err(|e| e.to_string())?;
    // let res_json: serde_json::Value = serde_json::from_str(&res_text).map_err(|e| e.to_string())?;
    // let pretty_json_string = serde_json::to_string_pretty(&res_json).map_err(|e| e.to_string())?;
    // println!("{}", pretty_json_string);

    if let Ok(results) = create_parent_block(&client, &token).await {
        let res = results["results"].as_array().expect("resultsでエラー");

        // parent_blockのchildrenの処理をこのスコープでおこなう
        match create_children_block(&client, &token, &res, 0).await {
            Ok(_) => {
                println!("create children");
            }
            Err(e) => {
                println!("Err create children: {}", e);
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
) -> Result<Value, String> {
    let some_number_result = parent_block_value[number].clone();
    let parent_block_id = some_number_result["id"].as_str().expect("parentエラー");

    let url = format!(
        "https://api.notion.com/v1/blocks/{}/children",
        parent_block_id
    );

    let meaning_block = json!({
        "children": [
            {
                "object": "block",
                "type": "paragraph",
                "paragraph": {
                    "text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "これは新しいパラグラフです。"
                            }
                        }
                    ]
                }
            }
        ]
    });

    let res = client
        .patch(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", "2021-08-16")
        .json(&meaning_block)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let res_text = res.text().await.map_err(|e| e.to_string())?;
    let res_json: Value = serde_json::from_str(&res_text).map_err(|e| e.to_string())?;
    let pretty_json_string = serde_json::to_string_pretty(&res_json).unwrap_or_default();
    println!("Block id: {}", parent_block_id);
    println!("Block Content: {}", pretty_json_string);

    Ok(res_json)
}

async fn create_parent_block(client: &Client, token: &str) -> Result<Value, String> {
    // 1段目のBLOCK
    // let parent_page_id = res_json["id"].as_str().unwrap_or("");
    let parent_page_id = "81a99280-fc96-455f-961e-eca8197b386e";

    let url = format!(
        "https://api.notion.com/v1/blocks/{}/children",
        parent_page_id
    );

    let meaning_block = json!({
        "children": [
            {
                "object": "block",
                "type": "toggle",
                "toggle": {
                    "text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "意味",
                            },
                            "annotations": {
                                "color": "blue",
                            },
                        }
                    ]
                },
            },
            {
                "object": "block",
                "type": "toggle",
                "toggle": {
                    "text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "語源",
                            },
                            "annotations": {
                                "color": "blue",
                            },
                        }
                    ]
                },
            },
            {
                "object": "block",
                "type": "callout",
                "callout": {
                    "text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "コロケーション",
                            },
                            "annotations": {
                                "color": "blue",
                            },
                        }
                    ],
                    "icon": {
                        "type": "emoji",
                        "emoji": "📎"
                    },
                }
            },
            {
                "object": "block",
                "type": "callout",
                "callout": {
                    "text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "例文",
                            },
                            "annotations": {
                                "color": "blue",
                            },
                        }
                    ],
                    "icon": {
                        "type": "emoji",
                        "emoji": "📎"
                    },
                }
            },
            {
                "object": "block",
                "type": "callout",
                "callout": {
                    "text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "イメージ",
                            },
                        }
                    ],
                    "icon": {
                        "type": "emoji",
                        "emoji": "🖼️"
                    },
                }
            },
            {
                "object": "block",
                "type": "callout",
                "callout": {
                    "text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "自由記述",
                            },
                        }
                    ],
                    "icon": {
                        "type": "emoji",
                        "emoji": "✏️"
                    },
                }
            },
        ],
    });

    let res = client
        .patch(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", "2021-08-16")
        .json(&meaning_block)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let res_text = res.text().await.map_err(|e| e.to_string())?;
    let res_json: Value = serde_json::from_str(&res_text).map_err(|e| e.to_string())?;
    let pretty_json_string = serde_json::to_string_pretty(&res_json).unwrap_or_default();
    println!("Block Content: {}", pretty_json_string);

    Ok(res_json)
}
