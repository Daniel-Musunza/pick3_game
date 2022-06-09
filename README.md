# Smart Contract for Near certification
The code for the Smart Contract is in the src lib.rs. The Smart contract is inspired by the combat example but this one prompts the Host to enter the number of participants(it loops until the last participant guesses his/her lucky number). In this Smart contract each participant is supposed to enter two lucky numbers between 1 and 9 without repeating a number e.g 12(of which interms of code it's a single two digit number). The program code generates a random number between 10 and 99 but numbers with 0(eg 20) or with repeated digits(eg 77) are not generated. if either of the participants gueses the right number he/she Wins otherwise loses.

The smart contracts also has tests.

## Software Required

Rust 1.58 + cargo
Node.js
NEAR CLI 3.1

## Author
 Daniel Musunza