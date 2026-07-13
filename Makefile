.PHONY: all clean core comms hacker civilian cli test
all: core comms hacker civilian cli
core: ; cargo build --release -p omega-core
comms: ; cargo build --release -p omega-comms
hacker: ; cargo build --release -p omega-hacker
civilian: ; cargo build --release -p omega-civilian
cli: ; cargo build --release -p omega-cli
test: ; cargo test --all
clean: ; cargo clean
install: all ; cp target/release/omega /usr/local/bin/
help: ; @echo "Targets: all, core, comms, hacker, civilian, cli, test, clean, install"
