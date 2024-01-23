use std::path::PathBuf;

use embedded_graphics::geometry::Point;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{AdditionalType, Country, Market},
    scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token,
};

use crate::pixel_display::pixel_display::PixelDisplay;

use super::{app::App, launcher::Input};

pub struct Spotify {
    spotify: AuthCodeSpotify,
}

impl Spotify {
    pub fn new() -> Self {
        let creds = Credentials::from_env().unwrap();

        // Same for RSPOTIFY_REDIRECT_URI. You can also set it explictly:
        //
        // ```
        let oauth = OAuth {
            redirect_uri: "https://localhost:8888/callback".to_string(),
            scopes: scopes!("user-read-playback-state"),
            ..Default::default()
        };

        // Enabling automatic token refreshing in the config
        let config = Config {
            token_cached: true,
            ..Default::default()
        };
        let token = Token::from_cache(PathBuf::from(".spotify_token_cache.json")).unwrap();
        let expired = token.is_expired();
        let spotify;
        if !expired {
            spotify = AuthCodeSpotify::from_token(token);
        } else {
            spotify = AuthCodeSpotify::with_config(creds, oauth, config);
            let url = spotify.get_authorize_url(false).unwrap();
            spotify
                .prompt_for_token(&url)
                .expect("couldn't authenticate successfully");
        }

        Spotify { spotify }
    }
}

impl App for Spotify {
    fn draw(&self, display: &mut PixelDisplay) {
        let market = Market::Country(Country::Netherlands);
        let additional_types = [AdditionalType::Episode];
        let playing = self
            .spotify
            .current_playing(Some(market), Some(&additional_types))
            .unwrap()
            .unwrap()
            .item
            .unwrap();

        match playing {
            rspotify::model::PlayableItem::Track(e) => {
                display.draw_text(&String::from(e.name), Point::new(5, 5))
            }
            rspotify::model::PlayableItem::Episode(_) => todo!(),
        }
    }

    fn input(&mut self, _input: Input) {}
}
