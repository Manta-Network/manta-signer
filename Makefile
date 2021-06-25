LD_FLAGS := "-r ./lib"

build-mac:
	cd lib/hello && cargo build --release --target=x86_64-apple-darwin
	cp lib/hello/target/release/libhello.dylib lib/
	GOOS=darwin GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/darwin/manta-daemon main.go

build-linux:
	cd lib/hello && cargo build --release --target=x86_64-unknown-linux-gnu
	cd lib/hello/target/release/libhello.so lib/
	GOOS=linux GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/linux/manta-daemon main.go

build-windows:
	cd lib/hello && cargo build -Zbuild-std --release --target=x86_64-pc-windows-gnu
	cd lib/hello/target/release/libhello.so lib/
	GOOS=windows GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/windows/manta-daemon.exe main.go

run-mac: build-mac
	./dist/darwin/manta-daemon