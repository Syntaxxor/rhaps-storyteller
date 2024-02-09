use macroquad::{prelude::*, ui::root_ui};
use crate::phases::*;

use std::{fs::{read_dir, File}, io::{BufRead, BufReader}};

#[derive(Copy, Clone)]
enum MenuState {
    Main,
    Options,
    StorySelector,
}

struct StoryInfo {
    name: String,
    description: String,
    path: String,
}

pub struct Menu {
    state: MenuState,
    available_stories: Vec<StoryInfo>,
}

impl PhaseBehavior<()> for Menu {
    async fn setup(_setup_data: ()) -> Self {
        let mut available_stories = Vec::new();

        let files = read_dir("stories/").unwrap()
            .filter(|v| {
                if let Ok(entry) = v {
                    if let Some(extension) = entry.path().extension() {
                        return extension == "story";
                    }
                }
                return false;
            })
            .map(|v| {
                v.unwrap().path()
            });
        
        for story_path in files {
            let path = story_path.to_str().unwrap().to_string();

            let story_file = File::open(story_path).unwrap();
            let mut lines = BufReader::new(story_file).lines();
            let name = lines.next().unwrap().unwrap()[1..].to_string();
            let description = lines.next().unwrap().unwrap()[1..].to_string();

            available_stories.push(StoryInfo{name, description, path});
        }

        Self {
            state: MenuState::Main,
            available_stories,
        }
    }

    async fn update(&mut self) -> Option<Phase> {
        clear_background(Color::new(1.0, 1.0, 1.0, 1.0));
        match &self.state{
            MenuState::Main => {
                let mut y_pos = 40.0;
        
                root_ui().label(vec2(10.0, y_pos), "Storyteller");

                y_pos += 20.0;

                if root_ui().button(vec2(10.0, y_pos), "Load Story") {
                    self.state = MenuState::StorySelector;
                }

                y_pos += 20.0;

                if root_ui().button(vec2(10.0, y_pos), "Options") {
                    self.state = MenuState::Options;
                }

                y_pos += 20.0;

                if root_ui().button(vec2(10.0, y_pos), "Quit") {
                    return Some(Phase::Quit);
                }
            },
            MenuState::Options => {
                if root_ui().button(vec2(10.0, 60.0), "Back") {
                    self.state = MenuState::Main;
                }
            },
            MenuState::StorySelector => {

                let mut y_pos = 20.0;

                root_ui().label(vec2(10.0, y_pos), "Stories");

                y_pos += 30.0;
                root_ui().canvas().line(vec2(10.0, y_pos), vec2(310.0, y_pos), BLACK);
                y_pos += 10.0;

                for story_info in &self.available_stories {
                    root_ui().label(vec2(10.0, y_pos), &story_info.name);
                    y_pos += 20.0;
                    root_ui().label(vec2(10.0, y_pos), &story_info.description);
                    y_pos += 20.0;
                    if root_ui().button(vec2(10.0, y_pos), "Play") {
                        return Some(Phase::Story(story_info.path.clone()));
                    }
                    y_pos += 30.0;
                    root_ui().canvas().line(vec2(10.0, y_pos), vec2(310.0, y_pos), BLACK);
                    y_pos += 10.0;
                }

                if root_ui().button(vec2(10.0, y_pos), "Back") {
                    self.state = MenuState::Main;
                }
            },
        }
        
        None
    }
}