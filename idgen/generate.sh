#!/bin/bash

echo "["
for id in $(cat animals.txt)
do
	u=$(uuidgen -s -N ${id} --namespace @dns | sed -e 's/-//g')
	echo "{\"id\": \"${id}\", \"uuid\": \"${u}\"}"
done
echo "]"
