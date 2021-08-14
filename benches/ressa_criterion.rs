use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use statistical::*;

mod test_constants;
use test_constants::*;

extern crate source_code_parser;
use source_code_parser::{
    msd::{run_msd_parse, Executor, NodePattern, ParserContext},
    *,
};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn laast_benchmark(c: &mut Criterion, name: &str, dir: &str) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    let dir = serde_json::from_str::<Directory>(dir).unwrap();
    let mut mem = vec![];
    c.bench_function(name, |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();
            let _ctx = black_box(parse_project_context(&dir)).unwrap();
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

fn ressa_benchmark(c: &mut Criterion, name: &str, ressa_json: &str) {
    let dir = serde_json::from_str::<Directory>(&*directory_json_dsb()).unwrap();
    let ctx = parse_project_context(&dir).unwrap();
    let ressa = serde_json::from_str::<Vec<NodePattern>>(ressa_json).unwrap();
    c.bench_function(name, |b| {
        b.iter(|| {
            let _ctx = black_box(run_msd_parse(&mut ctx.modules.clone(), ressa.clone()));
        })
    });
}

fn ressa_benchmark_endpoint_simple(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_deathstarbench_simple",
        ressa_json_endpoint_simple_dsb,
    )
}

fn ressa_benchmark_endpoint(c: &mut Criterion) {
    ressa_benchmark(
        c,
        "ressa_endpoint_deathstarbench_call_graph",
        ressa_json_endpoint_dsb,
    )
}

fn ressa_benchmark_entity(c: &mut Criterion) {
    ressa_benchmark(c, "ressa_entity_deathstarbench", ressa_json_entity_dsb)
}

fn ressa_benchmark_endpoint_tt(c: &mut Criterion) {
    ressa_benchmark(c, "ressa_endpoint_trainticket", ressa_json_endpoint_tt)
}

fn ressa_benchmark_entity_tt(c: &mut Criterion) {
    ressa_benchmark(c, "ressa_entity_trainticket", ressa_json_entity_tt)
}

fn laast_benchmark_dsb(c: &mut Criterion) {
    laast_benchmark(c, "laast_deathstarbench", &*directory_json_dsb())
}

fn laast_benchmark_tt(c: &mut Criterion) {
    laast_benchmark(c, "laast_trainticket", &*directory_json_tt())
}

// fn rune_benchmark(c: &mut Criterion) {
//     let epoch = jemalloc_ctl::epoch::mib().unwrap();
//     let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

//     let ressa = serde_json::from_str::<Vec<NodePattern>>(ressa_json_endpoint_simple).unwrap();
//     let msd = ressa
//         .get(0)
//         .unwrap()
//         .clone()
//         .subpatterns
//         .get(0)
//         .unwrap()
//         .clone();
//     let ex = Executor::new().unwrap();
//     let mut mem = vec![];
//     c.bench_function("Rune", |b| {
//         b.iter(|| {
//             epoch.advance().unwrap();
//             let before = allocated.read().unwrap();

//             let _ctx = black_box(ex.execute(&msd, ParserContext::default()));
//             epoch.advance().unwrap();
//             let after = allocated.read().unwrap();
//             println!("{} - {}", after, before);
//             mem.push((after - before) as f64);
//         })
//     });
//     let mean = mean(&mem);
//     println!(
//         "{} +/- {} ({})",
//         mean,
//         standard_deviation(&mem, Some(mean)),
//         median(&mem)
//     );
// }

// criterion_group!(
//     benches,
//     // laast_benchmark,
//     // ressa_benchmark_endpoint_simple,
//     // ressa_benchmark_endpoint,
//     // ressa_benchmark_entity
//     rune_benchmark
// );
criterion_group!(
    benches,
    laast_benchmark_dsb,
    laast_benchmark_tt,
    ressa_benchmark_endpoint_simple,
    ressa_benchmark_endpoint,
    ressa_benchmark_entity,
    ressa_benchmark_endpoint_tt,
    ressa_benchmark_entity_tt
);
criterion_main!(benches);
