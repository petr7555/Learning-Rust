use triangle::*;

fn main() {
    let a = get_side_from_user("a");
    let b = get_side_from_user("b");
    let c = get_side_from_user("c");

    verify_triangle(a, b, c);

    println!("Circumference is {}.", triangle_circumference(a, b, c));
    println!("Area is {}.", triangle_area(a, b, c));
    println!(
        "Triangle is {}right.",
        if is_right(a, b, c) { "" } else { "not " }
    );
    println!(
        "Triangle is {}isosceles.",
        if is_isosceles(a, b, c) { "" } else { "not " }
    );
    println!(
        "Triangle is {}equilateral.",
        if is_equilateral(a, b, c) { "" } else { "not " }
    );
}
