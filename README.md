# TDD Practice 1

Writing some basic functions with Test-Driven Development.

### How To Install And Compile

First, install Rust using the instructions provided at this link:

https://www.rust-lang.org/tools/install

This code was tested on a Linux machine, so while it should work on Windows, for best results, use Linux.
Get this code by cloning this git repo.
```
git clone https://github.com/camgrodgers/Test_Driven_Practice_1.git
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
    Test_Driven_Practice_1 [FLAGS]

FLAGS:
        --bodymass      Run the BMI input/calculation routine
        --distance      Run the distance input/calculation routine
        --email         Run the email checking routine
    -h, --help          Prints help information
        --retirement    Run the retirement savings routine
    -V, --version       Prints version information
```
Use any combination of custom flags to run the various routines. The program will prompt for input.

### Testing Process

In order to run the tests, use "cargo test." Here are the current test suite results:
```
$ cargo test
Compiling Test_Driven_Practice_1 v0.1.0
running 15 tests
test bmi_does_not_crash ... ok
test bmi_detects_impossible ... ok
test bmi_returns_correct_enum ... ok
test bmi_returns_correct_value ... ok
test distance_diagonal ... ok
test distance_does_not_crash ... ok
test distance_horizontal ... ok
test retirement_does_not_crash ... ok
test retirement_returns_correct_value ... ok
test retirement_returns_dead ... ok
test email_rejects_numeric_first_char ... ok
test email_rejects_period_first_char ... ok
test email_rejects_garbage ... ok
test email_rejects_double_period ... ok
test email_accepts_valid ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
I used cargo-tarpaulin to test code coverage of the functions and their tests (leaving out main).

https://crates.io/crates/cargo-tarpaulin

It says that the code has 91.87% coverage.
```
running 15 tests
...............
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

[INFO tarpaulin] Coverage Results:
|| Tested/Total Lines:
|| src/main.rs: 68/74
|| 
91.89% coverage, 68/74 lines covered
```
I am not sure if it's possible to get per-function coverage ratings with tarpaulin, and overall I think that code coverage tools in Rust are lacking at the moment.

#### My Experience with TDD

I hadn't used testing frameworks very much before this assignment, as I found the experience to be clunky with the languages I was using.
Usually, I would test by running the code, using a debugger, putting print statements in the code, and so on.
However, the experience of testing code in Rust is very pleasant, and it is helped by Rust's type system and borrow checker.
Since Rust prohibits many types of errors that would be a testing concern in many other languages (dereferencing null pointers, incorrect types, memory issues, etc),
I was able to focus almost solely on testing for logic errors. Being able to run all the tests using Cargo and do everything in the CLI was also pleasant.
I now see that there's a definite advantage to TDD. It is less haphazard than keeping a bunch of edge cases in my head and running them from input repeatedly.
For personal projects, I'm now convinced that I should use tests very early on, but I'm not convinced that purely dogmatic TDD (always write a failing test first) is always necessary.

### Naming And Organization

Because this is a simple project, I kept everything in the main file.
I separated the functions and their associated types and tests using comments. E.G.:
```
// BMI Calculation
#[derive(Debug)]
enum bmi {
    UnderWeight(f64),
	// ...
    Obese(f64),
}

fn calc_bmi(height_feet: f64, height_inches: f64, weight_pounds: f64) -> Result<bmi, String> {
    let height_inches = height_inches + (height_feet * 12.0);
    // ...
        _ => Ok(bmi::Obese(bmi)),
    }
}

#[test]
fn bmi_does_not_crash() {
    calc_bmi(1.0, 1.0, 1.0);
}
// ...

// Retirement savings
enum Age {
// ...
```
I prefaced each test function name with the name of the function it was associated with.
Tests for the retirement function are prefaced with "retirement" and so on. 
For general naming conventions, I followed the Rust guidelines:

https://rust-lang-nursery.github.io/api-guidelines/naming.html

### Videos
Here's the video of the Red, Green, Refactor process:

https://github.com/camgrodgers/Test_Driven_Practice_1/raw/master/RGR_cameron_rodgers.mp4

Here's the video of all the functions running:

https://github.com/camgrodgers/Test_Driven_Practice_1/raw/master/demo_cameron_rodgers.mp4
