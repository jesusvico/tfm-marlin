.PHONY: tests

project := tfm-marlin
program := target/release/$(project)

all: $(program)

$(program):
	cargo build --release --features print-trace

tests: $(program)
	mkdir -p logs
	mkdir -p csv
	./circuits-bench.sh bench
	./curves-bench.sh bench

clear:
	rm -rf logs csv target