#![feature(set_stdio)]
use console_error_panic_hook;
use wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod stdiowrap;

pub fn init() {
	console_error_panic_hook::set_once();
	std::io::set_panic(Some(Box::new(stdiowrap::StdErr::new())));
	std::io::set_print(Some(Box::new(stdiowrap::StdOut::new())));

	println!("Finished initializing.");
}