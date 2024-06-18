mod setup;

use self::setup::{
    generate_test_strings,
    BackendBenchmark,
    BenchBucket,
    BenchBuffer,
    BenchString,
    BENCH_LEN_STRINGS,
    BENCH_STRING_LEN,
};
use byte_string_interner::backend::Backend;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    measurement::WallTime,
    BatchSize,
    BenchmarkGroup,
    Criterion,
    Throughput,
};

criterion_group!(
    bench_resolve,
    bench_resolve_already_filled,
    bench_resolve_unchecked_already_filled
);
criterion_group!(bench_get, bench_get_already_filled);
criterion_group!(bench_iter, bench_iter_already_filled);
criterion_group!(
    bench_get_or_intern,
    bench_get_or_intern_fill,
    bench_get_or_intern_fill_with_capacity,
    bench_get_or_intern_already_filled,
    bench_get_or_intern_static,
);
criterion_main!(bench_get_or_intern, bench_resolve, bench_get, bench_iter);

fn bench_get_or_intern_static(c: &mut Criterion) {
    let mut g = c.benchmark_group("get_or_intern_static");
    fn bench_for_backend<BB: BackendBenchmark>(g: &mut BenchmarkGroup<WallTime>) {
        #[rustfmt::skip]
        let static_strings = &[
            b"a", b"b", b"c", b"d", b"e", b"f", b"g", b"h", b"i", b"j", b"k", b"l", b"m",
            b"n", b"o", b"p", b"q", b"r", b"s", b"t", b"u", b"v", b"w", b"x", b"y", b"z",
            b"A", b"B", b"C", b"D", b"E", b"F", b"G", b"H", b"I", b"J", b"K", b"L", b"M",
            b"N", b"O", b"P", b"Q", b"R", b"S", b"T", b"U", b"V", b"W", b"X", b"Y", b"Z",
        ];
        g.throughput(Throughput::Elements(static_strings.len() as u64));
        g.bench_with_input(
            format!("{}/{}", BB::NAME, "get_or_intern"),
            static_strings,
            |bencher, words| {
                bencher.iter_batched_ref(
                    || BB::setup(),
                    |interner| {
                        for word in words.iter().copied() {
                            black_box(interner.get_or_intern(word));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        g.bench_with_input(
            format!("{}/{}", BB::NAME, "get_or_intern_static"),
            static_strings,
            |bencher, words| {
                bencher.iter_batched_ref(
                    || BB::setup(),
                    |interner| {
                        for word in words.iter().copied() {
                            black_box(interner.get_or_intern_static(word));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    bench_for_backend::<BenchBucket>(&mut g);
    bench_for_backend::<BenchString>(&mut g);
    bench_for_backend::<BenchBuffer>(&mut g);
}

fn bench_get_or_intern_fill_with_capacity(c: &mut Criterion) {
    let mut g = c.benchmark_group("get_or_intern/fill-empty/with_capacity");
    g.throughput(Throughput::Elements(BENCH_LEN_STRINGS as u64));
    fn bench_for_backend<BB: BackendBenchmark>(g: &mut BenchmarkGroup<WallTime>) {
        g.bench_with_input(
            BB::NAME,
            &(BENCH_LEN_STRINGS, BENCH_STRING_LEN),
            |bencher, &(len_words, word_len)| {
                let words = generate_test_strings(len_words, word_len);
                bencher.iter_batched_ref(
                    || BB::setup_with_capacity(BENCH_LEN_STRINGS),
                    |interner| {
                        for word in &words {
                            black_box(interner.get_or_intern(word));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    bench_for_backend::<BenchBucket>(&mut g);
    bench_for_backend::<BenchString>(&mut g);
    bench_for_backend::<BenchBuffer>(&mut g);
}

fn bench_get_or_intern_fill(c: &mut Criterion) {
    let mut g = c.benchmark_group("get_or_intern/fill-empty/new");
    g.throughput(Throughput::Elements(BENCH_LEN_STRINGS as u64));
    fn bench_for_backend<BB: BackendBenchmark>(g: &mut BenchmarkGroup<WallTime>) {
        g.bench_with_input(
            BB::NAME,
            &(BENCH_LEN_STRINGS, BENCH_STRING_LEN),
            |bencher, &(len_words, word_len)| {
                let words = generate_test_strings(len_words, word_len);
                bencher.iter_batched_ref(
                    || BB::setup(),
                    |interner| {
                        for word in &words {
                            black_box(interner.get_or_intern(word));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    bench_for_backend::<BenchBucket>(&mut g);
    bench_for_backend::<BenchString>(&mut g);
    bench_for_backend::<BenchBuffer>(&mut g);
}

fn bench_get_or_intern_already_filled(c: &mut Criterion) {
    let mut g = c.benchmark_group("get_or_intern/already-filled");
    g.throughput(Throughput::Elements(BENCH_LEN_STRINGS as u64));
    fn bench_for_backend<BB: BackendBenchmark>(g: &mut BenchmarkGroup<WallTime>) {
        g.bench_with_input(
            BB::NAME,
            &(BENCH_LEN_STRINGS, BENCH_STRING_LEN),
            |bencher, &(len_words, word_len)| {
                let words = generate_test_strings(len_words, word_len);
                bencher.iter_batched_ref(
                    || BB::setup_filled(&words),
                    |interner| {
                        for word in &words {
                            black_box(interner.get_or_intern(word));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    bench_for_backend::<BenchBucket>(&mut g);
    bench_for_backend::<BenchString>(&mut g);
    bench_for_backend::<BenchBuffer>(&mut g);
}

fn bench_resolve_already_filled(c: &mut Criterion) {
    let mut g = c.benchmark_group("resolve/already-filled");
    g.throughput(Throughput::Elements(BENCH_LEN_STRINGS as u64));
    fn bench_for_backend<BB: BackendBenchmark>(g: &mut BenchmarkGroup<WallTime>) {
        g.bench_with_input(
            BB::NAME,
            &(BENCH_LEN_STRINGS, BENCH_STRING_LEN),
            |bencher, &(len_words, word_len)| {
                let words = generate_test_strings(len_words, word_len);
                bencher.iter_batched_ref(
                    || BB::setup_filled_with_ids(&words),
                    |(interner, word_ids)| {
                        for &word_id in &*word_ids {
                            black_box(interner.resolve(word_id));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    bench_for_backend::<BenchBucket>(&mut g);
    bench_for_backend::<BenchString>(&mut g);
    bench_for_backend::<BenchBuffer>(&mut g);
}

fn bench_resolve_unchecked_already_filled(c: &mut Criterion) {
    let mut g = c.benchmark_group("resolve_unchecked/already-filled");
    g.throughput(Throughput::Elements(BENCH_LEN_STRINGS as u64));
    fn bench_for_backend<BB: BackendBenchmark>(g: &mut BenchmarkGroup<WallTime>) {
        g.bench_with_input(
            BB::NAME,
            &(BENCH_LEN_STRINGS, BENCH_STRING_LEN),
            |bencher, &(len_words, word_len)| {
                let words = generate_test_strings(len_words, word_len);
                bencher.iter_batched_ref(
                    || BB::setup_filled_with_ids(&words),
                    |(interner, word_ids)| {
                        for &word_id in &*word_ids {
                            black_box(
                                // SAFETY: We provide only valid symbols to the tested interners.
                                unsafe { interner.resolve_unchecked(word_id) },
                            );
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    bench_for_backend::<BenchBucket>(&mut g);
    bench_for_backend::<BenchString>(&mut g);
    bench_for_backend::<BenchBuffer>(&mut g);
}

fn bench_get_already_filled(c: &mut Criterion) {
    let mut g = c.benchmark_group("get/already-filled");
    g.throughput(Throughput::Elements(BENCH_LEN_STRINGS as u64));
    fn bench_for_backend<BB: BackendBenchmark>(g: &mut BenchmarkGroup<WallTime>) {
        g.bench_with_input(
            BB::NAME,
            &(BENCH_LEN_STRINGS, BENCH_STRING_LEN),
            |bencher, &(len_words, word_len)| {
                let words = generate_test_strings(len_words, word_len);
                bencher.iter_batched_ref(
                    || BB::setup_filled(&words),
                    |interner| {
                        for word in &words {
                            black_box(interner.get(word));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    bench_for_backend::<BenchBucket>(&mut g);
    bench_for_backend::<BenchString>(&mut g);
    bench_for_backend::<BenchBuffer>(&mut g);
}

fn bench_iter_already_filled(c: &mut Criterion) {
    let mut g = c.benchmark_group("iter/already-filled");
    g.throughput(Throughput::Elements(BENCH_LEN_STRINGS as u64));
    fn bench_for_backend<BB: BackendBenchmark>(g: &mut BenchmarkGroup<WallTime>)
    where
        for<'a> &'a <BB as BackendBenchmark>::Backend: IntoIterator<
            Item = (
                <<BB as BackendBenchmark>::Backend as Backend>::Symbol,
                &'a [u8],
            ),
        >,
    {
        g.bench_with_input(
            BB::NAME,
            &(BENCH_LEN_STRINGS, BENCH_STRING_LEN),
            |bencher, &(len_words, word_len)| {
                let words = generate_test_strings(len_words, word_len);
                bencher.iter_batched_ref(
                    || BB::setup_filled(&words),
                    |interner| {
                        for word in &*interner {
                            black_box(word);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    bench_for_backend::<BenchBucket>(&mut g);
    bench_for_backend::<BenchString>(&mut g);
    bench_for_backend::<BenchBuffer>(&mut g);
}
