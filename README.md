# RECL

A simple program to record CLI with.

> Currently no support for recording stdin.

## Usage

#### Record

Record command's output to file.

```
recl r|record <file> <command>
```

#### Play

Play recording from file.

```
recl p|play <file>
```

#### Example

>     $ recl r log sh -c 'echo 1; sleep 1; echo 2'
>     1
>     2

>     $ cat log
>     1 4 49 10
>     1 1005 50 10

>     $ recl p log
>     1
>     2

## Installation

#### [Cargo](https://doc.rust-lang.org/cargo)

```
cargo install recl
```

#### [Osoy](https://github.com/osoy/osoy)

```
osoy c rasmusmerzin/recl
```
