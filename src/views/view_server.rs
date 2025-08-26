use dioxus::prelude::*;

use crate::{
    components::channel_list::PeonyChannelList,
    datastructures::PeonyData,
    views::view_messages::PeonyMessagesView,
};

#[component]
pub fn PeonyServerView(server_id: String, channel_id : String) -> Element {
    let servers = use_context::<PeonyData>().servers;
    
    let mut found_server = None;
    let mut found_channel = None;
    
    for server in servers {
        if server.did == server_id {
            found_server = Some(server.clone());

            for channel in server.channels {
                if channel.id == channel_id {
                    found_channel = Some(channel.clone());
                    break;
                }
            }
            break;
        }
    }


    rsx! {
        PeonyChannelList { server: found_server.clone(), current_channel: found_channel.clone() }
        PeonyMessagesView { channel: found_channel.clone() }
    }
}