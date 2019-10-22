use regex::Regex;
use assert_approx_eq::*;
use serde::{Serialize, Deserialize};
use super::store::*;

// Shortest Distance
#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn calc_shortest_distance_store(a: Point, b: Point, db: &mut impl Store) -> f64 {
    let dist = calc_shortest_distance(a, b);
    db.insert_distance(a, b, dist);
    dist
}

#[test]
fn mock_mock_shortest_distance() {
    let mut s = MockStore::new();
    let (a, b) = (Point{x: 0.0, y: 1.0}, Point{x: 0.0, y: 0.0});
    calc_shortest_distance_store(a, b, &mut s);
}

#[test]
#[should_panic]
fn mock_mock_shortest_distance_should_panic() {
    let mut s = MockStore::new();
    let (a, b) = (Point{x: 99.0, y: 1.0}, Point{x: 99.0, y: 0.0});
    calc_shortest_distance_store(a, b, &mut s);
}

#[test]
fn mock_dummy_shortest_distance() {
    let mut s = StubStore::new();
    let (a, b) = (Point{x: 0.0, y: 1.0}, Point{x: 0.0, y: 0.0});
    calc_shortest_distance_store(a, b, &mut s);
    s.get_distances();
}

#[test]
fn mock_stub_shortest_distance() {
    let mut s = StubStore::new();
    let (a, b) = (Point{x: 0.0, y: 1.0}, Point{x: 0.0, y: 0.0});
    calc_shortest_distance_store(a, b, &mut s);
    let v = s.get_distances();
    assert_eq!(v[0], (a, b, 1.0));
}

pub fn calc_shortest_distance(a: Point, b: Point) -> f64 {
    let x_dis = b.x - a.x;
    let y_dis = b.y - a.y;
    f64::sqrt((x_dis * x_dis) + (y_dis * y_dis))
}

#[test]
fn unit_distance_does_not_crash() {
    calc_shortest_distance(Point{x: 1.0, y: 1.0}, Point{x: 2.0, y: 1.0});
}

#[test]
fn unit_distance_horizontal() {
    assert_eq!(calc_shortest_distance(Point{x: 1.0, y: 1.0}, Point{x: 3.0, y: 1.0}),
                2.0);
}

#[test]
fn unit_distance_diagonal() {
    let result = calc_shortest_distance(Point{x: 1.5, y: 1.0}, Point{x: 4.0, y: 3.0});
    assert_approx_eq!(result, 3.20156, 0.0001)
}

// Email validation
pub fn email_is_valid_store(email: String, db: &mut impl Store) -> bool {
    let valid = email_is_valid(email.clone());
    db.insert_email(email, valid);
    valid
}

#[test]
fn mock_mock_email() {
    let mut s = MockStore::new();
    email_is_valid_store(String::from("asdf"), &mut s);
}

#[test]
#[should_panic]
fn mock_mock_email_should_panic() {
    let mut s = MockStore::new();
    email_is_valid_store(String::from("bobby@hotmail.net"), &mut s);
}


#[test]
fn mock_dummy_email() {
    let mut s = StubStore::new();
    email_is_valid_store(String::from("asdf"), &mut s);
    s.get_emails();
}

#[test]
fn mock_stub_email() {
    let mut s = StubStore::new();
    email_is_valid_store(String::from("asdf"), &mut s);
    let v = s.get_emails();
    assert_eq!(v[0], (String::from("asdf"), false));
}

pub fn email_is_valid(email: String) -> bool {
    Regex::new(
        r"^[^\d\.][\w\d\.!${+%*-=?^_}|]+\w@[\w\.]+\.\w{1,4}$"
        ).unwrap().is_match(&email)
    & !email.contains("..")
}

#[test]
fn unit_email_accepts_valid() {
    assert_eq!(true, email_is_valid(String::from("annie.!%*-=?^_}|${+1@mail.gmail.com")));
    assert_eq!(true, email_is_valid(String::from("annie@gmail.com")));
}

#[test]
fn unit_email_rejects_garbage() {
    assert_eq!(false, email_is_valid(String::from("erioggmdsm.v.vr..[[\\(")));
}

#[test]
fn unit_email_rejects_double_period() {
    assert_eq!(false, email_is_valid(String::from("ann..ie@gmail.com")));
}

#[test]
fn unit_email_rejects_numeric_first_char() {
    assert_eq!(false, email_is_valid(String::from("1annie@gmail.com")));
}

#[test]
fn unit_email_rejects_period_first_char() {
    assert_eq!(false, email_is_valid(String::from(".annie@gmail.com")));
}
