use serde_json::Value;

pub fn generate_new_page(database_id: String, first_input: &str, frequency: &str) -> Value {
    serde_json::json!({
        "parent": { "database_id": database_id },
        "properties": {
            "Vocabulary": {
                "title": [
                    {
                        "text": {
                            "content": first_input
                        }
                    }
                ]
            },
            "頻出度": {
                "select": {
                    "name": frequency
                }
            },
            "習得度": {
                "status": {
                    "name": "インプット中"
                }
            }
        }
    })
}

pub fn generate_children_block(content: &str) -> Value {
    serde_json::json!({
        "children": [
        {
            "object": "block",
            "type": "paragraph",
            "paragraph": {
                "text": [
                    {
                        "type": "text",
                        "text": {
                            "content": content
                        }
                    }
                ]
            }
        }
    ]
    })
}

pub fn generate_parent_block() -> Value {
    serde_json::json!({
        "children": [
            {
                "object": "block",
                "type": "callout",
                "callout": {
                    "text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "発音",
                            },
                            "annotations": {
                                "color": "blue",
                            },
                        }
                    ],
                    "icon": {
                        "type": "emoji",
                        "emoji": "🗣️"
                    },
                }
            },
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
                        "emoji": "📖"
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
    })
}
