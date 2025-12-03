use leptos::logging::*;
use leptos::prelude::*;
use reqwest::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DogFactResponse {
    data: Vec<DogFactData>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DogFactData {
    attributes: DogFactAttributes,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DogFactAttributes {
    body: String,
}

pub async fn get_dog_facts() -> String {
    let client = Client::new();
    let res = client
        .get("https://dogapi.dog/api/v2/facts/")
        .send()
        .await
        .map_err(|e| format!("Failed: {}", e));

    if res.is_ok() {
        match res.unwrap().json::<DogFactResponse>().await {
            Ok(data) => {
                if let Some(first_fact) = data.data.first() {
                    return first_fact.attributes.body.clone();
                }
            }
            Err(err) => {
                log!("Parse error: {}", err);
            }
        }
    }

    "Failed to get dog fact".to_string()
}

#[component]
pub fn CompleteChatbox() -> impl IntoView {
    let (messages, set_messages) = signal(vec![Message {
        role: "Asistent".to_string(),
        content: "Kako mogu da ti pomognem?".to_string(),
    }]);

    let handle_submit = Action::new_local(move |input: &()| async move {
        let new_entry = get_dog_facts().await;
        set_messages.update(|msgs| {
            msgs.push(Message {
                role: "API".to_string(),
                content: new_entry,
            });
        });
    });

    view! {
        <h1>"Asistent"</h1>
        <div class="messages">
            <For
                each=move || messages.get().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(_, message)| {
                    let is_user = message.role == "korisnik";
                    let wrapper_class = if is_user { "message-wrapper korisnik" } else { "message-wrapper asistent" };
                    let message_class = if is_user { "message korisnik" } else { "message asistent" };

                    view! {
                        <div class=wrapper_class>
                            <div class=message_class>
                                <h3>{message.role}</h3>
                                <p>{message.content}</p>
                            </div>
                        </div>
                    }
                }
            />
        </div>
        <div class="input-container">
            <div class="input-wrapper">
                <input
                    type="text"
                   // prop:value=move || input.get()
                  //  on:input=move |ev| set_input.set(event_target_value(&ev))
                  //  on:keypress=handle_keypress
                    placeholder="Type your message..."
                  //  disabled=move || is_loading.get()
                />
                <button
                  on:click=move |_| {handle_submit.dispatch(());}
                  //  disabled=move || is_loading.get() || input.get().trim().is_empty()
                >
                    "→"
                </button>
            </div>
        </div>
    }
}
