#!/bin/bash

curve="bls12_381"
systems=("basic" "product" "addition" "dense" "fibonacci" "sumprod" "sum")

# Generate the rounds interval
rounds=()
rounds_min=5
rounds_max=15
for ((i = $rounds_min; i <= $rounds_max; i += 1)); do
    rounds+=($((2**$i)))
done

# 1: Perform the benchmarks
if [ "$1" == "bench" ]; then
    for system in "${systems[@]}"; do
        echo "Executing $system over $curve curve"
        for r in "${rounds[@]}"; do
            echo -e "   Rounds $r"
            if [ ! -e "logs/$system-$curve-$r.txt" ]; then
                target/release/tfm-marlin -s $system -c $curve -r $r >> logs/$system-$curve-$r.txt
            fi
        done
    done
fi


# file_name, step_name
generate_csv () {
    echo "Generating $1"
    rm -f "$1"
    touch "$1"
    echo -n "constraints" >> "$1"
    for system in "${systems[@]}"; do
        echo -n ", $system" >> "$1"
    done
    echo "" >> "$1"

    for r in "${rounds[@]}"; do
        constraints=$(sed -n 's/Info: Constraints: \(.*\)/\1/p' logs/${systems[0]}-$curve-$r.txt)
        echo -n "$constraints" >> "$1"
        for system in "${systems[@]}"; do
            if [ ! -e "logs/$system-$curve-$r.txt" ]; then
                echo -n ", " >> "$1"
                continue
            fi

            time=$(sed -n "/End.*$2/p" logs/$system-$curve-$r.txt)
            time=$(echo "$time" | grep -oE '[0-9]+(\.[0-9]+)?(s|ms|µs)')

            if [[ $time == *ms ]]; then
                time="${time%??}"
                time=$(echo "scale=6; $time / 1000" | bc)
            elif [[ $time == *µs ]]; then
                time="${time%??}"
                time=$(echo "scale=6; $time / 1000000" | bc)
            else
                time="${time%?}"
            fi


            time=$(echo "$time" | sed 's/[a-z]*$//')
            echo -n ", $time" >> "$1"
        done
        echo "" >> "$1"
    done
}

# 1: Indexer time
generate_csv "csv/circuits-indexer-time.csv" "Marlin::Index"

generate_csv "csv/circuits-indexer-ahp-time.csv" "AHP::Index"
generate_csv "csv/circuits-indexer-commit-time.csv" "Commit to index polynomials"


# 2: Prover time
generate_csv "csv/circuits-prover-time.csv" "Marlin::Prove"

generate_csv "csv/circuits-prover-first-time.csv" "Committing to first round polys"
generate_csv "csv/circuits-prover-second-time.csv" "Committing to second round polys"
generate_csv "csv/circuits-prover-third-time.csv" "Committing to third round polys"


# 3: Verifier time
generate_csv "csv/circuits-verifier-time.csv" "Marlin::Verify"


