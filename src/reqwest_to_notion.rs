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
    let res_json: serde_json::Value = serde_json::from_str(&res_text).map_err(|e| e.to_string())?;
    let pretty_json_string = serde_json::to_string_pretty(&res_json).unwrap_or_default();
    println!("Block Content: {}", pretty_json_string);

    // トグルのchildren作成(合計5つ作成)
    let results = res_json["results"].as_array().expect("resultsでエラー");
    let first_result = &results[0];
    let parent_block_id = first_result["id"].as_str().expect("parentエラー");

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
    let res_json: serde_json::Value = serde_json::from_str(&res_text).map_err(|e| e.to_string())?;
    let pretty_json_string = serde_json::to_string_pretty(&res_json).unwrap_or_default();
    println!("Block id: {}", parent_block_id);
    println!("Block Content: {}", pretty_json_string);

    Ok(())
}
