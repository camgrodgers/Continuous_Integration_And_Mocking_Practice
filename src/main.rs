mod calc;
mod store;

use store::*;

extern crate actix_files;
extern crate actix_http_test;
extern crate actix_web;
extern crate assert_approx_eq;
extern crate clap;
extern crate postgres;
extern crate regex;

use actix_web::{web, App as ActixApp, HttpServer, Responder};
use actix_http::HttpService;
use actix_http_test::TestServer;

use clap::{App, Arg};
use std::io::{self};
use std::str::FromStr;
use std::sync::Mutex;



fn get_emails(db: web::Data<Mutex<store::RealStore>>) -> impl Responder {
    let db = db.lock().unwrap();
    web::Json(db.get_emails())
}

#[test]
fn http_returns_emails() {
    let s: store::RealStore = store::Store::new();
    let s = web::Data::new(Mutex::new(s));
    let mut srv = TestServer::new( move || {
        HttpService::new(
            ActixApp::new()
                .register_data(s.clone())
                .route("/api/emails", web::get().to(get_emails))
        )});

    let req = srv.get("/api/emails");
    let mut response = srv.block_on(req.send()).unwrap();
    assert!(response.status().is_success());
    let bytes = srv.block_on(response.body()).unwrap();
    let body = std::str::from_utf8(&bytes).unwrap();
    let _v: Vec<(String, bool)> = serde_json::from_str(body).unwrap();
}


fn get_distances(db: web::Data<Mutex<store::RealStore>>) -> impl Responder {
    let db = db.lock().unwrap();
    web::Json(db.get_distances())
}

#[test]
fn http_returns_distances() {
    let s: store::RealStore = store::Store::new();
    let s = web::Data::new(Mutex::new(s));
    let mut srv = TestServer::new( move || {
        HttpService::new(
            ActixApp::new()
                .register_data(s.clone())
                .route("/api/distances", web::get().to(get_distances))
        )});

    let req = srv.get("/api/distances");
    let mut response = srv.block_on(req.send()).unwrap();
    assert!(response.status().is_success());
    let bytes = srv.block_on(response.body()).unwrap();
    let body = std::str::from_utf8(&bytes).unwrap();
    let _v: Vec<(calc::Point, calc::Point, f64)> = serde_json::from_str(body).unwrap();
}


// NOTE: json format for this is like [{"x":0.0,"y":0.0},{"x":1.0,"y":10.0}]
fn distance(db: web::Data<Mutex<store::RealStore>>,
            points: web::Json<(calc::Point, calc::Point)>)
            -> impl Responder {
    let mut db = db.lock().unwrap();
    let (p1, p2) = points.into_inner();
    web::Json(calc::calc_shortest_distance_store(p1, p2, &mut *db))
}

#[test]
fn http_calculates_distance() {
    let s: store::RealStore = store::Store::new();
    let s = web::Data::new(Mutex::new(s));
    let mut srv = TestServer::new( move || {
        HttpService::new(
            ActixApp::new()
                .register_data(s.clone())
                .route("/api/distance", web::post().to(distance))
        )});

    let req = srv.post("/api/distance");
    let message = (calc::Point{x: 0.0, y: 5.0}, calc::Point{x: 10.0, y: 10.0});
    let mut response = srv.block_on(req.send_json(&message)).unwrap();
    assert!(response.status().is_success());
    let bytes = srv.block_on(response.body()).unwrap();
    let body = std::str::from_utf8(&bytes).unwrap();
    let _distance: f64 = serde_json::from_str(body).unwrap();
}

// Emails are sent in format with quotes like "annie@yodelehi.hoo"
fn email(db: web::Data<Mutex<store::RealStore>>,
            email: web::Json<String>)
            -> impl Responder {
    let mut db = db.lock().unwrap();
    let email = email.into_inner();
    web::Json(calc::email_is_valid_store(email, &mut *db))
}

#[test]
fn http_validates_email() {
    let s: store::RealStore = store::Store::new();
    let s = web::Data::new(Mutex::new(s));
    let mut srv = TestServer::new( move || {
        HttpService::new(
            ActixApp::new()
                .register_data(s.clone())
                .route("/api/email", web::post().to(email))
        )});

    let req = srv.post("/api/email");
    let mut response = srv.block_on(req.send_json(&String::from("asdf"))).unwrap();
    assert!(response.status().is_success());
    let bytes = srv.block_on(response.body()).unwrap();
    let body = std::str::from_utf8(&bytes).unwrap();
    let valid: bool = serde_json::from_str(body).unwrap();
    assert_eq!(valid, false);
}

fn serve(s: store::RealStore) {
    let s = web::Data::new(Mutex::new(s));
    HttpServer::new(move || {
        ActixApp::new()
            .register_data(s.clone())
            .route("/api/emails", web::get().to(get_emails))
            .route("/api/email", web::post().to(email))
            .route("/api/distances", web::get().to(get_distances))
            .route("/api/distance", web::post().to(distance))
            .service(actix_files::Files::new("/static", "./static"))
        })
        .bind("localhost:5000")
        .expect("Can not bind to port 5000")
        .run()
        .unwrap();
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

    let mut s: store::RealStore = store::Store::new();

    if matches.is_present("serve") {
        serve(s);
        return;
    }

    // TODO: breakout into functions?
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
        let d = calc::calc_shortest_distance_store(point_1, point_2, &mut s);
        println!("The distance between point 1 and 2 is: {}", 
                 d);
    }

    if matches.is_present("email") {
        for (e, v) in s.get_emails() {
            println!("email: {}, valid: {}", e, v);
        }
        println!("Please enter an email address for validation:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).unwrap();
        let valid = calc::email_is_valid_store(email, &mut s);
        match valid {
            true => println!("That email is valid!"),
            false => println!("That email is invalid."),
        }
    }
}

