extern crate regex;
extern crate clap;
#[macro_use]
extern crate assert_approx_eq;
//use assert_approx_eq::assert_approx_eq;
use regex::Regex;
use clap::{App, Arg};
use std::io::{self};
use std::str::FromStr;

fn main() {
    let matches = App::new("TDD_Practice")
        .arg(Arg::with_name("bodymass")
             .long("bodymass")
             .takes_value(false)
             .help("Run the BMI input/calculation routine"))
        .arg(Arg::with_name("distance")
             .long("distance")
             .takes_value(false)
             .help("Run the distance input/calculation routine"))
        .arg(Arg::with_name("retirement")
             .long("retirement")
             .takes_value(false)
             .help("Run the retirement savings routine"))
        .arg(Arg::with_name("email")
             .long("email")
             .takes_value(false)
             .help("Run the email checking routine"))
        .get_matches();

    // These are ugly because I'm in a rush
    if matches.is_present("bodymass") {
        println!("Please enter your height in feet (not including inches):");
        let mut height_ft = String::new();
        io::stdin().read_line(&mut height_ft).unwrap();
        println!("Please enter your height in inches (not including feet):");
        let mut height_in = String::new();
        io::stdin().read_line(&mut height_in).unwrap();
        println!("Please enter your weight in pounds:");
        let mut weight = String::new();
        io::stdin().read_line(&mut weight).unwrap();
        let height_ft = f64::from_str(height_ft.trim()).unwrap();
        let height_in = f64::from_str(height_in.trim()).unwrap();
        let weight = f64::from_str(weight.trim()).unwrap();
        let bmi_val = match calc_bmi(height_ft, height_in, weight) {
            Ok(val) => val,
            Err(_) => { println!("invalid inputs"); return; },
        };
        print!("You are ");
        match bmi_val {
            bmi::UnderWeight(x) => println!("underweight with a BMI of {}.",x),
            bmi::NormalWeight(x) => println!("normal with a BMI of {}.",x),
            bmi::OverWeight(x) => println!("over weight with a BMI of {}.",x),
            bmi::Obese(x) => println!("obese with a BMI of {}.",x),
        }
    }
             
    if matches.is_present("distance") {
        let mut x = String::new();
        let mut y = String::new();
        println!("Please enter the x coordinate of point 1:");
        io::stdin().read_line(&mut x).unwrap();
        println!("Please enter the x coordinate of point 1:");
        io::stdin().read_line(&mut y).unwrap();
        x = String::from(x.trim());
        y = String::from(y.trim());
        let point_1 = Point{x: f64::from_str(&x).unwrap(), y: f64::from_str(&y).unwrap()};
        println!("Please enter the x coordinate of point 2:");
        x = "".to_string(); y = "".to_string();
        io::stdin().read_line(&mut x).unwrap();
        println!("Please enter the x coordinate of point 2:");
        io::stdin().read_line(&mut y).unwrap();
        x = String::from(x.trim());
        y = String::from(y.trim());
        let point_2 = Point{x: f64::from_str(&x).unwrap(), y: f64::from_str(&y).unwrap()};
        println!("The distance between point 1 and 2 is: {}", 
                 calc_shortest_distance(point_1, point_2));
    }

    if matches.is_present("retirement") {
        println!("Please enter your current age in years:");
        let mut age = String::new();
        io::stdin().read_line(&mut age).unwrap();
        let age = u8::from_str(age.trim()).unwrap();
        println!("Please enter your annual salary:");
        let mut salary = String::new();
        io::stdin().read_line(&mut salary).unwrap();
        let salary = f64::from_str(salary.trim()).unwrap();
        println!("Please enter your savings rate:");
        let mut rate = String::new();
        io::stdin().read_line(&mut rate).unwrap();
        let rate = f64::from_str(rate.trim()).unwrap();
        println!("Please enter your savings goal:");
        let mut goal = String::new();
        io::stdin().read_line(&mut goal).unwrap();
        let goal = f64::from_str(goal.trim()).unwrap();
        match calc_retirement(age, salary, rate, goal) {
            Age::Dead => println!("You'll die before you finish saving up."),
            Age::Alive(age_fin) => println!("You'll be {} years old when you finish saving.", age_fin),
        }
    }

    if matches.is_present("email") {
        println!("Please enter an email address for validation:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).unwrap();
        match email_is_valid(String::from(email.trim())) {
            true => println!("That email is valid!"),
            false => println!("That email is invalid."),
        }
    }
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
    calc_bmi(1.0, 1.0, 1.0).unwrap();
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

#[test]
fn bmi_returns_correct_value() {
    match calc_bmi(6.0, 0.0, 300.0).unwrap() {
        bmi::Obese(bmi) => assert_approx_eq!(bmi, 41.66666, 0.001),
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
    assert_approx_eq!(result, 3.20156, 0.0001)
}

// Email validation
fn email_is_valid(email: String) -> bool {
    Regex::new(
        r"^[^\d\.][\w\d\.!${+%*-=?^_}|]+\w@[\w\.]+\.\w{1,4}$"
        ).unwrap().is_match(&email)
    & !email.contains("..")
}

#[test]
fn email_accepts_valid() {
    assert_eq!(true, email_is_valid(String::from("annie.!%*-=?^_}|${+1@mail.gmail.com")));
    assert_eq!(true, email_is_valid(String::from("annie@gmail.com")));
}

#[test]
fn email_rejects_garbage() {
    assert_eq!(false, email_is_valid(String::from("erioggmdsm.v.vr..[[\\(")));
}

#[test]
fn email_rejects_double_period() {
    assert_eq!(false, email_is_valid(String::from("ann..ie@gmail.com")));
}

#[test]
fn email_rejects_numeric_first_char() {
    assert_eq!(false, email_is_valid(String::from("1annie@gmail.com")));
}

#[test]
fn email_rejects_period_first_char() {
    assert_eq!(false, email_is_valid(String::from(".annie@gmail.com")));
}
