#!/bin/bash
BASE=
c=256

echo "["
for id in $(cat animals.txt)
do
	u=$(uuidgen -s -N ${id} --namespace @dns | sed -e 's/-//g')
	a=$(printf '%04x\n' $c)
	left=$(printf '%04x\n' $((c + 1)))
	right=$(printf '%04x\n' $((c + 2)))
	echo "{\"id\": \"${id}\", \"devkey\": \"${u}\", \"address\": \"${a}\", \"left\": \"${left}\", \"right\": \"${right}\"},"
	c=$((c + 10))
done
echo "]"
