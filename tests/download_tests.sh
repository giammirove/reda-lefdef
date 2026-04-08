#!/bin/bash

cd tests

for i in {1..10}
do
  url="https://www.ispd.cc/contests/19/benchmarks/ispd19_test$i.tgz";
  echo $url;
  (wget $url && gunzip ispd19_test$i.tgz && tar -xvf ispd19_test$i.tar) 
  rm -rf ispd19_test$i.tar
done

wget --no-check-certificate "https://vlsicad.eecs.umich.edu/BK/ISPD02bench/ibmISPD02Bench_LEFDEF.tar.gz" && tar -xvf ibmISPD02Bench_LEFDEF.tar.gz

cd ..
