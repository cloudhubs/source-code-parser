use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rune::{Diagnostics, Options, Sources};
use runestick::{Context, Module, Source};
use std::sync::Arc;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn rune_bench(source: &str) -> runestick::Result<()> {
    let mut context = Context::default();

    let module = Module::default();
    context.install(&module)?;

    let mut sources = Sources::new();
    sources.insert(Source::new("test", source));

    let mut diagnostics = Diagnostics::new();

    let unit = rune::load_sources(
        &context,
        &Options::default(),
        &mut sources,
        &mut diagnostics,
    )?;

    let vm = runestick::Vm::new(Arc::new(context.runtime()), Arc::new(unit));
    vm.execute(&["main"], ())?.complete()?;

    Ok(())
}

fn rune_benchmark_good(c: &mut Criterion) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    c.bench_function("Rune 1", |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();

            black_box(rune_bench(
                r#"
            struct User {
            }
            
            impl User {
            }
            "#,
            ));

            epoch.advance().unwrap();
            let after = allocated.read().unwrap();
            println!("{} - {}", after, before);
        })
    });
}

fn rune_benchmark_bad(c: &mut Criterion) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    c.bench_function("Rune 2", |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();

            black_box(rune_bench(r#"pub fn main() { }"#));

            epoch.advance().unwrap();
            let after = allocated.read().unwrap();
            println!("{} - {}", after, before);
        })
    });
}

criterion_group!(benches, rune_benchmark_good, rune_benchmark_bad);
criterion_main!(benches);
