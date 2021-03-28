TARGET_LINUX:=x86_64-unknown-linux-gnu
TARGET_WIN:=x86_64-pc-windows-gnu
VERSION:=$(shell grep version Cargo.toml | sed -e 's/.* = "\(.*\)"/\1/' | head -n1)
BIN_NAME:=$(shell grep name Cargo.toml | sed -e 's/.* = "\(.*\)"/\1/' | head -n1)
BIN_WIN:=target/${TARGET_WIN}/release/${BIN_NAME}.exe
BIN_LINUX:=target/${TARGET_LINUX}/release/${BIN_NAME}
SRC=$(wildcard **/*.rs)

.PHONY: build build_linux build_windows

build: build_linux build_windows
	echo ${SRC}

build_linux: ${SRC}
	@cargo build --target=${TARGET_LINUX}

build_linux_release: ${SRC}
	@cargo build --target=${TARGET_LINUX} --release

build_windows: ${SRC}
	@cargo build --target=${TARGET_WIN}

build_windows_release: ${SRC}
	@cargo build --target=${TARGET_WIN} --release

check: ${SRC}
	@cargo check

install: ${SRC}
	@cargo install --path . --force

release:
	@gh release create "v${VERSION}" --title "Release ${VERSION}" ${BIN_WIN} ${BIN_LINUX}
	@git fetch --tags origin

# end
