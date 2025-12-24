use dioxus::prelude::*;
use crate::{components::user_avatar::{PeonyUserAvatar, PeonyUserAvatarSmall}, datastructures::*};

#[component]
pub fn PeonyMembersView(server : Option<ChatServer>, current_channel : Option<ChatChannel>) -> Element {
    let members = vec![
        ChatUser {
                handle: "one.users.mrrp.lol".to_string(),
                did: "did:web:one.users.mrrp.lol".to_string(),
                display_name: "User One".to_string()
            },
            ChatUser {
                handle: "two.users.mrrp.lol".to_string(),
                did: "did:web:two.users.mrrp.lol".to_string(),
                display_name: "User Two".to_string()
            },
            ChatUser {
                handle: "three.users.mrrp.lol".to_string(),
                did: "did:web:three.users.mrrp.lol".to_string(),
                display_name: "User Three".to_string()
            },
            ChatUser {
                handle: "four.users.mrrp.lol".to_string(),
                did: "did:web:four.users.mrrp.lol".to_string(),
                display_name: "User Four".to_string()
            },
            ChatUser {
                handle: "five.users.mrrp.lol".to_string(),
                did: "did:web:five.users.mrrp.lol".to_string(),
                display_name: "User Five".to_string()
            },
            ChatUser {
                handle: "six.users.mrrp.lol".to_string(),
                did: "did:web:six.users.mrrp.lol".to_string(),
                display_name: "User Six".to_string()
            },
    ];

    rsx! {
        div {
            class: "sidebar-members",
            if let Some(server) = server {
                ol {
                    class: "list peony-scrollbar peony-scrollbar-hover",

                    li {
                        class: "member-list-section-header",
                        "Online - {members.len()}"
                    }

                    for member in &members {
                        li {
                            div {
                                key: "{member.did}",
                                class: "member-list-section-item",
                                PeonyUserAvatarSmall { display_name: member.display_name.clone() }
                                div {
                                    class: "member-list-item-content",
                                    div {
                                        class: "member-list-item-name",
                                        "{member.display_name}"
                                    }
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