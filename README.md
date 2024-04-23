# Rust Codes

Introductory and intermediate projcets practiced to learn the concepts of rust language, excerpted from the book: "The Rust Programming Language" by Rust developers Steve Klabnik and Carol Nichols.

## Project: Guessing Game 

User enter's the number until it guesses the secret number.

```
Guess the number!
Please input your guess.
50
You guessed: 50
Too big!
Please input your guess.
25
You guessed: 25
Too big!
Please input your guess.
12
You guessed: 12
Too small!
Please input your guess.
18
You guessed: 18
Too small!
Please input your guess.
22
You guessed: 22
You win!
```

## Project: Command Line
Version of the classic command line search tool grep - searching in a specified file for a specified string. Arguments are file path
and a saerch string.
```
cargo run -- body Poem.txt > output.txt
```
