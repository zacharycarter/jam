#![crate_name="jam"]
#![allow(dead_code)]

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate time;

extern crate cgmath;
extern crate image;
extern crate fnv;

extern crate notify;

pub mod render {
    pub mod quads;
    pub mod shader;
    pub mod texture;
}
pub mod camera;
pub mod input;
pub mod geometry;
pub mod game {
    pub mod simple;
}
pub mod spring;
pub mod audio;
pub mod color;

use fnv::FnvHasher;
use std::collections::{HashMap as StdHashMap, HashSet as StdHashSet};
use std::hash::BuildHasherDefault;

pub type HashMap<K, V> = StdHashMap<K, V, BuildHasherDefault<FnvHasher>>;
pub type HashSet<V> = StdHashSet<V, BuildHasherDefault<FnvHasher>>;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub fn clamp<T : PartialOrd>(n:T, min:T, max:T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

pub fn vec3(x:f64, y:f64, z:f64) -> Vec3 {
    Vec3::new(x, y, z)
}


pub fn vec2i(x:i32, y:i32) -> Vec2i {
    Vec2i::new(x, y)
}

pub fn lerp(a:Vec3, b:Vec3, alpha:f64) -> Vec3 {
    b * alpha + a * (1.0 - alpha)
}

pub type Vec2 = cgmath::Vector2<f64>;
pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec2Size = cgmath::Vector2<usize>;
pub type Vec3i = cgmath::Vector3<i32>;

pub type Vec3 = cgmath::Vector3<f64>;
pub type Vec3f = cgmath::Vector3<f32>;

pub type Vec4 = cgmath::Vector4<f64>;

pub type Mat3 = cgmath::Matrix3<f64>;
pub type Mat4 = cgmath::Matrix4<f64>;

pub fn round_down(f:f64) -> i32 {
    if f < 0.0 {
        f as i32 - 1
    } else {
        f as i32
    }
}

pub fn round_down_v3(v:Vec3) -> Vec3i {
    Vec3i::new(round_down(v.x), round_down(v.y), round_down(v.z))
}