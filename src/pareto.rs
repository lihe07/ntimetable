use std::cmp::Ordering;

use rand::seq::SliceRandom;

pub trait CanDominate {
    fn dominates(&self, other: &Self) -> bool;
    fn compare_first_element(&self, other: &Self) -> Ordering;
    fn avg(&self) -> f32;
}

// fn dominates<T: Ord>(this: T, other: T) -> bool {
//     let mut any_better = false;
//
//     for (i, &self_obj) in this.1.iter().enumerate() {
//         if self_obj > other.1[i] {
//             any_better = true;
//         } else if self_obj < other.1[i] {
//             return false;
//         }
//     }
//
//     any_better
// }

pub fn naive<T: CanDominate>(data: Vec<T>) -> Vec<T> {
    let mut pareto_set: Vec<T> = Vec::new();

    for sol in data {
        let mut flag = true;

        let mut new_pareto_set = vec![];
        for p in pareto_set {
            if p.dominates(&sol) {
                flag = false;
            }
            if !sol.dominates(&p) {
                new_pareto_set.push(p);
            }
        }

        pareto_set = new_pareto_set;
        if flag {
            pareto_set.push(sol);
        }
    }

    pareto_set
}

/// Kung Algorithm
pub fn kung_recursive<T: CanDominate>(mut data: Vec<T>) -> Vec<T> {
    if data.len() == 1 {
        return data;
    }

    // Sort the data according to the first dimension
    // data.sort_by(|a, b| a.1[0].partial_cmp(&b.1[0]).unwrap());
    data.sort_by(|a, b| a.compare_first_element(&b));

    // Split the data into two parts
    // Avoid clone
    let right = data.split_off(data.len() / 2);
    let left = data;

    // Recursively find the skyline of each part
    let left_skyline = kung_recursive(left);
    let right_skyline = kung_recursive(right);

    // Merge the two skylines
    let mut skyline = Vec::new();

    for l in left_skyline {
        let mut flag = true;

        for r in &right_skyline {
            if r.dominates(&l) {
                flag = false;
                break;
            }
        }
        if flag {
            skyline.push(l);
        }
    }

    for r in right_skyline {
        skyline.push(r);
    }

    skyline
}

pub fn kung_recursive_mosa<T: CanDominate>(mut data: Vec<T>, temp: f32) -> Vec<T> {
    if data.len() == 1 {
        return data;
    }

    // Sort the data according to the first dimension
    // data.sort_by(|a, b| a.1[0].partial_cmp(&b.1[0]).unwrap());
    data.sort_by(|a, b| a.compare_first_element(&b));

    // Split the data into two parts
    // Avoid clone
    let right = data.split_off(data.len() / 2);
    let left = data;

    // Recursively find the skyline of each part
    let left_skyline = kung_recursive_mosa(left, temp);
    let right_skyline = kung_recursive_mosa(right, temp);

    // Merge the two skylines
    let mut skyline = Vec::new();

    for l in left_skyline {
        let mut energy = 0.0;
        let avg = l.avg();

        for r in &right_skyline {
            if r.dominates(&l) {
                energy += r.avg() - avg;
            }
        }
        let rand: f32 = rand::random();

        if energy == 0.0 {
            skyline.push(l);
            continue;
        }

        let prob = 1.0 / (1.0 + (energy / temp).exp());

        if rand <= prob {
            skyline.push(l);
        }
    }

    for r in right_skyline {
        skyline.push(r);
    }

    skyline
}

pub fn random_frontline<T: CanDominate + PartialEq>(mut solutions: Vec<T>, max: usize) -> Vec<T> {
    // First Dedup
    solutions.dedup();

    solutions = kung_recursive(solutions);

    let mut rng = rand::thread_rng();
    solutions.shuffle(&mut rng);

    solutions.truncate(max);

    solutions
}

mod test {
    use super::*;
    use rand::Rng;

    #[derive(Debug, PartialEq)]
    struct Point {
        a: i8,
        b: i8,
        c: i8,
    }

    impl CanDominate for Point {
        fn dominates(&self, other: &Self) -> bool {
            self.a > other.a && self.b > other.b && self.c > other.c
        }

        fn compare_first_element(&self, other: &Self) -> Ordering {
            self.a.partial_cmp(&other.a).unwrap()
        }

        fn avg(&self) -> f32 {
            0.0
        }
    }

    fn to_strings(data: Vec<Point>) -> Vec<String> {
        let mut strings = vec![];
        for d in data {
            strings.push(format!("{},{},{}", d.a, d.b, d.c));
        }
        strings
    }

    #[test]
    fn test_kung_recursive() {
        let data = vec![
            Point { a: 1, b: 1, c: 1 },
            Point { a: 2, b: 2, c: 2 },
            Point { a: 1, b: 3, c: 1 },
            Point { a: 1, b: 3, c: 1 },
        ];

        let data = kung_recursive(data);
        let strings = to_strings(data);

        assert!(strings.iter().filter(|&s| s == "1,1,1").count() == 0);
        assert!(strings.iter().filter(|&s| s == "2,2,2").count() == 1);
        assert!(strings.iter().filter(|&s| s == "1,3,1").count() == 2);
    }

    #[test]
    fn test_random_frontline() {
        let data = vec![
            Point { a: 1, b: 1, c: 1 },
            Point { a: 2, b: 2, c: 2 },
            Point { a: 1, b: 3, c: 1 },
            Point { a: 1, b: 3, c: 1 },
        ];

        let data = random_frontline(data, 3);
        let strings = to_strings(data);

        assert!(strings.len() == 2);
        assert!(strings.iter().filter(|&s| s == "1,1,1").count() == 0);
        assert!(strings.iter().filter(|&s| s == "2,2,2").count() == 1);
        assert!(strings.iter().filter(|&s| s == "1,3,1").count() == 1); // Dedup
    }

    #[derive(Clone)]
    struct MOSA(Vec<f32>);

    impl CanDominate for MOSA {
        fn dominates(&self, other: &Self) -> bool {
            for (i, v) in other.0.iter().enumerate() {
                if *v > self.0[i] {
                    return false;
                }
            }
            true
        }

        fn compare_first_element(&self, other: &Self) -> Ordering {
            self.0[0].partial_cmp(&other.0[0]).unwrap()
        }

        fn avg(&self) -> f32 {
            self.0.iter().sum::<f32>() / self.0.len() as f32
        }
    }
}
