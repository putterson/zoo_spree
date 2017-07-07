extern crate toml;
extern crate serde;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub video: VideoConfig,
    pub input: InputConfig,
}

#[derive(Debug, Deserialize)]
pub struct VideoConfig {
    pub fullscreen: bool,
    resolution : String,
    scale: u32,
    auto_x_resolution: Option<u32>,
    auto_y_resolution: Option<u32>,
}

impl VideoConfig {
     pub fn x_resolution(&self) -> u32 {
         match self.auto_x_resolution {
             Some(x_res) if self.auto_resolution() => x_res / self.scale,
             None | _ => 800,
         }
     }

     pub fn y_resolution(&self) -> u32 {
         match self.auto_y_resolution {
             Some(y_res) if self.auto_resolution() => y_res / self.scale,
             None | _ => 600,
         }
     }

     pub fn auto_resolution(&self) -> bool {
         return "auto" == self.resolution;
     }

     pub fn set_auto_resolution(&mut self, w : u32, h : u32 ){
        self.auto_x_resolution = Some(w);
        self.auto_y_resolution = Some(h);
     }
}

#[derive(Debug, Deserialize)]
pub struct InputConfig {
    pub deadzone: u16,
    pub keyboard: bool,
}

const SETTINGS_FILE : &str = "settings.toml";

pub fn load() -> Result<Config, toml::de::Error> {
    
    let mut input = String::new();
    
    File::open(SETTINGS_FILE.to_string()).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    let decoded = toml::from_str(&input);

    return decoded
}