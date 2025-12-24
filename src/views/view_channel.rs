use dioxus::{html::h3, prelude::*};

use crate::{components::message::Message, datastructures::{ChatChannel, ChatServer}, views::view_members::PeonyMembersView};

#[component]
pub fn PeonyChannelView(server : Option<ChatServer>, channel : Option<ChatChannel>) -> Element {
    rsx! {
        if let Some(channel) = channel {
            div {
                class: "sidebar-messages",
                
                ol {
                    class: "list reverse-list messages-list peony-scrollbar",
                    for _x in 0..4 {
                        li {
                            Message {
                                display_name: "Tickle Monster",
                                timestamp: "1:06 PM",
                                message_content: "I'm here to tickle you."
                            }
                        }
                    }
                }

                div {
                    class: "messages-bar",
                    contenteditable: "true",
                    textarea {
                        placeholder: "Message #{channel.name}"
                    }
                }
            }
        } else {
            div {
                h2 { "# Something fucked up!" }
                p { "Something fucked up." }
            },
            p { "You shouldn't be seeing this!" }
        }
    }
}