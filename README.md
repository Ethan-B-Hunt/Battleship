## Overview

In this implementation of Hangman, the game is played in the terminal, where the player guesses letters to figure out a hidden word. Each incorrect guess reduces the number of lives, and the game ends when either the word is guessed or the player runs out of lives.

**Watch the demonstration**: [Hangman Game Demo](https://youtu.be/PUy9emAo2Ck)

### Features:
- **Word Selection**: The game can choose random words or words of a specific length, making it customizable.
- **Lives Tracking**: Players are given a set number of lives, which decrease with each incorrect guess.
- **Guessing**: Players enter one letter at a time, with the game displaying correctly guessed letters and hiding unguessed ones with underscores.
- **Letter Bank**: Shows the letters that have already been guessed, helping the player avoid repetition.
- **Game End Detection**: The game automatically detects when the player has either guessed the word or lost all their lives.

# Useful Resources

- [Rust Documentation](https://www.rust-lang.org/learn)
- [Introduction to Rust Programming Language](https://www.geeksforgeeks.org/introduction-to-rust-programming-language/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)