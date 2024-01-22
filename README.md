# TFM Marlin

## Overview

Marlin benchmarks using different elliptic curves and circuits. 

## Rust program

Compile the program:

```bash
make 
```
The result is in `target/release`. Get help about the program:
```bash
./target/release/tfm-marlin -h
``````

## Execute the benchmarks

```bash
make tests
```
The results are stored in the `logs` folder and the CSVs in the `csv` folder.

## Delete the data

```bash
make clear
```
