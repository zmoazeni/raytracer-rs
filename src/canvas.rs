use super::*;
use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, Write};
use std::ops::{Index,IndexMut};
use iterator::*;

const MAX_PPM_LINE_LENGTH: usize = 70;

pub struct Canvas {
    pub dimensions: (usize, usize),
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let row = vec![Color::black(); width];
        let mut v2 = Vec::with_capacity(height);
        for _ in 0..height {
            v2.push(row.clone())
        }

        Canvas { dimensions: (width, height), pixels: v2 }
    }

    pub fn set(&mut self, (x, y): (usize, usize), color: Color) {
        let (width, height) = self.dimensions;
        // Ignore any pixels outside the canvas
        if x < width && y < height {
            self[(x, y)] = color
        }
    }

    pub fn iter(&self) -> DimensionalIterator {
        DimensionalIterator::canvas(self.dimensions)
    }

    pub fn write_ppm<W: Write>(&self, write: &mut W) -> Result<(), std::io::Error> {
        write.write_all(self.ppm_header().as_bytes())?;
        self.write_ppm_pixels(write)?;
        Ok(())
    }

    pub fn save_as_ppm(&self, path: &Path) -> Result<(), std::io::Error> {
        let f = File::create(path).expect("Unable to open file");
        let mut f = BufWriter::new(f);
        self.write_ppm(&mut f)
    }

    pub fn ppm_header(&self) -> String {
        let (width, height) = self.dimensions;
        format!("P3\n\
            {} {}\n\
            256\n", width, height)
    }

    pub fn write_ppm_pixels<W: Write>(&self, write: &mut W) -> Result<(), std::io::Error> {
        for row in &self.pixels {
            let mut current_line = String::new();

            for color in row {
                let parts = color.ppm_parts();
                for part in parts {
                    let part_with_space = if !current_line.is_empty() {
                        format!(" {}", part)
                    } else {
                        part.clone()
                    };

                    if part_with_space.len() + current_line.len() < MAX_PPM_LINE_LENGTH {
                        current_line.push_str(&part_with_space);
                    } else {
                        write.write_all((format!("{}\n", current_line)).as_bytes())?;
                        current_line = part.clone();
                    }
                }
            }

            if !current_line.is_empty() {
                write.write_all((format!("{}\n", current_line)).as_bytes())?;
            }
        }
        write.write_all(("\n").as_bytes())?;
        Ok(())
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;
    fn index(&self, (x, y): (usize, usize)) -> &Color {
        &self.pixels[y][x]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Color {
        &mut self.pixels[y][x]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_canvas() {
        let c = Canvas::new(3, 5);
        assert_eq!(c.dimensions, (3, 5));

        let mut count = 0;
        for (x, y) in c.iter() {
            count += 1;
            let color = &c[(x, y)];
            assert_eq!(color, &Color::black());
        }
        assert_eq!(count, 15);
    }

    #[test]
    fn writing_pixels() {
        let mut c = Canvas::new(40, 20);
        c[(2, 10)] = Color::white();
        assert_eq!(c[(0, 0)], Color::black());
        assert_eq!(c[(2, 10)], Color::white());
    }

    #[test]
    fn save_ppm_to_file() {
        let mut c = Canvas::new(5, 3);
        c[(0, 0)] = Color::new(1.5, 0.0, 0.0);
        c[(2, 1)] = Color::new(0.0, 0.5, 0.0);
        c[(4, 2)] = Color::new(-0.5, 0.0, 1.0);

        let lines = ppm_lines(&c);
        assert_eq!(lines.len(), 7);
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "256");
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        assert_eq!(lines[6], "");
    }

    #[test]
    fn saving_ppm_splits_long_lines() {
        let mut c = Canvas::new(10, 2);
        for (x, y) in c.iter() {
            c[(x, y)] = Color::new(1.0, 0.8, 0.6);
        }
        let lines = ppm_lines(&c);
        assert_eq!(lines.len(), 8);
        assert_eq!(lines[3], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
        assert_eq!(lines[4], "153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert_eq!(lines[5], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
        assert_eq!(lines[6], "153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert_eq!(lines[7], "");
    }

    fn ppm_lines(canvas: &Canvas) -> Vec<String> {
        let mut v = Vec::new();
        let result = canvas.write_ppm(&mut v);
        assert!(result.is_ok());
        let s = String::from_utf8(v).unwrap();
        let lines: Vec<&str> = s.lines().collect();
        let mut return_vec = Vec::with_capacity(lines.len());
        for line in lines {
            return_vec.push(String::from(line));
        }
        return_vec
    }

    #[test]
    fn pixels_outside_canvas() {
        let mut c = Canvas::new(2, 2);
        c.set((5, 5), Color::white());
        for (x, y) in c.iter() {
            let color = &c[(x, y)];
            assert_eq!(color, &Color::black());
        }
    }
}
