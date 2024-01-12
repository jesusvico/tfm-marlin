.PHONY: tests

project := tfm-marlin
program := target/release/$(project)

all: $(program) tests

$(program):
	cargo build --release --features print-trace

tests:
	mkdir -p logs
	mkdir -p csv

clear:
	rm -rf target logs csv