#!/bin/bash
ijconsole calculate.j <<< '5235 300 3'
for i in `seq 1 10`; do ijconsole calculate.j <<< "5235 300 $i"; done | sed -e 's/^[[:space:]]*//' | sort -nr -k1 | head -n1
