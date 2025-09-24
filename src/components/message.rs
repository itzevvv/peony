use dioxus::prelude::*;

#[component]
pub fn Message(display_name : String, timestamp : String, message_content : String) -> Element {
    let split_name = display_name.split(" ");
    let mut characters = String::new();

    for subs in split_name {
        characters.push(subs.chars().next().unwrap());
    }
    
    characters.truncate(3);

    rsx! {
        div {
            class: "message",
            div {
                class: "avatar",
                "{characters}"
            },
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