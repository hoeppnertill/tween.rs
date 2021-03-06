extern crate tween;

use tween::{ yoyo, rep, seq, exec, pause };
use std::io::stdio::println;

fn check() {
	println("Check!");
}

fn main() {
    let mut tween = yoyo(seq(vec![
		exec(check),
        exec(check)
	]));

	while !tween.done() {
		tween.update(0.1);
	}

}
