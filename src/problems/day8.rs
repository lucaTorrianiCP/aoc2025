use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

pub fn part1() -> i64 {
    let file = File::open("./inputs/08").unwrap();
    let reader = BufReader::new(file);
    let mut somma = 1;

    let points: Vec<Point> = reader
        .lines()
        .map(|x| {
            let tmp = x.unwrap();

            let mut iterator = tmp.split(',');

            Point {
                x: iterator.next().unwrap().to_string().parse::<i64>().unwrap(),
                y: iterator.next().unwrap().to_string().parse::<i64>().unwrap(),
                z: iterator.next().unwrap().to_string().parse::<i64>().unwrap(),
            }
        })
        .collect();

    let groups = k_min_pairs(&points, 1000);

    let mut comps = connected_components(&groups);

    comps.sort_by_key(|x| Reverse(x.len()));

    for i in 0..3 {
        somma *= comps[i].len() as i64;
    }

    somma
}

pub fn part2() -> i64 {
    let file = File::open("./inputs/08").unwrap();
    let reader = BufReader::new(file);

    let points: Vec<Point> = reader
        .lines()
        .map(|x| {
            let tmp = x.unwrap();

            let mut iterator = tmp.split(',');

            Point {
                x: iterator.next().unwrap().to_string().parse::<i64>().unwrap(),
                y: iterator.next().unwrap().to_string().parse::<i64>().unwrap(),
                z: iterator.next().unwrap().to_string().parse::<i64>().unwrap(),
            }
        })
        .collect();

    let groups = build_mst_points(&points);

    groups[groups.len() - 1].0.x * groups[groups.len() - 1].1.x
}

fn connected_components(edges: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
    let mut graph: HashMap<Point, HashSet<Point>> = HashMap::new();

    for pair in edges {
        let a = pair[0];
        let b = pair[1];

        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for &node in graph.keys() {
        if visited.contains(&node) {
            continue;
        }

        let mut component = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(node);
        visited.insert(node);

        while let Some(n) = queue.pop_front() {
            component.push(n);

            if let Some(neighbors) = graph.get(&n) {
                for &next in neighbors {
                    if !visited.contains(&next) {
                        visited.insert(next);
                        queue.push_back(next);
                    }
                }
            }
        }

        components.push(component);
    }

    components
}

fn distance_sq(a: &Point, b: &Point) -> i128 {
    let dx = a.x as i128 - b.x as i128;
    let dy = a.y as i128 - b.y as i128;
    let dz = a.z as i128 - b.z as i128;
    dx * dx + dy * dy + dz * dz
}

fn k_min_pairs(points: &Vec<Point>, k: usize) -> Vec<Vec<Point>> {
    let mut result = Vec::new();
    let mut used_pairs: HashSet<(usize, usize)> = HashSet::new();

    for _ in 0..k {
        let mut best: Option<(usize, usize, i128)> = None;

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                if used_pairs.contains(&(i, j)) {
                    continue;
                }

                let d = distance_sq(&points[i], &points[j]);

                match best {
                    None => best = Some((i, j, d)),
                    Some((_, _, best_d)) if d < best_d => best = Some((i, j, d)),
                    _ => {}
                }
            }
        }

        if let Some((i, j, _)) = best {
            used_pairs.insert((i, j));
            result.push(vec![points[i].clone(), points[j].clone()]);
        } else {
            break;
        }
    }

    result
}

//Mi sono dovuto far dare una mano
struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);

        if pa == pb {
            return false;
        }

        self.parent[pa] = pb;
        true
    }
}

fn distance(a: Point, b: Point) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn build_mst_points(points: &Vec<Point>) -> Vec<(Point, Point)> {
    let n = points.len();

    let mut edges = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            edges.push((distance(points[i], points[j]), i, j));
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut dsu = DSU::new(n);

    let mut mst = Vec::new();

    for &(_, i, j) in &edges {
        if dsu.union(i, j) {
            mst.push((points[i], points[j]));
        }
    }

    mst
}
