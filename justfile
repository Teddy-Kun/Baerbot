build-all:
	cargo b -r -p shared
	cargo b -r --bins

build-lib:
	cargo b -r -p shared
