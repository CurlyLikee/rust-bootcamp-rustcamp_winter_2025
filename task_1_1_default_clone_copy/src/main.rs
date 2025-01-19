use task_1_1::Polyline;
use task_1_1::Point;


fn main() {
    let mut polyline = Polyline::new();
    polyline.add_point(Point { x: 1.0, y: 2.0 });
    polyline.add_point(Point { x: 3.0, y: 4.0 });
    polyline.print_points();
    polyline.remove_point(0);
    polyline.print_points();
}