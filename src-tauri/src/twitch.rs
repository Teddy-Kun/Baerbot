use twitch_api::HelixClient;

pub mod auth;

type TwitchClient = HelixClient<'static, reqwest::Client>;
