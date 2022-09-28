#!/bin/bash
BASE=
c=256

echo "["
for id in $(cat animals.txt)
do
	u=$(uuidgen -s -N ${id} --namespace @dns | sed -e 's/-//g')
	a=$(printf '%04x\n' $c)
	echo "{\"id\": \"${id}\", \"devkey\": \"${u}\", \"address\": \"${a}\"},"
	c=$((c + 10))
done
echo "]"
