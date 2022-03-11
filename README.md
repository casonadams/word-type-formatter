# word-type-formatter

## Problem

**Given** a space-seperated list of words

**WHEN** formating each word according to the following rules:

- Fruits should be in all caps (e.g. "BANANA")
- Vegetables should be bracketed (e.g. "[carrot]")
- Animals should be filled with asterisks (e.g. h\*o\*r\*s\*e)
- An unrecongized word, should return a string stating: "Unknown word: WORD"
  where "WORD" is replaced by the unknown word (e.g. "Unknown word: chair")

**THEN** return a string of the formatted space-seperated words **OR** return
"Unknown word: WORD"

Lists of supported fruits, vegetables, and animals are provided below.

## Goals

- This problem is intended to be simple to solve algorithmically
- The goad is to evalute your ablity to structure your solution such that it is
  extensible, maintainable, and testable.
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

- Pipe known words string to program

```sh
printf '%s' 'apple banana mango carrot zucchini broccoli horse giraffe mouse pigeon' | cargo run -q
# => APPLE BANANA MANGO [carrot] [zucchini] [broccoli] h*o*r*s*e g*i*r*a*f*f*e m*o*u*s*e p*i*g*e*o*n
```

- Pipe unknown words string to program

```sh
printf '%s' 'Apple' | cargo run -q
# => Unknown word: Apple
```
