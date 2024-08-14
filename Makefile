all: build

build:
	cargo build --release
install: build
	sudo cp target/release/dumpling /usr/bin
uninstall:
	sudo rm -f usr/bin/dumpling
clean:
	cargo clean
.PHONY: all build install uninstall clean
