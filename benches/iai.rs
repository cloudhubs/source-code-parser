use iai::black_box;

mod test_constants;
use test_constants::*;

extern crate source_code_parser;
use source_code_parser::{
    ressa::{run_ressa_parse, NodePattern},
    *,
};

fn laast_benchmark(dir: &str) {
    let dir = serde_json::from_str::<Directory>(&*dir).unwrap();
    let _ctx = black_box(parse_project_context(&dir)).unwrap();
}

fn ressa_benchmark(ressa_json: &str, dir: &str) {
    let dir = serde_json::from_str::<Directory>(dir).unwrap();
    let mut ctx = parse_project_context(&dir).unwrap();
    let ressa = serde_json::from_str::<Vec<NodePattern>>(ressa_json).unwrap();
    let _ctx = black_box(run_ressa_parse(&mut ctx.modules, ressa));
}

fn ressa_endpoint_deathstarbench_simple() {
    ressa_benchmark(RESSA_JSON_ENDPOINT_SIMPLE_DSB, &*directory_json_dsb())
}

fn ressa_endpoint_deathstarbench_call_graph() {
    ressa_benchmark(RESSA_JSON_ENDPOINT_DSB, &*directory_json_dsb())
}

fn ressa_entity_deathstarbench() {
    ressa_benchmark(RESSA_JSON_ENTITY_DSB, &*directory_json_dsb())
}

fn ressa_benchmark_endpoint_tt() {
    ressa_benchmark(RESSA_JSON_ENDPOINT_TT, &*directory_json_tt())
}

fn ressa_benchmark_entity_tt() {
    ressa_benchmark(RESSA_JSON_ENTITY_TT, &*directory_json_tt())
}

fn laast_deathstarbench() {
    laast_benchmark(&*directory_json_dsb())
}

fn laast_trainticket() {
    laast_benchmark(&*directory_json_tt())
}

iai::main!(
    laast_deathstarbench,
    laast_trainticket,
    ressa_endpoint_deathstarbench_simple,
    ressa_endpoint_deathstarbench_call_graph,
    ressa_entity_deathstarbench,
    ressa_benchmark_endpoint_tt,
    ressa_benchmark_entity_tt
);
