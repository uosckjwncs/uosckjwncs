# Uosckjwncs

uosckjwncs - random name for an anonymous project

## Description

The description of `uosckjwncs` has been redacted to make it harder for others
to search for on GitHub!

## Design Decisions

### Sanitising Input

Although the requirements state certain assumptions can be made about the
input, I always try to err on the side of caution by untainting input
(especially from user input or from other untrusted sources) e.g:

  - Explicitly truncate floating-points to 4 decimal places **without**
    rounding
  
  - Treat transaction IDs on a first come first serve basis. Once a transaction
    has been seen, we forever tie that transaction ID to the initially seen
    client ID. This should help prevent stealing by a customer disputing
    **another** customer's transactions.

### Global Dispute Data

As not every customer will have disputes, I found it wasteful to have each
customer hold its own dispute `HashSet`, so instead they are collected within
`main()`.

### Floating-Point Data and Operations

To prevent possible `IEEE754` bugs, we use `rust_decimal` for **all**
floating-point data and operations. On output, we hard-truncate rather than
round (in order to prevent rounding bugs) to 4 decimal places as per
requirements. 

Another handy thing `rust_decimal` gives us is underflow/overflow saturating
operations. As it wasn't stated in the requirements what to do in case of
underflow/overflow, I went with saturation rather than `panic!()` or skipping
transactions entirely.

### Missing Transaction IDs

It may seem weird that `struct Transaction` does not include the transaction
type, but as they were only refered to once in `main()`, I decided to save
space by throwing this data away.

### Library Documentation

There is no documentation within the library files themselves as I assumed this
`README.md` was the only documentation required. If this was a mistaken
assumption, let me know and I will add them ASAP.

### Tests

No test input files have been included as per requirements.

## Author

uosckjwncs@ - email address has been randomly generated to provide anonymity

## Warranty

Given this is a public repository, this sadly has to be stated:

IT COMES WITHOUT WARRANTY OF ANY KIND.
