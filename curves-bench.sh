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

# 1: Perform the benchmarks
for curve in "${curves[@]}"; do
    echo "Executing $system over $curve curve"
    for r in "${rounds[@]}"; do
        echo -e "   Rounds $r"
        rm -f logs/$system-$curve-$r.txt
        target/release/tfm-marlin -s $system -c $curve -r $r >> logs/$system-$curve-$r.txt
    done
done

# 2: Indexer time
echo "Generating indexer time CSV"
rm -f csv/curves-indexer-time.csv
touch csv/curves-indexer-time.csv
echo -n "constraints" >> csv/curves-indexer-time.csv
for curve in "${curves[@]}"; do
    echo -n ", $curve" >> csv/curves-indexer-time.csv
done
echo "" >> csv/curves-indexer-time.csv

for r in "${rounds[@]}"; do
    constraints=$(sed -n 's/Info: Constraints: \(.*\)/\1/p' logs/$system-$curve-$r.txt)
    echo -n "$constraints" >> csv/curves-indexer-time.csv
    for curve in "${curves[@]}"; do
        time=$(sed -n 's/Info: Indexer time: \(.*\)/\1/p' logs/$system-$curve-$r.txt)
        time="${time%?}"
        echo -n ", $time" >> csv/curves-indexer-time.csv
    done
    echo "" >> csv/curves-indexer-time.csv
done

# 3: Prover time
echo "Generating prover time CSV"
rm -f csv/curves-prover-time.csv
touch csv/curves-prover-time.csv
echo -n "constraints" >> csv/curves-prover-time.csv
for curve in "${curves[@]}"; do
    echo -n ", $curve" >> csv/curves-prover-time.csv
done
echo "" >> csv/curves-prover-time.csv

for r in "${rounds[@]}"; do
    constraints=$(sed -n 's/Info: Constraints: \(.*\)/\1/p' logs/$system-$curve-$r.txt)
    echo -n "$constraints" >> csv/curves-prover-time.csv
    for curve in "${curves[@]}"; do
        time=$(sed -n 's/Info: Prover time: \(.*\)/\1/p' logs/$system-$curve-$r.txt)
        time="${time%?}"
        echo -n ", $time" >> csv/curves-prover-time.csv
    done
    echo "" >> csv/curves-prover-time.csv
done

# 4: Verifier time
echo "Generating verifier time CSV"
rm -f csv/curves-verifier-time.csv
touch csv/curves-verifier-time.csv
echo -n "constraints" >> csv/curves-verifier-time.csv
for curve in "${curves[@]}"; do
    echo -n ", $curve" >> csv/curves-verifier-time.csv
done
echo "" >> csv/curves-verifier-time.csv

for r in "${rounds[@]}"; do
    constraints=$(sed -n 's/Info: Constraints: \(.*\)/\1/p' logs/$system-$curve-$r.txt)
    echo -n "$constraints" >> csv/curves-verifier-time.csv
    for curve in "${curves[@]}"; do
        time=$(sed -n 's/Info: Verifier time: \(.*\)/\1/p' logs/$system-$curve-$r.txt)
        time="${time%?}"
        echo -n ", $time" >> csv/curves-verifier-time.csv
    done
    echo "" >> csv/curves-verifier-time.csv
done