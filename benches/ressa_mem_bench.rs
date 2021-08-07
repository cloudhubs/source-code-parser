use std::{fs::File, ops::RemAssign};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use statistical::*;

mod test_constants;
use test_constants::*;

extern crate source_code_parser;
use source_code_parser::{
    msd::{run_msd_parse, NodePattern},
    *,
};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn ressa_benchmark(c: &mut Criterion, name: &str, ressa_json: &str) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();
    let guard = pprof::ProfilerGuard::new(100).unwrap();

    let dir = serde_json::from_str::<Directory>(&*directory_json_dsb()).unwrap();
    let ctx = parse_project_context(&dir).unwrap();
    let msds = serde_json::from_str::<Vec<NodePattern>>(ressa_json).unwrap();
    let mut mem = vec![];
    c.bench_function(name, |b| {
        epoch.advance().unwrap();
        let before = allocated.read().unwrap();
        b.iter(|| {
            let ctx = run_msd_parse(&mut ctx.modules.clone(), msds.clone());
            black_box(ctx);
            epoch.advance().unwrap();
            let after = allocated.read().unwrap();
            println!("{} - {}", after, before);
            mem.push((after - before) as f64);
        })
    });
    let mean = mean(&mem);
    println!(
        "{} +/- {} ({})",
        mean,
        standard_deviation(&mem, Some(mean)),
        median(&mem)
    );

    if let Ok(report) = guard.report().build() {
        let file = File::create(format!("{}.flamegraph.svg", ressa_json)).unwrap();
        report.flamegraph(file).unwrap();
    };
}

fn ressa_benchmark_endpoint_simple_dsb(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ReSSA Endpoint (DeathStarBench Simple)",
        ressa_json_endpoint_simple_dsb,
    )
}

fn ressa_benchmark_endpoint_dsb(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ReSSA Endpoint (DeathStarBench Call Graph)",
        ressa_json_endpoint_dsb,
    )
}

fn ressa_benchmark_entity_dsb(c: &mut Criterion) {
    ressa_benchmark(c, "ReSSA Entity (DeathStarBench)", ressa_json_entity_dsb)
}

fn ressa_benchmark_endpoint_tt(c: &mut Criterion) {
    ressa_benchmark(c, "ReSSA Endpoint (TrainTicket)", ressa_json_endpoint_tt)
}

fn ressa_benchmark_entity_tt(c: &mut Criterion) {
    ressa_benchmark(c, "ReSSA Entity (TrainTicket)", ressa_json_entity_tt)
}

criterion_group!(
    benches,
    ressa_benchmark_endpoint_simple_dsb,
    ressa_benchmark_endpoint_dsb,
    ressa_benchmark_entity_dsb,
    ressa_benchmark_endpoint_tt,
    ressa_benchmark_entity_tt
);
criterion_main!(benches);
