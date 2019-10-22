use postgres::{Connection, TlsMode};
use super::calc;

// It is necessary to create a trait wrapper for the db
// in order to create test doubles, due to Rust's type system.
pub trait Store {
    fn new() -> Self;
    fn get_emails(&self) -> Vec<(String, bool)>;
    fn get_distances(&self) -> Vec<(calc::Point, calc::Point, f64)>;
    fn insert_email(&mut self, email: String, valid: bool);
    fn insert_distance(&mut self, p1: calc::Point, p2: calc::Point, distance: f64);
}

pub struct RealStore {
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


// Store mock implementation
pub struct MockStore {
}

impl Store for MockStore {
    fn new() -> Self {
        MockStore{}
    }
    fn get_emails(&self) -> Vec<(String, bool)> {
        unimplemented!()
    }
    fn get_distances(&self) -> Vec<(calc::Point, calc::Point, f64)> {
        unimplemented!()
    }
    fn insert_email(&mut self, email: String, valid: bool) {
        assert_eq!(email, "asdf");
        assert_eq!(valid, false);
    }
    fn insert_distance(&mut self, p1: calc::Point, p2: calc::Point, distance: f64) {
        assert_eq!((calc::Point{x: 0.0, y: 1.0}, calc::Point{x: 0.0, y: 0.0}), (p1, p2));
        assert_eq!(1.0, distance);
    }
}

// Store stub implementation
pub struct StubStore {
}

impl Store for StubStore {
    fn new() -> Self {
        StubStore{}
    }
    fn get_emails(&self) -> Vec<(String, bool)> {
        vec![(String::from("asdf"), false)]
    }
    fn get_distances(&self) -> Vec<(calc::Point, calc::Point, f64)> {
        vec![(calc::Point{x: 0.0, y: 1.0}, calc::Point{x: 0.0, y: 0.0}, 1.0)]
    }
    fn insert_email(&mut self, _email: String, _valid: bool) {
    }
    fn insert_distance(&mut self, _p1: calc::Point, _p2: calc::Point, _distance: f64) {
    }
}
