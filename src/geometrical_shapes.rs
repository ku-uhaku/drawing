use raster::Color;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut dyn Displayable);
    fn color(&self) -> Color;
}

fn random_color() -> Color {
    Color::rgb(
        rand::random_range(0..255),
        rand::random_range(0..255),
        rand::random_range(0..255),
    )
}

#[derive(Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
    color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            color: Color::white(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut r = Self::new(rand::random_range(0..width), rand::random_range(0..height));
        r.color = random_color();
        r
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut dyn Displayable) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Line {
    p1: Point,
    p2: Point,
    color: Color,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            color: Color::white(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut r = Self::new(&Point::random(width, height), &Point::random(width, height));
        r.color = random_color();
        r
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut dyn Displayable) {
        let dx = i64::from(self.p2.x) - i64::from(self.p1.x);
        let dy = i64::from(self.p2.y) - i64::from(self.p1.y);

        let steps = dx.abs().max(dy.abs());

        if steps == 0 {
            image.display(self.p1.x, self.p1.y, self.color());
            return;
        }

        for i in 0..=steps {
            let x = i64::from(self.p1.x) + dx * i / steps;
            let y = i64::from(self.p1.y) + dy * i / steps;

            image.display(x as i32, y as i32, self.color());
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Circle {
    c: Point,
    r: i32,
    color: Color,
}

impl Circle {
    pub fn new(c: &Point, r: i32) -> Self {
        Self {
            c: c.clone(),
            r: r.max(0),
            color: Color::white(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut c = Self::new(&Point::random(width, height), rand::random_range(0..255));
        c.color = random_color();
        c
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut dyn Displayable) {
        let mut d = 1 - self.r;
        let mut x = 0;
        let mut y = self.r;
        while x <= y {
            image.display(self.c.x + x, self.c.y + y, self.color());
            image.display(self.c.x - x, self.c.y + y, self.color());
            image.display(self.c.x + x, self.c.y - y, self.color());
            image.display(self.c.x - x, self.c.y - y, self.color());
            image.display(self.c.x + y, self.c.y + x, self.color());
            image.display(self.c.x - y, self.c.y + x, self.color());
            image.display(self.c.x + y, self.c.y - x, self.color());
            image.display(self.c.x - y, self.c.y - x, self.color());
            if d < 0 {
                x += 1;
                d = d + (2 * x) + 1
            } else {
                x += 1;
                y -= 1;
                d = d + (2 * (x - y)) + 1
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    color: Color,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone(),
            color: Color::white(),
        }
    }

    // pub fn random(width: i32, height: i32) -> Self {
    //     Self {
    //         p1: Point::random(width, height),
    //         p2: Point::random(width, height),
    //         p3: Point::random(width, height),
    //         color: random_color(),
    //     }
    // }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let c = self.color();
        Line {
            p1: self.p1.clone(),
            p2: self.p2.clone(),
            color: c.clone(),
        }
        .draw(image);
        Line {
            p1: self.p2.clone(),
            p2: self.p3.clone(),
            color: c.clone(),
        }
        .draw(image);
        Line {
            p1: self.p3.clone(),
            p2: self.p1.clone(),
            color: c,
        }
        .draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            color: Color::white(),
        }
    }

    // pub fn random(width: i32, height: i32) -> Self {
    //     Self {
    //         p1: Point::random(width, height),
    //         p2: Point::random(width, height),
    //         color: random_color(),
    //     }
    // }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let a = self.p1.clone();
        let b = Point::new(self.p2.x, self.p1.y);
        let c = self.p2.clone();
        let d = Point::new(self.p1.x, self.p2.y);
        let col = self.color();
        Line {
            p1: a.clone(),
            p2: b.clone(),
            color: col.clone(),
        }
        .draw(image);
        Line {
            p1: b.clone(),
            p2: c.clone(),
            color: col.clone(),
        }
        .draw(image);

        Line {
            p1: c.clone(),
            p2: d.clone(),
            color: col.clone(),
        }
        .draw(image);
        Line {
            p1: d,
            p2: a,
            color: col,
        }
        .draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Canvas(Vec<(i32, i32)>);

    impl Displayable for Canvas {
        fn display(&mut self, x: i32, y: i32, _color: Color) {
            self.0.push((x, y));
        }
    }

    #[test]
    fn random_points_and_lines_stay_inside_image() {
        for (width, height) in [(1, 1), (1, 12), (12, 1), (100, 50)] {
            for _ in 0..100 {
                let point = Point::random(width, height);
                let line = Line::random(width, height);
                for p in [&point, &line.p1, &line.p2] {
                    assert!((0..width).contains(&p.x));
                    assert!((0..height).contains(&p.y));
                }
            }
        }
    }
    #[test]
    fn random_circle_center_inside_image() {
        for (width, height) in [(1, 1), (1, 12), (12, 1), (100, 50)] {
            for _ in 0..100 {
                let circle = Circle::random(width, height);
                assert!((0..width).contains(&circle.c.x));
                assert!((0..height).contains(&circle.c.y));
            }
        }
    }

    #[test]
    fn random_center_inside_image() {
        for (width, height) in [(1, 1), (1, 12), (12, 1), (100, 50)] {
            for _ in 0..100 {
                let center = Point::random(width, height);
                assert!((0..width).contains(&center.x));
                assert!((0..height).contains(&center.y));
            }
        }
    }

    #[test]
    fn rectangle_handles_reversed_corners() {
        let mut expected = Vec::new();
        for x in 2..=5 {
            expected.extend([(x, 3), (x, 7)]);
        }
        for y in 4..7 {
            expected.extend([(2, y), (5, y)]);
        }
        expected.sort_unstable();
        for ((x1, y1), (x2, y2)) in [
            ((2, 3), (5, 7)),
            ((5, 7), (2, 3)),
            ((2, 7), (5, 3)),
            ((5, 3), (2, 7)),
        ] {
            let mut canvas = Canvas::default();
            Rectangle::new(&Point::new(x1, y1), &Point::new(x2, y2)).draw(&mut canvas);
            canvas.0.sort_unstable();
            canvas.0.dedup();
            assert_eq!(canvas.0, expected);
        }
    }

    #[test]
    fn identical_line_endpoints_draw_one_pixel() {
        let line = Line::new(&Point::new(1, 1), &Point::new(1, 1));
        let mut canvas = Canvas::default();
        line.draw(&mut canvas);
        let line = Line::new(&Point::new(1, 1), &Point::new(1, 1));
        let mut canvas = Canvas::default();
        line.draw(&mut canvas);
        assert_eq!(canvas.0, vec![(1, 1)]);
        assert_eq!(canvas.0, vec![(1, 1)]);
    }
}
