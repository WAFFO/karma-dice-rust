# karma-dice-rust
Command line implementation of karma-dice.  
Module returns a JSON string or a tuple of (rolls: Vec<u32>, sum: i32, karma: f64).

`[nubmer of rolls]d<faces>[+|-<constant>] [karma]`

`cargo run 1d20+3`  
`cargo run 1d12`  
`cargo run d100`  

`cargo run 1d20+3 0.5`  
`cargo run 1d12 -1.2`  
`cargo run d100 4.34`  

Example:  
`$ ./karma_dice_rust 2d20 -0.5`  
 `{ "rolls": [ 6, 8 ], "addition": 0, "sum": 14, "karma": 0.026315789473684292 }`
