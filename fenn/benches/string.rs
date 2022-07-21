use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn string_small(c: &mut Criterion) {
    let mut trim_group = c.benchmark_group("Small - Trim");
    trim_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  "));
            let _new_text = text.trim().to_string();
        })
    });
    trim_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  "));
            text.trim();
        })
    });
    trim_group.finish();

    let mut trim_start_group = c.benchmark_group("Small - Trim Start");
    trim_start_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  "));
            let _new_text = text.trim_start().to_string();
        })
    });
    trim_start_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  "));
            text.trim_start();
        })
    });
    trim_start_group.finish();

    let mut trim_start_matches_group = c.benchmark_group("Small - Trim Start Matches");
    trim_start_matches_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("Hello World"));
            let _new_text = text.trim_start_matches("Hello ").to_string();
        })
    });
    trim_start_matches_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("Hello World"));
            text.trim_start_matches("Hello ");
        })
    });
    trim_start_matches_group.finish();

    let mut trim_end_group = c.benchmark_group("Small - Trim End");
    trim_end_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  "));
            let _new_text = text.trim_end().to_string();
        })
    });
    trim_end_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  "));
            text.trim_end();
        })
    });
    trim_end_group.finish();

    let mut trim_end_matches_group = c.benchmark_group("Small - Trim End Matches");
    trim_end_matches_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("Hello World"));
            let _new_text = text.trim_end_matches("World").to_string();
        })
    });
    trim_end_matches_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("Hello World"));
            text.trim_end_matches("World");
        })
    });
    trim_end_matches_group.finish();
}

fn string_large(c: &mut Criterion) {
    let mut trim_group = c.benchmark_group("Large - Trim");
    trim_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            let _new_text = text.trim().to_string();
        })
    });
    trim_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            text.trim();
        })
    });
    trim_group.finish();

    let mut trim_start_group = c.benchmark_group("Large - Trim Start");
    trim_start_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            let _new_text = text.trim_start().to_string();
        })
    });
    trim_start_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            text.trim_start();
        })
    });
    trim_start_group.finish();

    let mut trim_start_matches_group = c.benchmark_group("Large - Trim Start Matches");
    trim_start_matches_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("Hello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello World"));
            let _new_text = text.trim_start_matches("Hello ").to_string();
        })
    });
    trim_start_matches_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("Hello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello World"));
            text.trim_start_matches("Hello ");
        })
    });
    trim_start_matches_group.finish();

    let mut trim_end_group = c.benchmark_group("Large - Trim End");
    trim_end_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            let _new_text = text.trim_end().to_string();
        })
    });
    trim_end_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            text.trim_end();
        })
    });
    trim_end_group.finish();

    let mut trim_end_matches_group = c.benchmark_group("Large - Trim End Matches");
    trim_end_matches_group.bench_function("String", |b| {
        b.iter(|| {
            let text = black_box(String::from("Hello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello World"));
            let _new_text = text.trim_end_matches("World").to_string();
        })
    });
    trim_end_matches_group.bench_function("StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("Hello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello World"));
            text.trim_end_matches("World");
        })
    });
    trim_end_matches_group.finish();
}

fn string_both(c: &mut Criterion) {
    let mut trim_group = c.benchmark_group("Both - Trim");
    trim_group.bench_function("Small - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  "));
            let _new_text = text.trim().to_string();
        })
    });
    trim_group.bench_function("Small - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  "));
            text.trim();
        })
    });
    trim_group.bench_function("Large - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            let _new_text = text.trim().to_string();
        })
    });
    trim_group.bench_function("Large - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            text.trim();
        })
    });
    trim_group.finish();

    let mut trim_start_group = c.benchmark_group("Both - Trim Start");
    trim_start_group.bench_function("Small - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  "));
            let _new_text = text.trim_start().to_string();
        })
    });
    trim_start_group.bench_function("Small - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  "));
            text.trim_start();
        })
    });
    trim_start_group.bench_function("Large - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            let _new_text = text.trim_start().to_string();
        })
    });
    trim_start_group.bench_function("Large - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            text.trim_start();
        })
    });
    trim_start_group.finish();

    let mut trim_start_matches_group = c.benchmark_group("Both - Trim Start Matches");
    trim_start_matches_group.bench_function("Small - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("Hello World"));
            let _new_text = text.trim_start_matches("Hello ").to_string();
        })
    });
    trim_start_matches_group.bench_function("Small - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("Hello World"));
            text.trim_start_matches("Hello ");
        })
    });
    trim_start_matches_group.bench_function("Large - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("Hello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello World"));
            let _new_text = text.trim_start_matches("Hello ").to_string();
        })
    });
    trim_start_matches_group.bench_function("Large - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("Hello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello World"));
            text.trim_start_matches("Hello ");
        })
    });
    trim_start_matches_group.finish();

    let mut trim_end_group = c.benchmark_group("Both - Trim End");
    trim_end_group.bench_function("Small - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  "));
            let _new_text = text.trim_end().to_string();
        })
    });
    trim_end_group.bench_function("Small - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  "));
            text.trim_end();
        })
    });
    trim_end_group.bench_function("Large - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            let _new_text = text.trim_end().to_string();
        })
    });
    trim_end_group.bench_function("Large - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  Hello World  "));
            text.trim_end();
        })
    });
    trim_end_group.finish();

    let mut trim_end_matches_group = c.benchmark_group("Both - Trim End Matches");
    trim_end_matches_group.bench_function("Small - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("Hello World"));
            let _new_text = text.trim_end_matches("World").to_string();
        })
    });
    trim_end_matches_group.bench_function("Small - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("Hello World"));
            text.trim_end_matches("World");
        })
    });
    trim_end_matches_group.bench_function("Large - String", |b| {
        b.iter(|| {
            let text = black_box(String::from("Hello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello World"));
            let _new_text = text.trim_end_matches("World").to_string();
        })
    });
    trim_end_matches_group.bench_function("Large - StringExt", |b| {
        b.iter(|| {
            use fenn::StringExt;

            let mut text = black_box(String::from("Hello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello WorldHello World"));
            text.trim_end_matches("World");
        })
    });
    trim_end_matches_group.finish();
}

criterion_group!(benches, string_both, string_small, string_large);
criterion_main!(benches);
