PREFIX ?= /data/data/com.termux/files

build:
	rustc -O -o casex main.rs

install: build
	install -Dm755 casex $(PREFIX)/usr/bin/casex

uninstall:
	rm -f $(PREFIX)/usr/bin/casex
