default: run

run:
	#cargo run
	watchexec -r cargo run

release:
	cargo build --release

tidy:
	tidy -i -m assets/index.html

clean: 
	cargo clean

production:
	cargo run --release
	
