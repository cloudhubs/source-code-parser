#!/bin/bash

if [[ ! -d target || ! -d target/criterion ]]; then
    mkdir target/criterion
fi

benchmarks="laast_total_trainticket laast_total_deathstarbench treesitter_ast_trainticket treesitter_ast_deathstarbench laast_trainticket laast_deathstarbench ressa_entity_trainticket ressa_endpoint_trainticket ressa_entity_deathstarbench ressa_endpoint_deathstarbench_call_graph ressa_endpoint_deathstarbench_simple"

for benchmark in $benchmarks; do
    mkdir "target/criterion/${benchmark}_mem" && touch "target/criterion/${benchmark}_mem/flamegraph.svg"
done 
