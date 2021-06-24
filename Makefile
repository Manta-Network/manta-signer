ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

build-mac:
	cd lib/hello && cargo build --release
	cp lib/hello/target/release/libhello.dylib lib/
	go build -ldflags="-r $(ROOT_DIR)lib" -o manta main.go

build-linux:
	cd lib/hello && cargo build --release
	cd lib/hello/target/release/libhello.so lib/
	go build -ldflags="-r $(ROOT_DIR)lib" -o manta main.go

run-mac: build-mac
	./manta