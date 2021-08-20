build-mac-arm:
	cd lib/zkp && cargo build --release --target=aarch64-apple-darwin
	cp lib/zkp/target/aarch64-apple-darwin/release/libzkp.a lib/
	cd prover_key_gen && cargo run --release
	mkdir lib/zkp/keys && mv prover_key_gen/*.bin lib/zkp/keys
	GOOS=darwin GOARCH=arm64 go build -o dist/darwin/manta-daemon lstaticdarwin.go main.go
	go build -o dist/darwin/bundler mac/bundler.go

build-mac-x86:
	cd lib/zkp && cargo build --release --target=x86_64-apple-darwin
	cp lib/zkp/target/x86_64-apple-darwin/release/libzkp.a lib/
	cd prover_key_gen && cargo run --release
	mkdir lib/zkp/keys && mv prover_key_gen/*.bin lib/zkp/keys
	GOOS=darwin GOARCH=amd64 go build -o dist/darwin/manta-daemon lstaticdarwin.go main.go
	go build -o dist/darwin/bundler mac/bundler.go

build-linux:
	cd lib/zkp && cargo build --release --target=x86_64-unknown-linux-musl
	cp lib/zkp/target/x86_64-unknown-linux-gnu/release/libzkp.a lib/
	GOOS=linux GOARCH=amd64 CGO_ENABLED=1 CC=x86_64-linux-musl-gcc  CXX=x86_64-linux-musl-g++ go build -o dist/linux/manta-daemon main.go

build-windows:
	cd lib/zkp && cargo build --release --target=x86_64-pc-windows-gnu
	cp lib/zkp/target/x86_64-pc-windows-gnu/release/libzkp.a lib/
	#GOOS=windows GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/windows/manta-daemon.exe main.go
	GOOS=windows GOARCH=amd64 CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++ \
	CGO_ENABLED=1 go build -o dist/windows/manta-daemon.exe lstaticwindows.go main.go

run-mac: build-mac
	./dist/darwin/manta-daemon

mac-bundle:
	cp dist/darwin/manta-daemon ./resources/manta-daemon
	./dist/darwin/bundler -assets ./resources -bin manta-daemon -icon resources/icon.png -name manta-daemon
