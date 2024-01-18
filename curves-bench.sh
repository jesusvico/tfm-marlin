#!/bin/bash

system="basic"
curves=("bls12_381" "bls12_377" "mnt4_298" "mnt4_753" "mnt6_298" "mnt6_753")

# Generate the rounds interval
rounds=()
rounds_min=3
rounds_max=12
for ((i = $rounds_min; i <= $rounds_max; i += 1)); do
    rounds+=($((2**$i)))
done

# Perform the benchmarks
if [ "$1" == "bench" ]; then
    for curve in "${curves[@]}"; do
        echo "Executing $system over $curve curve"
        for r in "${rounds[@]}"; do
            echo -e "   Rounds $r"
            rm -f logs/$system-$curve-$r.txt
            target/release/tfm-marlin -s $system -c $curve -r $r >> logs/$system-$curve-$r.txt
        done
    done
fi


# file_name, step_name
generate_csv () {
    echo "Generating $1"
    rm -f "$1"
    touch "$1"
    echo -n "constraints" >> "$1"
    for curve in "${curves[@]}"; do
        echo -n ", $curve" >> "$1"
    done
    echo "" >> "$1"

    for r in "${rounds[@]}"; do
        constraints=$(sed -n 's/Info: Constraints: \(.*\)/\1/p' logs/$system-${curves[0]}-$r.txt)
        echo -n "$constraints" >> "$1"
        for curve in "${curves[@]}"; do
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

# Indexer time
generate_csv "csv/curves-indexer-time.csv" "Marlin::Index"


# Prover time
generate_csv "csv/curves-prover-time.csv" "Marlin::Prove"


# Verifier time
generate_csv "csv/curves-verifier-time.csv" "Marlin::Verify"