#!/bin/bash


start=$1
end=$2
echo "Running benchmarks from block number $start until $end iteration"
for i in $(seq "$start" "$end"); do
  echo "Iteration for block $i in progress..."
  block_number=$i
  echo $block_number >> results.txt  
  BLOCK_NUMBER=$block_number cargo bench -- "Apply blueprint" | grep "time:"| awk -F'[][]' '{split($2, a, " "); print a[3], a[4]}' >> results.txt
  echo "Iteration for block $i done"
done
echo "Benchmarks ran"