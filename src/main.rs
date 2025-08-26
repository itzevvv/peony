pub mod components;
pub mod views;
pub mod datastructures;

use dioxus::{prelude::{*}};

use crate::{
    components::server_list::PeonyServerList,
    datastructures::testing_peony_data,
    views::{
        view_friends::PeonyFriendsView,
        view_dms::PeonyDMsView,
        view_server::PeonyServerView,
    },
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Peony)]
    #[route("/")]
    PeonyFriendsView,
    #[route("/messages/:user_did")]
    PeonyDMsView { user_did : String },
    #[route("/servers/:server_id/channels/:channel_id")]
    PeonyServerView { server_id: String, channel_id : String },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let peony_data = use_context_provider(|| testing_peony_data());

    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

#[component]
fn Peony() -> Element {
    rsx! {
        div {
            class: "container",

            PeonyServerList {}
            Outlet::<Route> {}
        }
    }
}