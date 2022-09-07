#!/bin/bash

for i in seq 0 50 
do
	adj=$(shuf -n1 adjectives.txt)
	noun=$(shuf -n1 nouns.txt)

	echo "${adj} ${noun}"
done
