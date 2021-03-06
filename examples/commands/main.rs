#![allow(dead_code)]

extern crate jam;
extern crate cgmath;

use jam::render::shader::ShaderPair;
use jam::render::texture::texture_array::TextureDirectory;
use jam::input::InputState;
use jam::render::shader::fatter;
use jam::render::shader::fatter::Command::*;
use jam::render::shader::fatter::{Seconds, Dimensions, Application, Command, Uniforms};
use jam::camera::Camera;
use jam::color;
use jam::color::Color;
use jam::render::quads::GeometryTesselator;
use jam::render::texture::TextureRegion;
use jam::Vec3;
use std::f64::consts::PI;

use cgmath::Rad;


fn main() {
    let app = App {
        name: "mixalot".into(),
        camera: Camera {
            at: Vec3::new(0.0, 0.0, 0.0),
            pitch: Rad(PI / 4.0_f64),
            viewport: (800, 600),
            pixels_per_unit: 16.0 * 1.0,
        },
        zoom: 1.0,
        pixels_per_unit: 16.0,
        n: 0,
    };
    
    let shader_pair = ShaderPair::for_paths("resources/shader/fat.vert", "resources/shader/fat.frag");
    
    let texture_dir = TextureDirectory::for_path("resources/textures");

    fatter::fat_example(app, shader_pair, texture_dir, (600, 600));
}

struct App {
    name : String,
    camera : Camera,
    zoom : f64,
    pixels_per_unit : f64,
    n : u64,
}

impl App {
    fn units_per_pixel(&self) -> f64 {
        1.0 / self.pixels_per_unit
    }

    fn tesselator(&self) -> GeometryTesselator {
        let upp = self.units_per_pixel();
        let tesselator_scale = Vec3::new(upp, upp, upp);
        GeometryTesselator::new(tesselator_scale)
    }

     fn raster(&self, color:Color, x:f64, z:f64) -> GeometryTesselator {
         let texture_region = TextureRegion {
            u_min: 0,
            u_max: 128,
            v_min: 0,
            v_max: 128,
            texture_size: 128,
        };
        
        let mut t = self.tesselator();
        t.color = color.float_raw();
        t.draw_floor_tile(&texture_region, 0, x, 0.0, z, 0.0, false);
        t
    }
}

impl Application for App {
    fn new(&mut self) {
        println!("new! => {:?}", self.name);
    }

    fn render(&mut self, input_state:&InputState, dimensions:Dimensions, delta_time: Seconds) -> Vec<Command> {
        self.n += 1;

        self.camera.at = Vec3::new(17.0, 0.0, 17.0);
        self.camera.pixels_per_unit = self.pixels_per_unit * self.zoom;
        self.camera.viewport = dimensions;
        
        let colors = vec![color::WHITE, color::BLUE, color::RED];
        
        
        let mut commands : Vec<fatter::Command> = Vec::new();
        
        let an = self.n / 60;

        let on_second = (self.n % 60) == 0;

        let raster_color = colors[(((an / 16) % 16) % 3) as usize]; // cycles every 16 seconds

        if on_second && an % 5 == 0 { // every fifth second
            let column = (an / 4) % 4;
            let name = format!("zone_{}", column);
            println!("delete {}", name);
            commands.push(Delete {prefix : name});
        }

        let n = (((an % 16) as f64) / 16.0 * 255.0) as u8;
        let render_color = color::rgb(n, n, n);

        for i in 0..16 {
            let xo = i % 4;
            let zo = i / 4;
            let name : String = format!("zone_{}_{}", xo, zo);

            if (an % 16) == i && on_second {
                let t = self.raster(raster_color, (xo * 9) as f64, (zo * 9) as f64);
                commands.push(DrawNew {
                    key: Some(name), 
                    vertices: t.tesselator.vertices, 
                    uniforms: Uniforms {
                        transform : fatter::down_size_m4(self.camera.view_projection().into()),
                        color: render_color,
                    }
                }); 
            } else if ((an+8) % 16) == i && on_second {
                let t = self.raster(color::GREEN, (xo * 9) as f64, (zo * 9) as f64);
                commands.push(Update {
                    key: name, 
                    vertices: t.tesselator.vertices,
                }); 
            } else {
                commands.push(Draw {
                    key: name,
                     uniforms: Uniforms {
                        transform : fatter::down_size_m4(self.camera.view_projection().into()),
                        color: render_color,
                    }
                });
            }
        }

        if input_state.close {
            commands.push(Close);
        }     

        commands
    }
}
