use std::fs;
use std::collections::HashMap;
use image;
use image::RgbaImage;
use std::time::Instant;

// FIXME: Textures have to be square or they exhibit corruption??
pub struct GameTexture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>
}

pub struct ResourcePool {
    pub textures: HashMap<String, GameTexture>
}

pub fn create_and_load () -> ResourcePool {
    println!("Loading resources...");
    let started_load = Instant::now();

    let mut pool = ResourcePool {
        textures: HashMap::new()
    };

    let paths = fs::read_dir("./resources/textures").unwrap();

    for path in paths {
        let pth = path.unwrap().path();
        let tex_name = pth.file_stem().unwrap().to_str().unwrap().to_owned();

        let ex = pth.extension();
        // Files like .DS_Store don't show up as having extensions
        if ex.is_none() { continue }

        let ext = ex.unwrap();

        if ext != "png" && ext != "jpg" && ext != "bmp" {
            println!("Texture with extension {:?} wasn't loaded because it wasn't PNG, JPG or BMP.", ext);
            continue;
        }

        println!("Loading \"{}\"...", tex_name);
        let tex = image::open(&pth).unwrap().to_rgba();

        let (w, h) = tex.dimensions();

        // Decompress and save into memory the colours
        // Speeds up render
        let mut pixels = vec![];
        for y in 0..h {
            for x in 0..w {
                let col = tex.get_pixel(x, y).0;
                pixels.push(col[0]);
                pixels.push(col[1]);
                pixels.push(col[2]);
                pixels.push(col[3]);
            }
        }

        pool.textures.insert(tex_name, GameTexture {
            width: w as usize,
            height: h as usize,
            pixels
        });
    }

    println!("Resource pool loaded in {}ms.", started_load.elapsed().as_millis());
    pool
}