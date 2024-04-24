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
Version of the classic command line search tool grep - searching in a specified file for a specified string and outputs the result. Arguments are file path (Poem.txt) and a search string ("body"). Output is stored in output.txt File: 
```
cargo run -- body Poem.txt > output.txt
```
```
Search Results:
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

## Project: Web Server
Creating web server on local address of 127:0:0:1:7878 and establishing TCP connection with threadpool of 4 threads to run the HTML scripts:

*  127:0:0:1:7878 - Displays main, _hello page_
*  127:0:0:1:7878/sleep - Webpage sleeps for 4 seconds and displays the _sleep page_
*  127:0:0:1:7878/"ANYTHING" - Displayes _error page_

![](https://github.com/dzmanashvilisaba/Rust/blob/main/proj_web_server/html_server_rust.png )
