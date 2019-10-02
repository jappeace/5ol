build: 
	date
	cargo build

# TODO make work with watch, maybe builtfarm
clean:
	cargo clean 

watch:
	nix-shell --run "scripts/watch.sh"
file-watch: watch

