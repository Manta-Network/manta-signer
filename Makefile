LD_FLAGS := "-r ./lib"

build-mac:
	cd lib/zkp && cargo build --release --target=x86_64-apple-darwin
	cp lib/zkp/target/x86_64-apple-darwin/release/libzkp.dylib lib/
	GOOS=darwin GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/darwin/manta-daemon main.go

build-linux:
	cd lib/zkp && cargo build --release --target=x86_64-unknown-linux-gnu
	cd lib/zkp/target/release/libzkp.so lib/
	GOOS=linux GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/linux/manta-daemon main.go

build-windows:
	cd lib/zkp && cargo build -Zbuild-std --release --target=x86_64-pc-windows-gnu
	cd lib/zkp/target/release/libzkp.so lib/
	GOOS=windows GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/windows/manta-daemon.exe main.go

run-mac: build-mac
	./dist/darwin/manta-daemon