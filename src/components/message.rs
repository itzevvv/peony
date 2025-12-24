use dioxus::prelude::*;

use crate::components::user_avatar::PeonyUserAvatar;

#[component]
pub fn Message(display_name : String, timestamp : String, message_content : String) -> Element {
    let passed_display_name = display_name.clone();
    
    rsx! {
        div {
            class: "message",
            PeonyUserAvatar { display_name: passed_display_name },
            div {
                class: "contents",
                div {
                    class: "header",
                    div {
                        class: "name",
                        "{display_name}"
                    }
                    span {
                        class: "timestamp",
                        "{timestamp}"
                    }
                },
                span {
                    class: "content",
                    "{message_content}"
                }
            }
        }
    }
}