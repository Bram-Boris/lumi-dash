use std::{path::PathBuf, sync::RwLock, thread::sleep, time::Instant};

use chrono::Duration;
use embedded_graphics::{
    geometry::Point,
    pixelcolor::{Rgb888, WebColors},
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Triangle},
};
use image::{codecs::bmp::BmpEncoder, EncodableLayout};
use rspotify::{
    clients::OAuthClient,
    model::{AdditionalType, Country, FullTrack, Market},
    scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token,
};
use std::sync::mpsc;
use std::thread;

use super::{app::App, launcher::Input};
use crate::{
    modules::{image::Image, module::Module},
    pixel_display::pixel_display::PixelDisplay,
};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Spotify {
    sender: Sender<Input>,
    data: Arc<RwLock<SpotifyData>>,
    prev_data: Option<SpotifyData>,
    is_rendering: Arc<RwLock<bool>>,
}

pub struct SpotifyClient {
    spotify: AuthCodeSpotify,
    last_update: Instant,
    device_id: Option<String>,
    market: Market,
    current_song: Option<FullTrack>,
    paused: bool,
}

#[derive(Clone)]
pub struct SpotifyData {
    current_song: Option<FullTrack>,
    duration: Option<Duration>,
    progress: Option<Duration>,
    device_id: Option<String>,
    cover_raw: Option<Vec<u8>>,
    paused: bool,
}

impl Spotify {
    pub fn new(input_tx: Sender<Input>) -> Self {
        let creds = Credentials::from_env().unwrap();

        let oauth = OAuth {
            redirect_uri: "https://localhost:8888/callback".to_string(),
            scopes: scopes!("user-read-playback-state", "user-modify-playback-state"),
            ..Default::default()
        };

        let config = Config {
            token_cached: true,
            ..Default::default()
        };

        let token: Option<Token> =
            match Token::from_cache(PathBuf::from(".spotify_token_cache.json")) {
                Ok(e) => {
                    if !e.is_expired() {
                        Some(e);
                    }
                    None
                }
                Err(_) => None,
            };

        let spotifyapi;
        match token {
            Some(t) => spotifyapi = AuthCodeSpotify::from_token(t),
            None => {
                spotifyapi = AuthCodeSpotify::with_config(creds, oauth, config);
                let url = spotifyapi.get_authorize_url(false).unwrap();
                spotifyapi
                    .prompt_for_token(&url)
                    .expect("couldn't authenticate successfully");
            }
        }

        let market = Market::Country(Country::Netherlands);
        let mut client = SpotifyClient {
            current_song: None,
            last_update: Instant::now(),
            device_id: None,
            spotify: spotifyapi,
            market,
            paused: false,
        };

        let data = Arc::new(RwLock::new(SpotifyData {
            current_song: None,
            duration: None,
            progress: None,
            device_id: None,
            paused: true,
            cover_raw: None,
        }));

        let (tx, rx) = mpsc::channel();

        let spotify = Spotify {
            sender: tx,
            data: data.clone(),
            prev_data: None,
            is_rendering: Arc::new(RwLock::new(false)),
        };

        let is_rendering = spotify.is_rendering.clone();

        thread::spawn(move || {
            let input_tx = input_tx.clone();
            let mut elapsed = Instant::now();
            loop {
                match rx.try_recv() {
                    Ok(input) => match input {
                        Input::Next => client.next_track(),
                        Input::Prev => client.previous_track(),
                        Input::Pressed => client.toggle_playback(),
                        Input::Held => (),
                    },
                    Err(_) => (),
                }
                sleep(std::time::Duration::from_secs(1));
                if elapsed.elapsed().as_secs() > 2 {
                    let new = client.update_data();
                    let mut data = data.write().unwrap();
                    if data.current_song.is_none()
                        && new.current_song.is_some()
                        && !*is_rendering.read().unwrap()
                    {
                        input_tx.send(Input::Held);
                    }
                    if data.current_song.is_some()
                        && new.current_song.is_none()
                        && *is_rendering.read().unwrap()
                    {
                        input_tx.send(Input::Held);
                    }
                    data.current_song = new.current_song;
                    data.duration = new.duration;
                    data.progress = new.progress;
                    data.paused = new.paused;
                    if new.cover_raw.is_some() {
                        data.cover_raw = new.cover_raw;
                    }
                    drop(data);

                    elapsed = Instant::now()
                };
            }
        });

        spotify
    }
}

