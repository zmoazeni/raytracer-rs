use super::*;
use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, Write};

const MAX_PPM_LINE_LENGTH: usize = 70;

pub struct Canvas {
    dimensions: (usize, usize),
    pixels: Vec<Vec<Color>>
}

pub struct CanvasIterator {
    dimensions: (usize, usize),
    position: (usize, usize),
    first_pass: bool,
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

    pub fn write_pixel(&mut self, (x, y): (usize, usize), color: Color) {
        let (width, height) = self.dimensions;
        // Ignore any pixels outside the canvas
        if x < width && y < height {
            self.pixels[y][x] = color
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y][x]
    }

    pub fn iter(&self) -> CanvasIterator {
        CanvasIterator { dimensions: self.dimensions, position: (0, 0), first_pass: true }
    }

    pub fn write_ppm<W: Write>(&self, write: &mut W) -> Result<(), std::io::Error> {
        write.write_all(self.ppm_header().as_bytes())?;
        write.write_all(self.ppm_pixels().as_bytes())?;
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

    pub fn ppm_pixels(&self) -> String {
        let mut output = String::new();
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
                        output.push_str(&format!("{}\n", current_line));
                        current_line = part.clone();
                    }
                }
            }

            if !current_line.is_empty() {
                output.push_str(&format!("{}\n", current_line));
            }
        }
        output.push_str("\n");
        output
    }
}

impl Iterator for CanvasIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let (width, height) = self.dimensions;
        let (x, y) = self.position;

        if self.first_pass && x < width {
            self.first_pass = false;
            return Some(self.position);
        }

        if x + 1 < width {
            self.position = (x + 1, y);
        } else if y + 1 < height {
            self.position = (0, y + 1);
        } else {
            return None;
        }

        Some(self.position)
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
            let color = c.pixel_at(x, y);
            assert_eq!(color, &Color::black());
        }
        assert_eq!(count, 15);
    }

    #[test]
    fn writing_pixels() {
        let mut c = Canvas::new(40, 20);
        c.write_pixel((2, 10), Color::white());
        assert_eq!(c.pixel_at(0, 0), &Color::black());
        assert_eq!(c.pixel_at(2, 10), &Color::white());
    }

    #[test]
    fn save_ppm_to_file() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel((0, 0), Color::new(1.5, 0.0, 0.0));
        c.write_pixel((2, 1), Color::new(0.0, 0.5, 0.0));
        c.write_pixel((4, 2), Color::new(-0.5, 0.0, 1.0));

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
            c.write_pixel((x, y), Color::new(1.0, 0.8, 0.6));
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
        c.write_pixel((5, 5), Color::white());
        for (x, y) in c.iter() {
            let color = c.pixel_at(x, y);
            assert_eq!(color, &Color::black());
        }
    }
}
