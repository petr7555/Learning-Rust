use std::cmp::{max, min};
use std::io;

pub fn get_side_from_user(name: &str) -> u32 {
    loop {
        println!("Please enter side '{}'", name);
        let mut side = String::new();

        io::stdin().read_line(&mut side).expect("Stdin not working");
        let side = match side.trim().parse() {
            Ok(0) => {
                println!("Side of a rectangle cannot be zero");
                continue;
            }
            Ok(val) => val,
            Err(_) => continue
        };
        println!("Side '{}' is: {}", name, side);

        break side;
    }
}

pub fn verify_triangle(a: u32, b: u32, c: u32) {
    if a + b <= c || a + c <= b || b + c <= a {
        panic!("This is not a valid triangle!");
    }
}

fn get_semiperimeter(a: u32, b: u32, c: u32) -> f32 {
    (a + b + c) as f32 / 2.0
}

pub fn triangle_circumference(a: u32, b: u32, c: u32) -> u32 {
    a + b + c
}

pub fn triangle_area(a: u32, b: u32, c: u32) -> f32 {
    let s = get_semiperimeter(a, b, c);
    (((s * (s - a as f32) * (s - b as f32) * (s - c as f32)) as f32).sqrt() * 100.0).round() / 100.0
}

pub fn is_equilateral(a: u32, b: u32, c: u32) -> bool {
    a == b && b == c
}

pub fn is_isosceles(a: u32, b: u32, c: u32) -> bool {
    a == b || a == c || b == c
}

pub fn is_right(a: u32, b: u32, c: u32) -> bool {
    let smallest = min(a, min(b, c));
    let median = max(min(a, b), min(max(a, b), c));
    let largest = max(a, max(b, c));
    (smallest ^ 2) + (median ^ 2) == largest ^ 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn verify_triangle_panics_when_sum_of_two_sides_is_equal_to_the_third() {
        let a = 10;
        let b = 20;
        let c = a + b;
        verify_triangle(a, b, c);
    }

    #[test]
    #[should_panic]
    fn verify_triangle_panics_when_sum_of_two_sides_is_smaller_than_the_third() {
        let a = 10;
        let b = 20;
        let c = a + b + 1;
        verify_triangle(a, b, c);
    }

    #[test]
    fn verify_triangle_completes_normally_when_sum_of_two_sides_is_larger_than_the_third() {
        let a = 10;
        let b = 20;
        let c = a + b - 1;
        verify_triangle(a, b, c);
    }

    #[test]
    fn triangle_circumference_returns_circumference() {
        assert_eq!(triangle_circumference(10, 15, 20), 45);
    }

    #[test]
    fn triangle_area_returns_area() {
        assert_eq!(triangle_area(10, 15, 20), 72.62);
    }

    #[test]
    fn get_semiperimetr_returns_semiperimeter() {
        assert_eq!(get_semiperimeter(10, 15, 20), 22.5);
    }

    #[test]
    fn equilateral() {
        assert!(is_equilateral(5, 5, 5));
    }

    #[test]
    fn not_equilateral() {
        assert!(!is_equilateral(5, 5, 4));
    }

    #[test]
    fn isosceles() {
        assert!(is_isosceles(4, 5, 5));
    }

    #[test]
    fn equilateral_is_isosceles() {
        assert!(is_isosceles(5, 5, 5));
    }

    #[test]
    fn not_isosceles() {
        assert!(!is_isosceles(5, 7, 4));
    }

    #[test]
    fn right_triangle() {
        assert!(is_right(3, 4, 5));
    }

    #[test]
    fn not_right_triangle() {
        assert!(!is_right(3, 4, 6));
    }
}