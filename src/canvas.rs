use crate::vector::Vec3f;
use std::vec::Vec;
use std::io::BufWriter;
use std::fs::File;
use std::io::Write;


macro_rules! catch {
    ($a:expr) => {
        if let Err(err) = $a {
            eprintln!("Error: {}", err);
            return;
        }
    };
}

pub struct Canvas {
    framebuffer: Vec<Vec3f>,
    width: usize,
    height: usize
}

impl Canvas {
    pub fn new (width: usize, height: usize) -> Canvas {
        let mut framebuffer = Vec::with_capacity(width*height);
        // TODO: find a better way of doing this, probably by using a Nalgebra matrix
        // Safety: `new_len` here is less or equal than the vector capacity
        unsafe { framebuffer.set_len(width * height); } 

        Canvas {
            framebuffer,
            width,
            height
        }
    }

    pub fn set (&mut self, x: usize, y: usize, color: Vec3f) {
        self.framebuffer[x + y * self.width] = color;
    }

    pub fn save_to_image (self, filename: &str) {
        let f = File::create(filename);
        catch!(f);
        let mut f = BufWriter::new(f.unwrap());

        let res = write!(f, "P6\n{} {}\n{}\n", self.width, self.height, 255);
        catch!(res);

        let mut bytes = Vec::with_capacity(self.height * self.width * 3);
        
        for i in 0.. self.height * self.width {
            // TODO: Cap the current position to at max 1
            for j in 0 .. 3_usize {
                let byte = 255_f64 * f64::max(0.0, f64::min(1.0, self.framebuffer[i][j]));
                let byte = byte as u8;
                bytes.push(byte);
            }
        }
        if let Err(err)  = f.write_all(&bytes) {
            println!("Error: {}", err);
        }
        let res = f.flush();
        catch!(res);
    }
} 