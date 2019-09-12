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
There is 100% test coverage on all of the functions except for (possibly) the email validator, as all code paths are covered.
The email validator is covered on both "code paths," but the regex portion of the function would require much more intensive testing for full coverage.
I was unable to find and install a suitable code coverage tester for Rust at this time.

### Installing

A step by step series of examples that tell you how to get a development env running

Say what the step will be

```
Give the example
```

And repeat

```
until finished
```

End with an example of getting some data out of the system or using it for a little demo

