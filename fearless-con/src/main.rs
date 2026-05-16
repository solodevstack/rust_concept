//, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

//Race conditions, in which threads are accessing data or resources in an inconsistent order
//Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
//Bugs that only happen in certain situations and are hard to reproduce and fix reliably


use std::thread;
use std::time::Duration;
// use crate::join::join_main;
mod join;
mod move_closures;

fn main() {
   move_closures::move_main();
}
