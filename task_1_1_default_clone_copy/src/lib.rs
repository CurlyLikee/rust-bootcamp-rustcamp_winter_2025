#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
pub struct Polyline {
    points: Vec<Point>,
}


impl Default for Point {
    fn default() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}



impl Polyline {
    pub fn new() -> Self {
        Polyline { points: Vec::new() }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn remove_point(&mut self, index: usize) {
        if index < self.points.len() {
            self.points.remove(index);
        } else {
            println!(" індекс {} поза діапазоном ", index);
        }
    }

    pub fn get_point(&self, index: usize) -> Option<&Point> {
        self.points.get(index)
    }


    pub fn print_points(&self) {
        for (i, point) in self.points.iter().enumerate() {
            println!("Point {}: ({}, {})", i, point.x, point.y);
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut polyline = Polyline::new();
        polyline.add_point(Point { x: 3.0, y: 4.0 });
        polyline.add_point(Point { x: 3.0, y: 4.0 });
        assert_eq!(polyline.get_point(1), Some(&Point { x: 3.0, y: 4.0 }));
        polyline.remove_point(0);
        assert_eq!(polyline.get_point(0), Some(&Point { x: 3.0, y: 4.0 }));
    }
}