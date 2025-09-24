use dioxus::prelude::*;
use crate::datastructures::*;

#[component]
pub fn PeonyChannelsView(server : Option<ChatServer>, current_channel : Option<ChatChannel>) -> Element {
    rsx! {
        div {
            class: "sidebar-channels",
            if let Some(server) = server {
                h3 {
                    "{server.name}"
                }

                ol {
                    class: "list peony-scrollbar channels-list peony-scrollbar-hover",
                    
                    for channel in &server.channels {
                        li {
                            Link {
                                class: "linkable",
                                to: "/servers/{server.did}/channels/{channel.id}",

                                div {
                                    key: "{channel.id}",
                                    class: if let Some(current_channel) = &current_channel {
                                        if current_channel.id == channel.id {
                                            "channel-name-selected"
                                        } else {
                                            "channel-name"
                                        }
                                    } else {
                                        "channel-name"
                                    },
                                    p { "# {channel.name}" }
                                }
                            }
                        }
                    }
                }
            } else {
                h2 {
                    "Something fucked up!"
                }
            }
        }
    }
}