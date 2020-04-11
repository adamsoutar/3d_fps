use std::fs;
use std::collections::HashMap;
use image;
use image::RgbaImage;

pub struct ResourcePool {
    pub textures: HashMap<String, RgbaImage>
}

pub fn create_and_load () -> ResourcePool {
    println!("Loading resources...");

    let mut pool = ResourcePool {
        textures: HashMap::new()
    };

    let paths = fs::read_dir("./resources/textures").unwrap();

    for path in paths {
        let pth = path.unwrap().path();
        let tex_name = pth.file_stem().unwrap().to_str().unwrap().to_owned();
        println!("Loading \"{}\"...", tex_name);
        let tex = image::open(&pth).unwrap().to_rgba();

        pool.textures.insert(tex_name, tex);
    }

    println!("Resource pool loaded.");
    pool
}