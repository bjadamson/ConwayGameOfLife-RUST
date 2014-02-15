all: bin

bin:
	mkdir -p bin
	cp examples/* bin/
	rustc --out-dir=bin src/main.rs

clean:
	rm -rf bin/
