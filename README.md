# Weekly Exploratory Demos
This repository will hold my weekly demos for exploring rust and cloud computing.

## Week 2
random: In week 2 I created a command line tool that interacts with the rand crate. There are two functions: coin, which returns the output of a coin flip (heads if the random boolean output is 1, tails otherwise) and coin_n, which takes a integer input n and flips that many coins, returning the proportion of heads. The commands are flip for the single coin flip and flip-n for multiple coin flips. I also implemented two tests, one for each of the functions.

To run a single coin flip, cd into the random directory and run `cargo run -- flip`
To run 15 coin flips and generate the proportion of heads, cd into the random directory and run `cargo run -- flip-n --n 15`

## Week 3
plotting: In week 3 I create a plotting function that generates two vectors with 100 random values between -100 and 100 and plots them on a scatter plot. It then saves the scatter plot to a .png file that can be viewed. This function uses the plotters crate to generate the plot and the rand crate to generate the vectors. After saving the plot, it prints "Plot finished" so the user knows that the plot is ready to be viewed.

To run the function, cd into the plotting directory and run `cargo run`

## Week 4
temperature: In week 4 I created a command line tool that converts temperatures between Farenheit, Celsius, and Kelvin. When running the module, the user must specify what measurement system they are starting in (see below for how to run it in the command line). It will then ask the user for the value to convert. Then the module calls the appropriate function to convert the value into the other two measurement systems and returns the results. 

To run the function, cd into the temperature directory. 
If you'd like to convert a temperature from Farenheit, run `cargo run -- farenheit`
If you'd like to convert a temperature from Celsius, run `cargo run -- celsius`
If you'd like to convert a temperature from Kelvin, run `cargo run -- kelvin`

## References

* [rust-mlops-template](https://github.com/nogibjj/rust-mlops-template)



