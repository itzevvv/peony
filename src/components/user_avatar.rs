use dioxus::prelude::*;

#[component]
pub fn PeonyUserAvatar(display_name : String) -> Element {
    let split_name = display_name.split(" ");
    let mut characters = String::new();

    for subs in split_name {
        characters.push(subs.chars().next().unwrap());
    }
    
    characters.truncate(3);

    rsx! {
        div {
            class: "avatar",
            "{characters}"
        },
    }
}

#[component]
pub fn PeonyUserAvatarSmall(display_name : String) -> Element {
    let split_name = display_name.split(" ");
    let mut characters = String::new();

    for subs in split_name {
        characters.push(subs.chars().next().unwrap());
    }
    
    characters.truncate(3);

    rsx! {
        div {
            class: "avatar avatar-small",
            "{characters}"
        },
    }
}