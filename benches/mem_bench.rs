use std::fs::OpenOptions;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use statistical::*;

mod test_constants;
use test_constants::*;

extern crate source_code_parser;
use source_code_parser::{
    ressa::{run_ressa_parse, NodePattern},
    *,
};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn abs_diff(x: usize, y: usize) -> usize {
    if y < x {
        // println!("{} - {} = {}", x, y, x - y);
        x - y
    } else {
        // println!("{} - {} = {}", y, x, y - x);
        y - x
    }
}

fn laast_benchmark(c: &mut Criterion, name: &str, dir: &Directory) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    let parsed_trees = parse_directory_trees(dir).unwrap();

    let mut mem = vec![];
    c.bench_function(name, |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();
            let _modules = black_box(convert_trees_to_laast(parsed_trees.clone()));
            epoch.advance().unwrap();
            mem.push(abs_diff(allocated.read().unwrap(), before) as f64);
        })
    });
    let mean = mean(&mem);
    println!(
        "{} +/- {} ({})",
        mean,
        standard_deviation(&mem, Some(mean)),
        median(&mem)
    );
}

fn ast_benchmark<'a, T, F>(c: &mut Criterion, name: &str, dir: &'a Directory, f: F)
where
    T: 'a,
    F: Fn(&'a Directory) -> T,
{
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    let mut mem = vec![];
    c.bench_function(name, |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();
            let _ctx = black_box(f(dir));
            epoch.advance().unwrap();
            mem.push(abs_diff(allocated.read().unwrap(), before) as f64);
        })
    });
    let mean = mean(&mem);
    println!(
        "{} +/- {} ({})",
        mean,
        standard_deviation(&mem, Some(mean)),
        median(&mem)
    );
}

fn ressa_benchmark(c: &mut Criterion, name: &str, ressa_json: &str, dir: &str) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    let dir = serde_json::from_str::<Directory>(dir).unwrap();
    let ctx = parse_project_context(&dir).unwrap();
    let ressas = serde_json::from_str::<Vec<NodePattern>>(ressa_json).unwrap();

    let mut mem = vec![];

    c.bench_function(name, |b| {
        let guard = pprof::ProfilerGuard::new(100).unwrap();

        black_box(run_ressa_parse(&mut ctx.modules.clone(), ressas.clone()));

        match guard.report().build() {
            Ok(report) => {
                let p = format!(
                    "/home/jacob/dev/rust/source-code-parser/target/criterion/{}/flamegraph.svg",
                    name
                );
                let file = match OpenOptions::new()
                    .truncate(true)
                    .create(true)
                    .write(true)
                    .open(&*p)
                {
                    Ok(file) => file,
                    Err(err) => panic!("{:?} - {}", err, p),
                };
                report.flamegraph(file).unwrap();
            }
            Err(err) => {
                panic!("flamegraph - {:?}", err);
            }
        };

        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();
            let ctx = run_ressa_parse(&mut ctx.modules.clone(), ressas.clone());
            black_box(ctx);
            epoch.advance().unwrap();
            let after = allocated.read().unwrap();
            mem.push(abs_diff(after, before) as f64);
        })
    });
    let mean = mean(&mem);
    println!(
        "{} +/- {} ({})",
        mean,
        standard_deviation(&mem, Some(mean)),
        median(&mem)
    );
}

fn ressa_benchmark_endpoint_simple_dsb(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_deathstarbench_simple_mem",
        RESSA_JSON_ENDPOINT_SIMPLE_DSB,
        &*directory_json_dsb(),
    )
}

fn ressa_benchmark_endpoint_dsb(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_deathstarbench_call_graph_mem",
        RESSA_JSON_ENDPOINT_DSB,
        &*directory_json_dsb(),
    )
}

fn ressa_benchmark_entity_dsb(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_entity_deathstarbench_mem",
        RESSA_JSON_ENTITY_DSB,
        &*directory_json_dsb(),
    )
}

fn ressa_benchmark_endpoint_tt(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_trainticket_mem",
        RESSA_JSON_ENDPOINT_TT,
        &*directory_json_tt(),
    )
}

fn ressa_benchmark_entity_tt(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_entity_trainticket_mem",
        RESSA_JSON_ENTITY_TT,
        &*directory_json_tt(),
    )
}

fn laast_benchmark_dsb(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_dsb().as_str()).unwrap();
    laast_benchmark(c, "laast_deathstarbench_mem", &dir)
}

fn laast_benchmark_tt(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_tt().as_str()).unwrap();
    laast_benchmark(c, "laast_trainticket_mem", &dir)
}

fn treesitter_ast_benchmark_dsb(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_dsb().as_str()).unwrap();
    ast_benchmark(
        c,
        "treesitter_ast_deathstarbench_mem",
        &dir,
        parse_directory_trees,
    )
}

fn treesitter_ast_benchmark_tt(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_tt().as_str()).unwrap();
    ast_benchmark(
        c,
        "treesitter_ast_trainticket_mem",
        &dir,
        parse_directory_trees,
    )
}

fn laast_total_benchmark_dsb(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_dsb().as_str()).unwrap();
    ast_benchmark(
        c,
        "laast_total_deathstarbench_mem",
        &dir,
        parse_project_context,
    )
}

fn laast_total_benchmark_tt(c: &mut Criterion) {
    let dir = serde_json::from_str::<Directory>(directory_json_tt().as_str()).unwrap();
    ast_benchmark(
        c,
        "laast_total_trainticket_mem",
        &dir,
        parse_project_context,
    )
}

criterion_group!(
    benches,
    laast_benchmark_dsb,
    treesitter_ast_benchmark_dsb,
    laast_total_benchmark_dsb,
    laast_benchmark_tt,
    treesitter_ast_benchmark_tt,
    laast_total_benchmark_tt,
    ressa_benchmark_endpoint_simple_dsb,
    ressa_benchmark_endpoint_dsb,
    ressa_benchmark_entity_dsb,
    ressa_benchmark_endpoint_tt,
    ressa_benchmark_entity_tt
);
criterion_main!(benches);
