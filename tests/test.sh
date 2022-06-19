#!/bin/bash

for file in ./tests/*; do
    if [[ "$file" == "./tests/test.sh" ]]; then
        continue;
    fi
    cargo run $file

    if [[ $? -ne 0 ]]; then
        echo "$file FAILED"
        exit 1
    fi
done

echo "All done"
