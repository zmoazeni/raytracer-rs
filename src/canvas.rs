use super::*;

pub struct Canvas {
    dimensions: (usize, usize),
    pixels: Vec<Vec<Color>>
}

pub struct CanvasIterator<'a> {
    canvas: &'a Canvas,
    position: (usize, usize),
    first_pass: bool,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let row = vec![Color::white(); width];
        let mut v2 = Vec::with_capacity(height);
        for _ in 0..height {
            v2.push(row.clone())
        }

        Canvas { dimensions: (width, height), pixels: v2 }
    }

    pub fn write_pixel(&mut self, (x, y): (usize, usize), color: Color) {
        self.pixels[y][x] = color
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y][x]
    }

    pub fn pixel_att(&self, (x, y): (usize, usize)) -> &Color {
        self.pixel_at(x, y)
    }

    pub fn iter(&self) -> CanvasIterator {
        CanvasIterator { canvas: self, position: (0, 0), first_pass: true }
    }
}

impl<'a> Iterator for CanvasIterator<'a> {
    type Item = ((usize, usize), &'a Color);

    fn next(&mut self) -> Option<((usize, usize), &'a Color)> {
        let (width, height) = self.canvas.dimensions;
        let (x, y) = self.position;

        if self.first_pass && x < width {
            let color = self.canvas.pixel_att(self.position);
            self.first_pass = false;
            return Some((self.position, &color));
        }

        if x + 1 < width {
            self.position = (x + 1, y);
        } else if y + 1 < height {
            self.position = (0, y + 1);
        } else {
            return None;
        }

        let color = self.canvas.pixel_att(self.position);
        Some((self.position, &color))
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
        for (_, color) in c.iter() {
            count += 1;
            assert_eq!(color, &Color::white());
        }
        assert_eq!(count, 15);
    }

    #[test]
    fn writing_pixels() {
        let mut c = Canvas::new(40, 20);
        c.write_pixel((2, 10), Color::black());
        assert_eq!(c.pixel_at(0, 0), &Color::white());
        assert_eq!(c.pixel_at(2, 10), &Color::black());
    }
}
