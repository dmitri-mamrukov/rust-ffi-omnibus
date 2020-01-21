coverage:
	cargo tarpaulin --verbose

analyze-code:
	cargo clippy

format:
	cargo fmt

build:
	cargo build

test-lib: analyze-code format
	cargo test -- --nocapture --test-threads=1

test-all-clients: test-lib run-c run-python run-js run-csharp

run-c: build
	gcc --std=c11 -o c/client c/client.c -L target/debug/ -lapi
	LD_LIBRARY_PATH=target/debug c/client

run-python: build
	LD_LIBRARY_PATH=target/debug python python/client.py

run-js: build
	LD_LIBRARY_PATH=target/debug node js/client.js

run-csharp: build
	cp target/debug/libapi.so csharp/libapi.so
	cd csharp && dotnet format && dotnet build
	cd csharp && LD_LIBRARY_PATH=. dotnet run

clean:
	cargo clean
