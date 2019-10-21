mod calc;

extern crate actix_web;
extern crate assert_approx_eq;
extern crate clap;
extern crate postgres;
extern crate regex;

use actix_web::{web, App as ActixApp, HttpRequest, HttpServer, Responder};
use clap::{App, Arg};
use postgres::{Connection, TlsMode};
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use std::io::{self};
use std::str::FromStr;
use std::sync::Mutex;


// TODO: could just use generics as a hacky fix?
trait Store {
    fn new() -> Self;
    fn get_emails(&self) -> Vec<(String, bool)>;
    fn get_distances(&self) -> Vec<(calc::Point, calc::Point, f64)>;
    fn insert_email(&mut self, email: String, valid: bool);
    fn insert_distance(&mut self, p1: calc::Point, p2: calc::Point, distance: f64);
}

struct RealStore {
    conn: postgres::Connection
}

impl Store for RealStore {
    fn new() -> Self {
        let conn = Connection::connect("postgresql://postgres:docker@localhost/calcs", TlsMode::None)
            .unwrap();

        conn.execute(
            "create table if not exists distances (
                x1  double precision,
                y1  double precision,
                x2  double precision,
                y2  double precision,
                distance double precision
                )", &[]).unwrap();
        conn.execute(
            "create table if not exists emails (
                email text,
                valid bool
                )", &[]).unwrap();

        RealStore{ conn: conn}
    }

    fn get_emails(&self) -> Vec<(String, bool)> {
        let results = self.conn.query("
            select * from emails"
            , &[]).unwrap();
        let mut v = Vec::new();
        for r in &results {
            let email: String = r.get("email");
            let valid: bool = r.get("valid");
            v.push((email, valid));
        }
        v
    }

    fn get_distances(&self) -> Vec<(calc::Point, calc::Point, f64)> {
        let results = self.conn.query("
            select * from distances"
            , &[]).unwrap();
        let mut v = Vec::new();
        for r in &results {
            let p1 = calc::Point{ x: r.get("x1"), y: r.get("y1")};
            let p2 = calc::Point{ x: r.get("x2"), y: r.get("y2")};
            let dist = r.get("distance");
            v.push((p1, p2, dist));
        }
        v
    }

    fn insert_email(&mut self, email: String, valid: bool) {
        self.conn.execute("
            insert into emails (email, valid) values ($1, $2)"
            , &[&email, &valid]).unwrap();
    }

    fn insert_distance(&mut self, p1: calc::Point, p2: calc::Point, distance: f64) {
        self.conn.execute("
            insert into distances values ($1, $2, $3, $4, $5)"
            , &[&p1.x, &p1.y, &p2.x, &p2.y, &distance]).unwrap();
    }
}

fn get_emails(db: web::Data<Mutex<RealStore>>) -> impl Responder {
    let db = db.lock().unwrap();
    web::Json(db.get_emails())
}

fn get_distances(db: web::Data<Mutex<RealStore>>) -> impl Responder {
    let db = db.lock().unwrap();
    //serde_json::to_string(&db.get_distances());
    web::Json(db.get_distances())
}

// NOTE: json format for this is like [{"x":0.0,"y":0.0},{"x":1.0,"y":10.0}]
fn distance(db: web::Data<Mutex<RealStore>>,
            points: web::Json<(calc::Point, calc::Point)>)
            -> impl Responder {
    let mut db = db.lock().unwrap();
    let (p1, p2) = points.into_inner();
    let dist = calc::calc_shortest_distance(p1.clone(), p2.clone());
    db.insert_distance(p1, p2, dist);
    web::Json(dist)
}

fn email(db: web::Data<Mutex<RealStore>>,
            email: web::Json<String>)
            -> impl Responder {
    let mut db = db.lock().unwrap();
    let email = email.into_inner();
    let valid = calc::email_is_valid(email.clone());
    db.insert_email(email, valid);
    web::Json(valid)
}

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let matches = App::new("TDD_Practice")
        .arg(Arg::with_name("distance")
             .long("distance")
             .short("d")
             .takes_value(false)
             .help("Run the distance input/calculation routine"))
        .arg(Arg::with_name("email")
             .long("email")
             .short("e")
             .takes_value(false)
             .help("Run the email checking routine"))
        .arg(Arg::with_name("serve")
             .long("serve")
             .short("s")
             .takes_value(false)
             .help("Run the web server"))
        .get_matches();

    let mut s: RealStore = Store::new();

    if matches.is_present("serve") {
        let s = web::Data::new(Mutex::new(s));
        HttpServer::new(move || {
            ActixApp::new()
                .register_data(s.clone())
                .route("/api/emails", web::get().to(get_emails))
                .route("/api/email", web::post().to(email))
                .route("/api/distances", web::get().to(get_distances))
                .route("/api/distance", web::post().to(distance))
            })
            .bind("localhost:5000")
            .expect("Can not bind to port 5000")
            .run()
            .unwrap();

        return;
    }

    // TODO: breakout into functions
    if matches.is_present("distance") {
        for (p1, p2, d) in s.get_distances() {
            println!("point 1: {:?}, point 2: {:?}, distance: {}", p1, p2, d);
        }
        let mut x = String::new();
        let mut y = String::new();
        println!("Please enter the x coordinate of point 1:");
        io::stdin().read_line(&mut x).unwrap();
        println!("Please enter the x coordinate of point 1:");
        io::stdin().read_line(&mut y).unwrap();
        x = String::from(x.trim());
        y = String::from(y.trim());
        let point_1 = calc::Point{x: f64::from_str(&x).unwrap(), y: f64::from_str(&y).unwrap()};
        println!("Please enter the x coordinate of point 2:");
        x = "".to_string(); y = "".to_string();
        io::stdin().read_line(&mut x).unwrap();
        println!("Please enter the x coordinate of point 2:");
        io::stdin().read_line(&mut y).unwrap();
        x = String::from(x.trim());
        y = String::from(y.trim());
        let point_2 = calc::Point{x: f64::from_str(&x).unwrap(), y: f64::from_str(&y).unwrap()};
        let d = calc::calc_shortest_distance(point_1.clone(), point_2.clone());
        println!("The distance between point 1 and 2 is: {}", 
                 d);
        s.insert_distance(point_1, point_2, d);
    }

    if matches.is_present("email") {
        for (e, v) in s.get_emails() {
            println!("email: {}, valid: {}", e, v);
        }
        println!("Please enter an email address for validation:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).unwrap();
        let valid = calc::email_is_valid(String::from(email.trim()));
        match valid {
            true => println!("That email is valid!"),
            false => println!("That email is invalid."),
        }
        s.insert_email(String::from(email.trim()), valid);
    }
}

