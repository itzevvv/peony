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
                class: "message-header",
                div {
                    class: "avatar",
                    "{characters}"
                },
                div {
                    class: "message-header-info",
                    "{display_name}",
                    
                    span {
                        " • {timestamp}"
                    }   

                    p { "{message_content}" }
                },
            }
        }
    }
}