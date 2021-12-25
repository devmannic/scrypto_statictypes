#!/usr/bin/env bash
set -e
set -x

for example in examples/*; do
    (cd $example;
        if [ -e ./test.sh ]; then
            ./test.sh
        else
            scrypto test
        fi
    )
done
