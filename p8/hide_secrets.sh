#!/bin/sh

i=1

while IFS='' read -r LINE || [ -n "${LINE}" ]; do
    python encoder.py imgs/$i.bmp $LINE
    i=$((i+1))
done < secrets.txt
