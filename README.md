# karma-dice-rust
Command line implementation of karma-dice.  
Module returns a JSON string.

`[nubmer of rolls]d<faces>[+|-<constant>] [karma]`

`cargo run 1d20+3`  
`cargo run 1d12`  
`cargo run d100`  

`cargo run 1d20+3 0.5`  
`cargo run 1d12 -1.2`  
`cargo run d100 4.34`  
