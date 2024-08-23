PKG_NAME=dumpling

all: build

build:
	cargo build --release
install: build
	sudo cp target/release/$(PKG_NAME) /usr/bin
uninstall:
	sudo rm -f usr/bin/$(PKG_NAME)
clean:
	cargo clean
.PHONY: all build install uninstall clean
