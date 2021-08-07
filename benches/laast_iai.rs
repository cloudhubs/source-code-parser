use std::collections::HashMap;

use iai::{black_box, main};

mod test_constants;
use test_constants::*;

extern crate source_code_parser;
use source_code_parser::{
    msd::{run_msd_parse, Executor, NodePattern, ParserContext},
    *,
};

fn laast_benchmark(dir: &str) {
    let dir = serde_json::from_str::<Directory>(&*directory_json_dsb()).unwrap();
    let _ctx = black_box(parse_project_context(&dir)).unwrap();
}

fn ressa_benchmark(ressa_json: &str) {
    let dir = serde_json::from_str::<Directory>(&*directory_json_dsb()).unwrap();
    let ctx = parse_project_context(&dir).unwrap();
    let ressa = serde_json::from_str::<Vec<NodePattern>>(ressa_json).unwrap();
    let _ctx = black_box(run_msd_parse(&mut ctx.modules.clone(), ressa.clone()));
}

fn ressa_benchmark_endpoint_simple() {
    ressa_benchmark(ressa_json_endpoint_simple_dsb)
}

fn ressa_benchmark_endpoint() {
    ressa_benchmark(ressa_json_endpoint_dsb)
}

fn ressa_benchmark_entity() {
    ressa_benchmark(ressa_json_entity_dsb)
}

fn ressa_benchmark_endpoint_tt() {
    ressa_benchmark(ressa_json_endpoint_tt)
}

fn ressa_benchmark_entity_tt() {
    ressa_benchmark(ressa_json_entity_tt)
}

fn laast_benchmark_dsb() {
    laast_benchmark(&*directory_json_dsb())
}

fn laast_benchmark_tt() {
    laast_benchmark(&*directory_json_tt())
}

iai::main!(
    laast_benchmark_dsb,
    laast_benchmark_tt,
    ressa_benchmark_endpoint_simple,
    ressa_benchmark_endpoint,
    ressa_benchmark_entity,
    ressa_benchmark_endpoint_tt,
    ressa_benchmark_entity_tt
);
