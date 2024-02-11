use crate::phases::{Phase, PhaseBehavior};
use crate::tilemap::Tilemap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::VecDeque;

use macroquad::prelude::*;


#[derive(Debug, Clone)]
enum StoryEvent {
    LoadTilemap{path: String},
    WaitForInput,
}

impl StoryEvent {
    fn new(source: String) -> Option<Self> {
        let source = source.trim();

        if source.is_empty() {
            return None;
        }

        if source.starts_with("#") {
            return None;
        }

        let (event, params) = {
            if let Some((event, params)) = source.split_once(" ") {
                (event, params)
            } else {
                (source, "")
            }
        };


        return match event {
            "lt" => Some(Self::LoadTilemap { path: params.to_string() }),
            "wfi" => Some(Self::WaitForInput),
            _ => None
        };
    }
}


pub struct Story {
    events: VecDeque<StoryEvent>,
    tilemap: Option<Tilemap>
}

impl<P: AsRef<Path>> PhaseBehavior<P> for Story {
    async fn setup(story_path: P) -> Self {
        let mut events = VecDeque::new();

        let file = File::open(story_path).unwrap();
        let lines = BufReader::new(file).lines();
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }
                if let Some(event) = StoryEvent::new(line) {
                    events.push_back(event);
                }
            }
        }

        Self {
            events,
            tilemap: None,
        }
    }

    async fn update(&mut self) -> Option<Phase> {
        if self.events.is_empty() {
            return Some(Phase::Menu);
        }

        match self.events.front().unwrap() {
            StoryEvent::LoadTilemap { path } => {
                self.tilemap = Tilemap::new(path.clone()).await;
                self.events.pop_front();
            }
            StoryEvent::WaitForInput => {
                if is_key_down(KeyCode::Enter) {
                    self.events.pop_front();
                }
            }
        }

        clear_background(BLACK);

        if let Some(tilemap) = &self.tilemap {
            tilemap.draw();
        }

        None
    }
}