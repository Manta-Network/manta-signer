build-mac-arm:
	cd lib/zkp && cargo +stable build --release --target=aarch64-apple-darwin
	cp lib/zkp/target/aarch64-apple-darwin/release/libzkp.a lib
	GOOS=darwin GOARCH=arm64 wails build -o lstaticdarwin.go main.go app.go server.go service.go incoming_urls.go

build-mac-x86:
	cd lib/zkp && cargo +stable build --release --target=x86_64-apple-darwin
	cp lib/zkp/target/x86_64-apple-darwin/release/libzkp.a lib
	GOOS=darwin GOARCH=amd64 ~/go/bin/wails build -o dist/darwin/manta-signer lstaticdarwin.go main.go app.go server.go service.go incoming_urls.go

build-linux:
	mkdir -p lib/linux
	cd lib/zkp && cargo build --release --target=x86_64-unknown-linux-musl
	cp lib/zkp/target/x86_64-unknown-linux-gnu/release/libzkp.a lib/linux
	GOOS=linux GOARCH=amd64 CGO_ENABLED=1 CC=x86_64-linux-musl-gcc  CXX=x86_64-linux-musl-g++ go build -o dist/linux/manta-signer main.go app.go server.go service.go incoming_urls.go


build-windows:
	source ~/.bash_profile
	mkdir -p lib/windows
	cd lib/zkp && cargo build --release --target=x86_64-pc-windows-gnu
	cp lib/zkp/target/x86_64-pc-windows-gnu/release/libzkp.a lib/windows/
	#GOOS=windows GOARCH=amd64 go build -ldflags=$(LD_FLAGS) -o dist/windows/manta-signer.exe main.go
	GOOS=windows GOARCH=amd64 CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++ \
	CGO_ENABLED=1 go build -o dist/windows/manta-daemon.exe lstaticwindows.go main.go

run-mac: build-mac
	./dist/darwin/manta-daemon

mac-bundle:
	cp dist/darwin/manta-daemon ./resources/manta-daemon
	./dist/darwin/bundler -assets ./resources -bin manta-daemon -icon resources/icon.png -name manta-daemon
