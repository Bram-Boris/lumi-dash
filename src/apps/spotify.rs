use std::{path::PathBuf, time::Instant};

use chrono::Duration;
use embedded_graphics::{
    geometry::Point,
    pixelcolor::{Rgb888, WebColors},
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Triangle},
};
use image::{codecs::bmp::BmpEncoder, ImageDecoder};
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{AdditionalType, Country, FullTrack, Market},
    scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token,
};

use super::{app::App, launcher::Input};
use crate::{
    modules::{image::Image, module::Module},
    pixel_display::pixel_display::PixelDisplay,
};

pub struct Spotify {
    spotify: AuthCodeSpotify,
    current_song: Option<FullTrack>,
    duration: Option<Duration>,
    progress: Option<Duration>,
    cover_raw: Option<Vec<u8>>,
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

        Self {
            spotify,
            current_song: None,
            duration: None,
            progress: None,
            paused: true,
            last_update: Instant::now(),
            cover_raw: None,
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
                        match &self.current_song {
                            Some(s) => {
                                if s.name != e.name {
                                    self.cover_raw = Some(Self::download_cover(
                                        e.album.images.first().unwrap().url.clone(),
                                    ));
                                }
                                self.duration = Some(s.duration);
                            }
                            None => {
                                self.cover_raw = Some(Self::download_cover(
                                    e.album.images.first().unwrap().url.clone(),
                                ));
                            }
                        }
                        self.duration = Some(e.duration);
                        self.current_song = Some(e);
                        if playing.is_playing {
                            self.progress = playing.progress;
                            self.paused = false
                        } else {
                            self.paused = true
                        }
                    }
                    rspotify::model::PlayableItem::Episode(_) => (),
                },
                None => self.current_song = None,
            },
            Err(_) => todo!(),
        }

        match self
            .spotify
            .current_playback(Some(self.market), Some(&[AdditionalType::Track]))
        {
            Ok(p) => {
                self.progress = match p {
                    Some(playbackcontext) => playbackcontext.progress,
                    None => None,
                }
            }
            Err(_) => (),
        }

        self.last_update = Instant::now();
    }

    fn download_cover(imgurl: String) -> Vec<u8> {
        let resp = ureq::get(&imgurl).call().ok().unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        resp.into_reader().read_to_end(&mut bytes).unwrap();
        let mut image = image::load_from_memory(&bytes).unwrap();
        image = image.resize(32, 32, image::imageops::FilterType::Lanczos3);
        let mut bmpraw = Vec::new();
        let mut bmpencoder = BmpEncoder::<Vec<u8>>::new(&mut bmpraw);
        bmpencoder
            .encode(&image.into_bytes(), 32, 32, image::ColorType::Rgb8)
            .unwrap();

        bmpraw
    }

    fn draw_progress_bar(&mut self, display: &mut PixelDisplay) {
        let duration = Line {
            start: Point::new(36, 17),
            end: Point::new(59, 17),
        };

        let style = PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GRAY, 1);

        display.draw_line(duration, style);

        let max_seconds = self.duration.unwrap().num_seconds() as f32;
        let current_seconds = self.progress.unwrap().num_seconds() as f32;
        let ratio: f32 = current_seconds / max_seconds;
        let adjusted_progress = ((ratio * (59.0 - 36.0)) + 36.0).round() as i32;
        if adjusted_progress > 33 {
            let progress = Line {
                start: Point::new(36, 17),
                end: Point::new(adjusted_progress.try_into().unwrap(), 17),
            };
            let style = PrimitiveStyle::with_stroke(Rgb888::CSS_WHITE, 1);

            display.draw_line(progress, style)
        }
    }

    fn draw_playing_indicator(&mut self, display: &mut PixelDisplay) {
        match self.paused {
            false => {
                let style = PrimitiveStyle::with_stroke(Rgb888::CSS_LIME_GREEN, 2);

                let left = Line {
                    start: Point::new(45, 22),
                    end: Point::new(45, 28),
                };
                let right = Line {
                    start: Point::new(48, 22),
                    end: Point::new(48, 28),
                };
                display.draw_line(left, style);
                display.draw_line(right, style);
            }
            true => {
                let style = PrimitiveStyleBuilder::new()
                    .stroke_color(Rgb888::CSS_LIME_GREEN)
                    .stroke_width(1)
                    .fill_color(Rgb888::CSS_LIME_GREEN)
                    .build();

                let p1 = Point::new(44, 22);
                let p2 = Point::new(44, 28);
                let p3 = Point::new(51, 25);

                let triangle = Triangle {
                    vertices: [p1, p2, p3],
                };

                display.draw_triangle(triangle, style);
            }
        }
    }
}

impl App for Spotify {
    fn draw(&mut self, display: &mut PixelDisplay) {
        if self.cover_raw.is_some() {
            let raw = &self.cover_raw.as_mut().unwrap();
            let image = Image::new(raw);
            image.draw(Point::new(0, 0), display);
        }
        if self.last_update.elapsed().as_secs() > 2 {
            let _ = self.update_song();
        }
        match &self.current_song {
            Some(_e) => {
                // TODO: Text scrolling
                self.draw_progress_bar(display);
            }
            None => display.draw_text(&String::from("Nothing playing"), Point::new(2, 16)),
        }

        self.draw_playing_indicator(display);
    }

    fn input(&mut self, _input: Input) {}
}
