# CI and Test Double Practice

Setting up a basic CRUD api with tests, integrated in Jenkins

### Notes

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

### 

#### My Experience with CI and Test Doubles

