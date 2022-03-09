# word-type-formatter

## Problem

Given a space-seperated list of words, format each word according to the following rules:

- Fruits should be in all caps (e.g. "BANANA")
- Vegetables should be bracketed (e.g. "[carrot]")
- Animals should be filled with asterisks (e.g. h*o*r*s*e)

Return a string of the formatted space-seperated words
Lists of supported fruits, vegetables, and animals are provided below.
When the string contains an unrecongized word, return a string stating: "Unknown word: <WORD>" where "<WORD>" is replaced by the unknown word (e.g. "Unknown word: chair)

## Goals

- This problem is intended to be simple to solve algorithmically
- The goad is to evalute your ablity to structure your solution such that it is extensible, maintainable, and testable.
- HINT: Your solution should include one or more "classes"

## Supported Words

### Fruits

- apple
- banana
- mango

### Vegetables

- carrot
- zucchini
- broccoli

### Animals

- horse
- giraffe
- mouse
- pigeon


## usage

- Run tests

```sh
cargo test
```

- Run with input

```sh
cargo run -- -i <SOME_STRING>
```
