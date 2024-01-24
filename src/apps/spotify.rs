use std::{path::PathBuf, time::Instant};

use embedded_graphics::geometry::Point;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{AdditionalType, Country, CurrentlyPlayingContext, FullTrack, Market, PlayableItem},
    scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token,
};

use crate::pixel_display::pixel_display::PixelDisplay;

use super::{app::App, launcher::Input};

pub struct Spotify {
    spotify: AuthCodeSpotify,
    current_song: Option<FullTrack>,
    last_update: Instant,
    paused: bool,
    market: Market,
}

impl Spotify {
    pub fn new() -> Self {
        let creds = Credentials::from_env().unwrap();

        let oauth = OAuth {
            redirect_uri: "https://localhost:8888/callback".to_string(),
            scopes: scopes!("user-read-playback-state"),
            ..Default::default()
        };

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

        let market = Market::Country(Country::Netherlands);

        Spotify {
            spotify,
            current_song: None,
            paused: false,
            last_update: Instant::now(),
            market,
        }
    }

    fn update_song(&mut self) {
        match self
            .spotify
            .current_playing(Some(self.market), Some(&[AdditionalType::Track]))
        {
            Ok(e) => match e {
                Some(playing) => match playing.item.unwrap() {
                    rspotify::model::PlayableItem::Track(e) => {
                        self.current_song = Some(e);
                        if playing.is_playing {
                            self.paused = false
                        } else {
                            self.paused = true
                        }
                    }
                    rspotify::model::PlayableItem::Episode(_) => (),
                },
                None => {}
            },
            Err(_) => todo!(),
        }
        self.last_update = Instant::now();
    }
}

impl App for Spotify {
    fn draw(&mut self, display: &mut PixelDisplay) {
        if self.last_update.elapsed().as_secs() > 2 {
            let _ = self.update_song();
        }
        match &self.current_song {
            Some(e) => display.draw_text(&e.name, Point::new(5, 16)),
            None => display.draw_text(
                &String::from("Nothing playing currently"),
                Point::new(5, 16),
            ),
        }
    }

    fn input(&mut self, _input: Input) {}
}
