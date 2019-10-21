use regex::Regex;
use assert_approx_eq::*;
use serde::{Serialize, Deserialize};


// Shortest Distance
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn calc_shortest_distance(a: Point, b: Point) -> f64 {
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
pub fn email_is_valid(email: String) -> bool {
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
