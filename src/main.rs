
fn main() {
    println!("Hello, world!");
}

// BMI Calculation
#[derive(Debug)]
enum bmi {
    UnderWeight(f64),
    NormalWeight(f64),
    OverWeight(f64),
    Obese(f64),
}

fn calc_bmi(height_feet: f64, height_inches: f64, weight_pounds: f64) -> Result<bmi, String> {
    let height_inches = height_inches + (height_feet * 12.0);
    let height_m = height_inches * 0.025;
    let weight_kg = weight_pounds * 0.45;
    let bmi = weight_kg / (height_m * height_m);
    match bmi {
        bmi if bmi <= 0.0 => Err(String::from("Impossible BMI")),
        bmi if bmi < 18.5 => Ok(bmi::UnderWeight(bmi)),
        bmi if bmi < 25.0 => Ok(bmi::NormalWeight(bmi)),
        bmi if bmi < 30.0 => Ok(bmi::OverWeight(bmi)),
        _ => Ok(bmi::Obese(bmi)),
    }
}

#[test]
fn bmi_does_not_crash() {
    calc_bmi(1.0, 1.0, 1.0);
}

#[test]
fn bmi_detects_impossible() {
    assert!(calc_bmi(1.0, 0.0, -90.0).is_err());
}

#[test]
fn bmi_returns_correct_enum() {
    match calc_bmi(5.0, 10.0, 100.0).unwrap() {
        bmi::UnderWeight(_) => (),
        _ => panic!(),
    }
    match calc_bmi(5.0, 10.0, 160.0).unwrap() {
        bmi::NormalWeight(_) => (),
        _ => panic!(),
    }
    match calc_bmi(5.0, 10.0, 200.0).unwrap() {
        bmi::OverWeight(_) => (),
        _ => panic!(),
    }
    match calc_bmi(5.0, 10.0, 300.0).unwrap() {
        bmi::Obese(_) => (),
        _ => panic!(),
    }
}


// Retirement savings
enum Age {
    Alive(u8),
    Dead,
}

fn calc_retirement(age: u8, salary: f64, savings_rate: f64, goal: f64) -> Age {
    let mut annual_savings = salary * savings_rate;
    annual_savings += annual_savings * 0.35;
    let years = (goal / annual_savings) as i64;
    let retirement_age = years + (age as i64);
    match retirement_age {
        0..=99 => Age::Alive(retirement_age as u8),
        _ => Age::Dead,
    }
}

#[test]
fn retirement_does_not_crash() {
    calc_retirement(0, 0.0, 0.0, 0.0);
}

#[test]
fn retirement_returns_dead() {
    match calc_retirement(99, 1.0, 0.1, 1000.0) {
        Age::Dead => (),
        _ => panic!(),
    }
}

#[test]
fn retirement_returns_correct_value() {
    if let Age::Alive(x) = calc_retirement(20, 100_000.0, 0.01, 5000.0) {
        println!("{}", x);
        assert_eq!(x, 23);
    }
}

// Shortest Distance
struct Point {
    x: f64,
    y: f64,
}

fn calc_shortest_distance(a: Point, b: Point) -> f64 {
    let x_dis = b.x - a.x;
    let y_dis = b.y - a.y;
    f64::sqrt((x_dis * x_dis) + (y_dis * y_dis))
}

#[test]
fn distance_does_not_crash() {
    calc_shortest_distance(Point{x: 1.0, y: 1.0}, Point{x: 2.0, y: 1.0});
}

#[test]
fn distance_horizontal() {
    assert_eq!(calc_shortest_distance(Point{x: 1.0, y: 1.0}, Point{x: 3.0, y: 1.0}),
                2.0);
}

#[test]
fn distance_diagonal() {
    let result = calc_shortest_distance(Point{x: 1.5, y: 1.0}, Point{x: 4.0, y: 3.0});
    if result > 3.202 || result < 3.200 {
        panic!();
    }
}



















