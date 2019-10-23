# CI and Test Double Practice

Setting up a basic CRUD api with tests and test doubles, integrated in Jenkins

### Notes
Rust is not a language that easily accomidates usage of test doubles.
Its type system is very strict compared to dynamic languages, and as a result, I needed to wrap the database in a Trait struct (like an interface) that managed the db connection.
However, even using traits, there are a number of pitfalls involving the choice Rust gives of either using dynamic dispatch (dyn Trait objects) or the preferred method, bounded generics (impl Trait).
While I was able to get the test double working for the database tests using impl Trait, I was unable to navigate the complexities of the type system when it came to using a test double in the web server itself.
In order to make all tests pass (specifically, the http tests), you will need to get the postgres docker image running. That process is described below in the Installation section.

As a side note, I used the server test capabilities provided by Actix to include the http tests as part of the code. 

### API
The running server has four url paths.
These return the array of database entries in JSON format:
```
http://localhost:5000/api/emails
http://localhost:5000/api/distances
```

These take POST requests, with an associated JSON format:
```
// Send a POST request with this format, with quotes included: "emailnamehere@mailprovider.com"
http://localhost:5000/api/email

// Send a POST request with this format: [{"x":0.0,"y":0.0},{"x":1.0,"y":10.0}]
http://localhost:5000/api/distance
```

### Videos

### How To Install And Compile

First, install Rust using the instructions provided at this link:

https://www.rust-lang.org/tools/install

This code was tested on a Linux machine, so while it should work on Windows, for best results, use Linux.
Get this code by cloning this git repo.
```
git clone https://github.com/camgrodgers/ppa2.git
```
Then enter the directory and use Cargo (Rust's package manager and build system) to compile and run.
```
// "run" should be followed by -- to make sure that following arguments are sent to the program
cargo run -- --help
```
The help flag will print out a list of options for the program:
```
     Running `target/debug/Test_Driven_Practice_1 --help`
TDD_Practice

USAGE:
    PPA2 [FLAGS]

FLAGS:
    -d, --distance    Run the distance input/calculation routine
    -e, --email       Run the email checking routine
    -h, --help        Prints help information
    -s, --serve       Run the web server
    -V, --version     Prints version information
```
All of the routines depend on a running instance of the postgresql database.
The server flag --serve will prevent the other flags from running, and will start the server.
Use any combination of custom flags to run the various routines. The program will prompt for input.

In order to get the database image, pull it from my repo:
```
docker pull camgrodgers/calcdb
```
And then run it with this command:
```
sudo docker run --rm   --name calcdb -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v $HOME/docker/volumes/postgres:/var/lib/postgresql/data  postgres
```
If the tests still fail, it's because docker didn't package the database. You will have to create the database through the postgres cli, and then the program will create the tables when you run it.

#### My Experience with CI and Test Doubles
I am not convinced of the utility of test doubles that don't spin up a real database, especially in strongly-typed, compiled languages like Rust. 
While there may be a lot of pitfalls in scripting languages that are worth testing using mocks, fakes, dummies, etc., I think many of those pitfalls don't exist in Rust.
On the other hand, the scripting languages make it easy to create test doubles, while Rust makes it difficult.
I was, however, glad to be able to use the TestServer functionality that is provided by Actix-web, the backend library that I used in this repo. 

My experience with Docker and Jenkins for CI was overall not bad, but I noticed there is a lot of depth and complexity that I haven't engaged with yet.
I didn't dockerize most of my pipeline, and I don't know if that's good or bad. 

