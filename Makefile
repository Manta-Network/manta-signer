build-linux:
	mkdir -p lib/linux
	cd lib/zkp && cargo build --release --target=x86_64-unknown-linux-musl
	cp lib/zkp/target/x86_64-unknown-linux-gnu/release/libzkp.a lib/linux
	GOOS=linux GOARCH=amd64 CGO_ENABLED=1 CC=x86_64-linux-musl-gcc  CXX=x86_64-linux-musl-g++ go build -o dist/linux/manta-signer main.go

build-windows:
	mkdir -p lib/windows
	cd lib/zkp && cargo build --release --target=x86_64-pc-windows-gnu
	cp lib/zkp/target/x86_64-pc-windows-gnu/release/libzkp.a lib/windows/
	#GOOS=windows GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/windows/manta-signer.exe main.go
	GOOS=windows GOARCH=amd64 CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++ \
	CGO_ENABLED=1 wails build -platform=windows