default:
	just --list

run:
    cd js/two_sum && node index.js
    cd rust/two_sum/src && cargo run --release
