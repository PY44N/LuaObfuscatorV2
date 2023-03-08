install:
	cd minifier && npm i

build: install
	# cd minifier && npm run build
	cargo build

run:
	cargo run

all: build