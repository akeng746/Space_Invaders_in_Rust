Hello! 

This is a project that I made while following along to the instructions of Nathan Stocks and his Ultimate Rust Crash Course on Udemy. See his original Space Invaders-style game at https://github.com/CleanCut/invaders.

For those of you wondering why my sound is different than the original-- I used some of the sound contributions from others who have enrolled in the course before me. I give credit to Shining-Chen (https://github.com/Shining-Chen) for the startupDoMiReDo sound effects.

For some further notes on differences from the original: I needed to make some modifications in order to fix an issue I had where the "shot" left a trail that never disappeared. You can see how I created a field called prev_y for the Shot struct in shot.rs and forced the program to change the " | " pipe mark into a blank space "  " once prev_y != y. 

To play the game, simply open the terminal and type in "cargo run".

I had fun working with concurrency and implementing traits in Rust. I hope you enjoy the project as well!
