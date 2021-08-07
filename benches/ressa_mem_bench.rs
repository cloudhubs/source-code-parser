use std::{
    fs::{File, OpenOptions},
    ops::RemAssign,
    path::Path,
};

use pprof::criterion::{Output, PProfProfiler};

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

fn prune_callbacks(np: &mut NodePattern) {
    np.callback = None;
    np.subpatterns
        .iter_mut()
        .for_each(|sub| prune_callbacks(sub));
}

fn ressa_benchmark(c: &mut Criterion, name: &str, ressa_json: &str) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();
    let guard = pprof::ProfilerGuard::new(100).unwrap();

    let dir = serde_json::from_str::<Directory>(&*directory_json_dsb()).unwrap();
    let ctx = parse_project_context(&dir).unwrap();
    let mut msds = serde_json::from_str::<Vec<NodePattern>>(ressa_json).unwrap();
    msds.iter_mut().for_each(|msd| prune_callbacks(msd));

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
}

fn ressa_benchmark_endpoint_simple_dsb(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_deathstarbench_simple",
        ressa_json_endpoint_simple_dsb,
    )
}

fn ressa_benchmark_endpoint_dsb(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_deathstarbench_call_graph",
        ressa_json_endpoint_dsb,
    )
}

fn ressa_benchmark_entity_dsb(c: &mut Criterion) {
    ressa_benchmark(c, "ressa_entity_deathstarbench", ressa_json_entity_dsb)
}

fn ressa_benchmark_endpoint_tt(c: &mut Criterion) {
    ressa_benchmark(c, "ressa_endpoint_trainticket", ressa_json_endpoint_tt)
}

fn ressa_benchmark_entity_tt(c: &mut Criterion) {
    ressa_benchmark(c, "reessa_entity_trainticket", ressa_json_entity_tt)
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
