# Excel Index Problem

My solution to an interview problem for converting numbers to "Excel" indexes.

## Problem Statement

Given an interger number greater than zero, convert the number to the corresponding Excel
index. For example 1 is converted to A, 3 to C, 27 to AA, and 16384 to XFD.
The program should read numbers from the standard in, one per line, ignoring whitespace,
and print the Excel index to the standard out.

## Algorithm

The problem would appear to be a simple base 10 to base 26 conversion, however the problem
presents difficulty that needs addressing: A = 1 not 0. The typical algorithm for converting
from one base to another is to repeatedly take the modulus of the number using that as the
current digit in order from smallest to greatest. We then take divide the number by the base,
26. By repeatedly dividing by 26 and taking the remainder we can calculate each digit until
we reach 0.

To handle the fact that 1 == A, we subtract one when taking the modulus. And to handle the
subsequent digits we also subtract the modulus from the current number before we divide it
by 26. This has the effect of removing up to 1 unit of 26 from the over all sum ensuring the
remaining digits are correct. For example 26 because of how Excel counts where there are no
zeros we have:

```
0: (26 - 1) % 26 = 25
0: (26 - 25) / 26 = 0
```

giving our answer of Z.

Another example 704:

```
0: (704 - 1) % 26 = 1  -> B
0: (704 - 1) / 26 = 27
1:  (27 - 1) % 26 = 0  -> A
1:  (27 - 0) / 26 = 1
2:   (1 - 1) % 26 = 0  -> A
2:   (1 - 0) / 26 = 0
```

giving our answer AAB.

## Background and Discussion

This question was given to me during an interview. As someone who as interviewed many people
from technical, programming positions, I would not use this question if I intended to see
a working solution. The problem is deceptively difficult and took me far longer to figure out
on my own than I would like to admit. Although it was a fun exercise, it's suitablity for
determining if an individual can be successful in a job involving programming is questionable.

## Building

My solution is done using rust and cargo. Both can be installed using [rustup](https://rustup.rs).

## Usage

```bash
cargo test
```
