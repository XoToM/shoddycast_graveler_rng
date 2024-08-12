# Graveler softlock diceroll simulator

## What is this?
This is my submission for the challange submitted in [this video](https://www.youtube.com/watch?v=M8C8dHQE2Ro). The goal of this challange is to write a program which rolls a 4 sided die 231 times, then counts the amount of 1s rolled. It then repeats this process 1 billion times to check what the biggest number of 1s rolled is.

## My optimizations
I've made the code able to run on multiple cpu cores (use multi-threading), and I've also made several adjustments to the algorithm such as using a faster rng algorithm, and calling the rng algorithm less times.

As a result of using Rust the above optimisations, the time needed to process the full 1 billion iterations went from 8 days (time from your video) to below 1.5 seconds (my time) which is over x460800 times faster.

## What else could be done?
This new time is still not perfect though. This code can be easily ran in parallel, meaning that it could be ported over to run on a graphics card. Despite their name, graphics cards can be used for processing things other than graphics. Graphics cards are really really good at running simple calculations hundreds if not thousands of times in parallel, which makes them perfect for processing large amounts of data. I think this task could be set up to run on a graphics card, meaning that it could finish significantly faster than this solution.

Even if we assume that we do not want to run this code on a graphics card, I doubt this solution is perfect. My code doesn't use SIMD instructions which are special a special type of cpu instructions for processing large amounts of data in parallel. By properly porting over this code to SIMD instructions this code could be made aproximately 8 times faster.

## A quick note on the problem we are trying to solve here
Both the video mentioned above, and the original video descibing the softlock were written with an incorrect assumption that computers are capable of producing random numbers. Computers are devices which can only follow their instructions, and they are incapable of producing truly random sequences of numbers. The best computers can do is to use algorithms to generate sequences of numbers which look random. Some of these pseudo random number generators can also be influenced by other external inputs a computer can measure to produce numbers which appear more random, however if those inputs are reproduced, the sequences of the random numbers can also be recreated.

My point here is that these sequences are deterministic, and due to limitations of computers and these rng algoithms not every sequence of numbers may be achievable. This is just a theory, but it may be possible that the algorithm the Pokemon games use to generate their random numbers may in fact not be capable of outputting the sequence of values needed to escape the softlock described in the videos. This would mean that this pseudo-softlock is an actual unescapeable softlock.

# How to run

To run this code you have to download and install the Rust build tools. You can follow [the official Rust installation guide](https://www.rust-lang.org/tools/install) for this.

Once you have installed Rust on your system, open a command prompt/powershell/terminal window in this project's directory and type in `cargo run --release` or `cargo build --release`.

Cargo is a tool for managing Rust projects and their dependencies, and the above commands will download and compile this project and its dependencies.

If you use the `cargo run --release` command Cargo will also run this project as soon as it is compiled.

If you use the `cargo build --release` command you should be able to find the compiled program in the `target/release/` folder.

# Important note
Please be aware that the time this program may take to run will depend on the device it is running on and its cpu. I have done all of my benchmarks with an Intel i7-11800H cpu which is a relatively recent laptop cpu. This also means that the program might perform better on a desktop/server, or it might perform worse on older devices.

# Credits

All of the code written in this repository was written by me without the use of generative AI, however this project does use Rust libraries which were written by other people. The following libraries were used, and all the credit for making them goes to their authors and their maintainers:

- [fastrand](https://crates.io/crates/fastrand) - A fast random number generator used by this project.
- [rand, rand_xoshiro, rand_isaac](https://crates.io/crates/rand) - Alternative rng sources. They are not used by default, but I've left in comments on how to enable them in the code.
- [rayon](https://crates.io/crates/rayon) - A library for assisting with running rust code in parallel.