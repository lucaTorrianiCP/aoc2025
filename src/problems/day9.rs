use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i128,
    y: i128,
}

pub fn part1() -> i128 {
    let file = File::open("./inputs/09").unwrap();
    let reader = BufReader::new(file);
    let mut somma = 0;

    let points: Vec<Point> = reader
        .lines()
        .map(|x| {
            let tmp = x.unwrap();

            let mut iterator = tmp.split(',');

            Point {
                x: iterator
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<i128>()
                    .unwrap(),
                y: iterator
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<i128>()
                    .unwrap(),
            }
        })
        .collect();

    for fp in &points {
        for sp in &points {
            if fp == sp {
                continue;
            }
            let area = (i128::abs(fp.x - sp.x) + 1) * (i128::abs(fp.y - sp.y) + 1);

            if area > somma {
                somma = area;
            }
        }
    }

    somma
}

pub fn part2() -> i128 {
    let file = File::open("./inputs/09").unwrap();
    let reader = BufReader::new(file);
    let mut somma = 0;

    let points: Vec<Point> = reader
        .lines()
        .map(|x| {
            let tmp = x.unwrap();

            let mut iterator = tmp.split(',');

            Point {
                x: iterator
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<i128>()
                    .unwrap(),
                y: iterator
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<i128>()
                    .unwrap(),
            }
        })
        .collect();

    for i in 0..points.len() {
        for j in 0..points.len() {
            if points[i] == points[j] {
                continue;
            }

            if !rectangle_in_polygon(
                [
                    points[j],
                    points[i],
                    Point {
                        x: points[j].x,
                        y: points[i].y,
                    },
                    Point {
                        x: points[i].x,
                        y: points[j].y,
                    },
                ],
                &points,
            ) {
                continue;
            }

            let area = (i128::abs(points[i].x - points[j].x) + 1)
                * (i128::abs(points[i].y - points[j].y) + 1);

            if area > somma {
                somma = area;
            }
        }
    }

    somma
}

fn orientation(a: Point, b: Point, c: Point) -> i128 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);

    (o1 * o2 < 0) && (o3 * o4 < 0)
}

fn point_in_polygon_inclusive(p: Point, poly: &[Point]) -> bool {
    let n = poly.len();

    for &v in poly {
        if (p.x as f64 - v.x as f64).abs() < f64::EPSILON
            && (p.y as f64 - v.y as f64).abs() < f64::EPSILON
        {
            return true;
        }
    }

    let mut inside = false;
    let mut j = n - 1;

    for i in 0..n {
        let pi = poly[i];
        let pj = poly[j];

        let min_x = pi.x.min(pj.x);
        let max_x = pi.x.max(pj.x);
        let min_y = pi.y.min(pj.y);
        let max_y = pi.y.max(pj.y);

        if (pj.x as f64 - pi.x as f64).abs() > f64::EPSILON {
            let slope = (pj.y - pi.y) / (pj.x - pi.x);
            let y_on_line = pi.y + slope * (p.x - pi.x);
            if (y_on_line as f64 - p.y as f64).abs() < f64::EPSILON && p.x >= min_x && p.x <= max_x
            {
                return true;
            }
        } else {
            if (p.x as f64 - pi.x as f64).abs() < f64::EPSILON && p.y >= min_y && p.y <= max_y {
                return true;
            }
        }

        let intersect = ((pi.y > p.y) != (pj.y > p.y))
            && (p.x < (pj.x - pi.x) * (p.y - pi.y) / (pj.y - pi.y) + pi.x);

        if intersect {
            inside = !inside;
        }

        j = i;
    }

    inside
}

fn rectangle_in_polygon(rect_vertices: [Point; 4], poly: &[Point]) -> bool {
    for &v in &rect_vertices {
        if !point_in_polygon_inclusive(v, poly) {
            return false;
        }
    }

    let rect_edges = [
        (rect_vertices[0], rect_vertices[1]),
        (rect_vertices[1], rect_vertices[2]),
        (rect_vertices[2], rect_vertices[3]),
        (rect_vertices[3], rect_vertices[0]),
    ];

    let poly_edges: Vec<(Point, Point)> = poly
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, poly[(i + 1) % poly.len()]))
        .collect();

    for &(a, b) in &rect_edges {
        for &(c, d) in &poly_edges {
            if segments_intersect(a, b, c, d) {
                return false;
            }
        }
    }

    true
}
