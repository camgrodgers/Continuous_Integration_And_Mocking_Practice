# TDD Practice 1

Writing some basic functions with Test-Driven Development.

## How To Install And Compile

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
Use any combination of custom flags to run the various routines.



### Prerequisites

What things you need to install the software and how to install them

```
Give examples
```

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

