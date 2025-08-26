#[derive(Clone, PartialEq)]
pub struct ChatChannel {
    pub id : String,
    pub name : String
}

#[derive(Clone, PartialEq)]
pub struct ChatServer {
    pub handle : String,
    pub name : String,
    pub did : String,
    pub channels : Vec<ChatChannel>,
}

#[derive(Clone)]
pub struct ChatUser {
    pub handle : String,
    pub did : String,
    pub display_name : String
}

#[derive(Clone)]
pub struct PeonyData {
    pub friends: Vec<ChatUser>,
    pub servers: Vec<ChatServer>,
}

pub fn testing_peony_data() -> PeonyData {
    PeonyData {
        friends: vec![
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
        ],
        servers: vec![
            ChatServer {
                name: "Prawn".to_string(),
                handle: "prawn.mreow.mrrp.lol".to_string(),
                did: "did:web:prawn.mreow.mrrp.lol".to_string(),
                channels: vec![
                    ChatChannel {
                        id: "SPO99GDFOIJGDF".to_string(),
                        name: "general".to_string()
                    }
                ]
            },
            ChatServer {
                name: "Einstein".to_string(),
                handle: "einstein.mreow.mrrp.lol".to_string(),
                did: "did:web:einstein.mreow.mrrp.lol".to_string(),
                channels: vec![
                    ChatChannel {
                        id: "ISDJIOFDFJIOG".to_string(),
                        name: "genderal".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG0".to_string(),
                        name: "test-1".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG1".to_string(),
                        name: "test-2".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG2".to_string(),
                        name: "test-3".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG3".to_string(),
                        name: "test-4".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG4".to_string(),
                        name: "test-5".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG5".to_string(),
                        name: "test-6".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG6".to_string(),
                        name: "test-7".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG7".to_string(),
                        name: "test-8".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG8".to_string(),
                        name: "test-9".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG9".to_string(),
                        name: "test-10".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG90".to_string(),
                        name: "test-11".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG91".to_string(),
                        name: "test-12".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG92".to_string(),
                        name: "test-13".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG93".to_string(),
                        name: "test-14".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG94".to_string(),
                        name: "test-15".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG95".to_string(),
                        name: "test-16".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG96".to_string(),
                        name: "test-17".to_string()
                    },

                    ChatChannel {
                        id: "ISDJIOFDFJIOG90".to_string(),
                        name: "test-18".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG97".to_string(),
                        name: "test-19".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG98".to_string(),
                        name: "test-20".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG99".to_string(),
                        name: "test-21".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG990".to_string(),
                        name: "test-22".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG991".to_string(),
                        name: "test-23".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG992".to_string(),
                        name: "test-24".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG993".to_string(),
                        name: "test-25".to_string()
                    },
                    ChatChannel {
                        id: "ISDJIOFDFJIOG994".to_string(),
                        name: "test-26".to_string()
                    },
                ]
            },
            ChatServer {
                name: "Orangutan".to_string(),
                handle: "orangutan.mreow.mrrp.lol".to_string(),
                did: "did:web:orangutan.mreow.mrrp.lol".to_string(),
                channels: vec![
                    ChatChannel {
                        id: "ISDGIOJDFGOIJDF".to_string(),
                        name: "gendoorgal".to_string()
                    }
                ]
            },
            ChatServer {
                name: "The Pixel Empire".to_string(),
                handle: "tpe.com".to_string(),
                did: "did:web:tpe.com".to_string(),
                channels: vec![
                    ChatChannel {
                        id: "OPJ9RTIOJGX".to_string(),
                        name: "general".to_string()
                    },
                    ChatChannel {
                        id: "890UE354T".to_string(),
                        name: "artwork".to_string()
                    },
                    ChatChannel {
                        id: "U89034U89F".to_string(),
                        name: "creations".to_string()
                    },
                    ChatChannel {
                        id: "ERTGGJIOE".to_string(),
                        name: "tech".to_string()
                    },
                    ChatChannel {
                        id: "IJERRJIOG".to_string(),
                        name: "gaming".to_string()
                    },

                    ChatChannel {
                        id: "ETRJDFDJA".to_string(),
                        name: "media".to_string()
                    },
                    ChatChannel {
                        id: "3549U0T4".to_string(),
                        name: "memes-and-spam".to_string()
                    },
                    ChatChannel {
                        id: "I903W490".to_string(),
                        name: "music".to_string()
                    },
                    ChatChannel {
                        id: "132I09434I95".to_string(),
                        name: "animals".to_string()
                    },
                    ChatChannel {
                        id: "23JEGREWF".to_string(),
                        name: "slop".to_string()
                    },
                    ChatChannel {
                        id: "J34E4JRJERG".to_string(),
                        name: "happy-happy-positivity".to_string()
                    },
                    ChatChannel {
                        id: "12435TRF".to_string(),
                        name: "news".to_string()
                    },
                    ChatChannel {
                        id: "24tig9tfew".to_string(),
                        name: "personal".to_string()
                    },
                    ChatChannel {
                        id: "R43WF9I3GW".to_string(),
                        name: "clop-open-rp".to_string()
                    },
                    ChatChannel {
                        id: "W4F9sSDGDRE".to_string(),
                        name: "lux".to_string()
                    },
                    ChatChannel {
                        id: "R9DIFERERG".to_string(),
                        name: "tpetopia".to_string()
                    },
                    ChatChannel {
                        id: "R9DIFERERE".to_string(),
                        name: "test".to_string()
                    },
                    ChatChannel {
                        id: "R9DIFERERF".to_string(),
                        name: "test-2".to_string()
                    },
                    ChatChannel {
                        id: "R9DIFERERD".to_string(),
                        name: "test-3".to_string()
                    },
                    ChatChannel {
                        id: "R9DIFERER1".to_string(),
                        name: "test-4".to_string()
                    },
                    ChatChannel {
                        id: "R9DIFERER2".to_string(),
                        name: "test-5".to_string()
                    },
                    ChatChannel {
                        id: "R9DIFERER3".to_string(),
                        name: "test-6".to_string()
                    },
                    ChatChannel {
                        id: "R9DIFERER4".to_string(),
                        name: "test-7".to_string()
                    },
                ]
            },
            ChatServer {
                name: "Nougat".to_string(),
                handle: "nougat.mreow.mrrp.lol".to_string(),
                did: "did:web:nougat.mreow.mrrp.lol".to_string(),
                channels: vec![
                    ChatChannel {
                        id: "354U89GER89UG".to_string(),
                        name: "nougat-general".to_string()
                    }
                ]
            },
            ChatServer {
                name: "Yvette".to_string(),
                handle: "yvette.mreow.mrrp.lol".to_string(),
                did: "did:web:yvette.mreow.mrrp.lol".to_string(),
                channels: vec![
                    ChatChannel {
                        id: "8UE5GGREER".to_string(),
                        name: "yvette-general".to_string()
                    }
                ]
            },
            ChatServer {
                name: "Horror".to_string(),
                handle: "horror.mreow.mrrp.lol".to_string(),
                did: "did:web:horror.mreow.mrrp.lol".to_string(),
                channels: vec![
                    ChatChannel {
                        id: "IJOSEIOJG".to_string(),
                        name: "horror-general".to_string()
                    }
                ]
            },
            ChatServer {
                name: "Igloo".to_string(),
                handle: "igloo.mreow.mrrp.lol".to_string(),
                did: "did:web:igloo.mreow.mrrp.lol".to_string(),
                channels: vec![
                    ChatChannel {
                        id: "98ERGR90UERGU".to_string(),
                        name: "cold-general".to_string()
                    }
                ]
            },
        ]
    }
}