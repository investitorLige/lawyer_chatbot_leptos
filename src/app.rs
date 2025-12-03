use crate::Components::chatbox::Chatbox;
use crate::Components::complete_chatbox::CompleteChatbox;
use leptos::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    view! {

        <CompleteChatbox/>
    }
}
