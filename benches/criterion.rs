use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod test_constants;
use test_constants::*;

extern crate source_code_parser;
use source_code_parser::{
    ressa::{run_ressa_parse, NodePattern},
    *,
};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn laast_benchmark(c: &mut Criterion, name: &str, dir: &Directory) {
    let parsed_trees = parse_directory_trees(dir).unwrap();
    c.bench_function(name, |b| {
        b.iter(|| {
            let _modules = black_box(convert_trees_to_laast(parsed_trees.clone()));
        })
    });
}

fn ast_benchmark<'a, T, F>(c: &mut Criterion, name: &str, dir: &'a Directory, f: F)
where
    T: 'a,
    F: Fn(&'a Directory) -> T,
{
    c.bench_function(name, |b| {
        b.iter(|| {
            let _ctx = black_box(f(dir));
        })
    });
}

fn ressa_benchmark(c: &mut Criterion, name: &str, ressa_json: &str, dir: &str) {
    let dir = serde_json::from_str::<Directory>(dir).unwrap();
    let ctx = parse_project_context(&dir).unwrap();
    let ressa = serde_json::from_str::<Vec<NodePattern>>(ressa_json).unwrap();
    c.bench_function(name, |b| {
        b.iter(|| {
            let _ctx = black_box(run_ressa_parse(&mut ctx.modules.clone(), ressa.clone()));
        })
    });
}

fn ressa_benchmark_endpoint_simple(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_deathstarbench_simple",
        RESSA_JSON_ENDPOINT_SIMPLE_DSB,
        &*directory_json_dsb(),
    )
}

fn ressa_benchmark_endpoint(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_deathstarbench_call_graph",
        RESSA_JSON_ENDPOINT_DSB,
        &*directory_json_dsb(),
    )
}

fn ressa_benchmark_entity(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_entity_deathstarbench",
        RESSA_JSON_ENTITY_DSB,
        &*directory_json_dsb(),
    )
}

fn ressa_benchmark_endpoint_tt(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_trainticket",
        RESSA_JSON_ENDPOINT_TT,
        &*directory_json_tt(),
    )
}

fn ressa_benchmark_entity_tt(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_entity_trainticket",
        RESSA_JSON_ENTITY_TT,
        &*directory_json_tt(),
    )
}

fn laast_benchmark_dsb(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_dsb().as_str()).unwrap();
    laast_benchmark(c, "laast_deathstarbench", &dir)
}

fn laast_benchmark_tt(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_tt().as_str()).unwrap();
    laast_benchmark(c, "laast_trainticket", &dir)
}

fn treesitter_ast_benchmark_dsb(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_dsb().as_str()).unwrap();
    ast_benchmark(
        c,
        "treesitter_ast_deathstarbench",
        &dir,
        parse_directory_trees,
    )
}

fn treesitter_ast_benchmark_tt(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_tt().as_str()).unwrap();
    ast_benchmark(c, "treesitter_ast_trainticket", &dir, parse_directory_trees)
}

fn laast_total_benchmark_dsb(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_dsb().as_str()).unwrap();
    ast_benchmark(c, "laast_total_deathstarbench", &dir, parse_project_context)
}

fn laast_total_benchmark_tt(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_tt().as_str()).unwrap();
    ast_benchmark(c, "laast_total_trainticket", &dir, parse_project_context)
}

criterion_group!(
    benches,
    laast_benchmark_dsb,
    treesitter_ast_benchmark_dsb,
    laast_total_benchmark_dsb,
    laast_benchmark_tt,
    treesitter_ast_benchmark_tt,
    laast_total_benchmark_tt,
    ressa_benchmark_endpoint_simple,
    ressa_benchmark_endpoint,
    ressa_benchmark_entity,
    ressa_benchmark_endpoint_tt,
    ressa_benchmark_entity_tt
);
criterion_main!(benches);
