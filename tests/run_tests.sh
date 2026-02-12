#!/bin/bash

for i in {1..10}
do
  cmd="../target/release/lef_rparser ispd19_test$i/ispd19_test$i.input.lef ispd19_test$i/ispd19_test$i.input.def";
  ($cmd)
done

for i in $(seq -f "%02g" 1 18)
do
  cmd="../target/release/lef_rparser ibm$i/ibm$i.lef ibm$i/ibm$i.def";
  ($cmd)
done
