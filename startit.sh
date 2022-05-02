kill -9 $(ps -ef | grep test_server | awk '{print $2}')

cd js/e2e/simulation
cargo run 10 | tail -n +2 > out.json

cd ../../../
./target/release/examples/test_server 127.0.0.1:29988 &
./target/release/examples/test_server 127.0.0.1:29989 &
./target/release/examples/test_server 127.0.0.1:29990 &
