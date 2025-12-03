use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Clone, Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
}

pub async fn get_dog_facts() {}

#[component]
pub fn Chatbox() -> impl IntoView {
    let (messages, set_messages) = signal(vec![Message {
        role: "Asistent".to_string(),
        content: "Kako mogu da ti pomognem?".to_string(),
    }]);

    let (input, set_input) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);

    let messages_end_ref = create_node_ref::<html::Div>();

    let scroll_to_bottom = move || {
        if let Some(element) = messages_end_ref.get() {
            element.scroll_into_view();
        }
    };

    create_effect(move |_| {
        messages.get();
        scroll_to_bottom();
    });

    let handle_submit = move || {
        let input_value = input.get().trim().to_string();

        if input_value.is_empty() || is_loading.get() {
            return;
        }

        set_input.set(String::new());
        let mut current_messages = messages.get();
        current_messages.push(Message {
            role: "user".to_string(),
            content: input_value.clone(),
        });
        set_messages.set(current_messages.clone());
        set_is_loading.set(true);

        spawn_local(async move {
            let request = ApiRequest {
                model: "claude-sonnet-4-20250514".to_string(),
                max_tokens: 1000,
                messages: current_messages
                    .iter()
                    .map(|m| Message {
                        role: m.role.clone(),
                        content: m.content.clone(),
                    })
                    .collect(),
            };

            let client = reqwest::Client::new();
            let response = client
                .get("https://dogapi.dog/api/v2/facts/")
                .send()
                .await
                .map_err(|e| format!("Request failed :{}", e));

            let assistant_message = match response {
                Ok(resp) => match resp.json::<ApiResponse>().await {
                    Ok(data) => data
                        .content
                        .iter()
                        .find(|block| block.content_type == "text")
                        .and_then(|block| block.text.clone())
                        .unwrap_or_else(|| "I apologize, but I encountered an error.".to_string()),
                    Err(_) => "I apologize, but I encountered an error processing your request."
                        .to_string(),
                },
                Err(_) => {
                    "I apologize, but I encountered an error processing your request.".to_string()
                }
            };

            let mut updated_messages = messages.get();
            updated_messages.push(Message {
                role: "assistant".to_string(),
                content: assistant_message,
            });
            set_messages.set(updated_messages);
            set_is_loading.set(false);
        });
    };

    let handle_keypress = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            handle_submit();
        }
    };

    view! {
        <div class="container">
            <div class="chatbot">
                <div class="header">
                    <h1>"Asistent"</h1>
                </div>

                <div class="messages">
                    <For
                        each=move || messages.get().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(_, message)| {
                            let is_user = message.role == "user";
                            let wrapper_class = if is_user { "message-wrapper user" } else { "message-wrapper assistant" };
                            let message_class = if is_user { "message user" } else { "message assistant" };

                            view! {
                                <div class=wrapper_class>
                                    <div class=message_class>
                                        <p>{message.content}</p>
                                    </div>
                                </div>
                            }
                        }
                    />

                    {move || is_loading.get().then(|| view! {
                        <div class="message-wrapper assistant">
                            <div class="message assistant">
                                <p>"Thinking..."</p>
                            </div>
                        </div>
                    })}

                    <div node_ref=messages_end_ref></div>
                </div>

                <div class="input-container">
                    <div class="input-wrapper">
                        <input
                            type="text"
                            prop:value=move || input.get()
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                            on:keypress=handle_keypress
                            placeholder="Type your message..."
                            disabled=move || is_loading.get()
                        />
                        <button
                            on:click=move |_| handle_submit()
                            disabled=move || is_loading.get() || input.get().trim().is_empty()
                        >
                            "→"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