impl SpotifyClient {
    fn update_data(&mut self) -> SpotifyData {
        let mut current_song = None;
        let mut duration = None;
        let mut progress = None;
        let mut device_id = None;
        let mut paused = true;
        let mut cover_raw = None;

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
                                    cover_raw = Some(Self::download_cover(
                                        e.album.images.first().unwrap().url.clone(),
                                    ));
                                }
                                duration = Some(s.duration);
                            }
                            None => {
                                cover_raw = Some(Self::download_cover(
                                    e.album.images.first().unwrap().url.clone(),
                                ));
                            }
                        }
                        duration = Some(e.duration);
                        current_song = Some(e.clone());
                        self.current_song = Some(e.clone());
                        if playing.is_playing {
                            progress = playing.progress;
                            paused = false;
                            self.paused = false;
                        } else {
                            paused = true;
                            self.paused = false;
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
                progress = match p {
                    Some(ref playbackcontext) => playbackcontext.progress,
                    None => None,
                };
                self.device_id = match p {
                    Some(ref playbackcontext) => playbackcontext.device.id.clone(),
                    None => None,
                };
                device_id = match p {
                    Some(ref playbackcontext) => playbackcontext.device.id.clone(),
                    None => None,
                }
            }
            Err(_) => (),
        }

        self.last_update = Instant::now();

        SpotifyData {
            current_song,
            duration,
            progress,
            device_id,
            paused,
            cover_raw,
        }
    }

    fn download_cover(imgurl: String) -> Vec<u8> {
        let resp = ureq::get(&imgurl).call().ok().unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        resp.into_reader().read_to_end(&mut bytes).unwrap();
        let mut image = image::load_from_memory(&bytes).unwrap();
        image = image.resize_to_fill(32, 32, image::imageops::FilterType::Lanczos3);
        let mut bmpraw = Vec::new();
        let mut bmpencoder = BmpEncoder::<Vec<u8>>::new(&mut bmpraw);
        bmpencoder
            .encode(&image.as_bytes(), 32, 32, image.color())
            .unwrap();

        bmpraw
    }

    fn next_track(&mut self) {
        self.spotify.next_track(self.device_id.as_deref()).unwrap();
    }

    fn previous_track(&mut self) {
        self.spotify
            .previous_track(self.device_id.as_deref())
            .unwrap();
    }

    fn toggle_playback(&mut self) {
        match self.paused {
            true => self
                .spotify
                .resume_playback(self.device_id.as_deref(), None)
                .unwrap(),
            false => self
                .spotify
                .pause_playback(self.device_id.as_deref())
                .unwrap(),
        }
    }
}

impl SpotifyData {
    fn draw_progress_bar(&self, display: &mut PixelDisplay) {
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
        let progress = Line {
            start: Point::new(36, 17),
            end: Point::new(adjusted_progress.try_into().unwrap(), 17),
        };
        let style = PrimitiveStyle::with_stroke(Rgb888::CSS_WHITE, 1);

        display.draw_line(progress, style)
    }

    fn draw_playing_indicator(&self, display: &mut PixelDisplay) {
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
        match self.data.try_read() {
            Ok(d) => {
                d.draw(display);
                self.prev_data = Some(d.clone());
            }
            Err(d) => self.prev_data.clone().unwrap().draw(display),
        }
    }

    fn input(&mut self, input: Input) {
        self.sender.send(input);
    }

    fn enable(&mut self) {
        let mut data = self.is_rendering.write().unwrap();
        *data = true
    }

    fn disable(&mut self) {
        let mut data = self.is_rendering.write().unwrap();
        *data = false
    }
}

impl SpotifyData {
    fn draw(&self, display: &mut PixelDisplay) {
        match &self.current_song {
            Some(t) => {
                // TODO: Text scrolling & Album name
                display.draw_text(&t.name, Point::new(33, 10));
                self.draw_progress_bar(display);
                if self.cover_raw.is_some() {
                    let bytes = &self.cover_raw.clone().unwrap().to_vec();
                    let image = Image::new(bytes);
                    image.draw(Point::new(0, 0), display);
                }
            }
            None => display.draw_text(&String::from("Nothing playing"), Point::new(2, 16)),
        }

        self.draw_playing_indicator(display);
    }
}
