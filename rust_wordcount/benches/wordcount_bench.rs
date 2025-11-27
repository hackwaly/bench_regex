use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regex::{Regex};

const TEST_STRING: &str = "Non enim laborum velit commodo deserunt incididunt elit sunt nulla ullamco. Quis ipsum aliqua id mollit minim velit et cupidatat eiusmod nostrud pariatur duis irure ad. Dolore enim et elit fugiat.\nNostrud elit minim aute qui labore id aute aute ea nostrud cupidatat. Aliquip et commodo anim dolor nostrud voluptate proident. Voluptate tempor amet consequat nisi excepteur aute anim aute. Sunt ipsum tempor esse consequat cupidatat.\nIpsum minim cillum adipisicing incididunt incididunt qui non excepteur mollit qui. Non aute sunt dolore eu sunt ea aute nisi dolor eiusmod. Fugiat id culpa exercitation consectetur cupidatat. Sunt sint nostrud dolor aute sit cupidatat voluptate reprehenderit ut cillum nulla. Dolore sunt elit elit et quis qui.\n";

// Pre-compiled regex version (most realistic scenario)
struct WordCounter {
    re: Regex,
}

impl WordCounter {
    fn new() -> Self {
        Self {
            re: Regex::new(r"(\w+)").unwrap(),
        }
    }

    fn count(&self, input: &str) -> (usize, usize, usize) {
        let mut words = 0;

        for _cap in self.re.captures_iter(input) {
            words += 1;
        }

        (0, words, 0)
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("wordcount_implementations");

    // // Original implementation (creates regex each time)
    // group.bench_function("original", |b| {
    //     b.iter(|| wordcount(black_box(TEST_STRING)))
    // });

    // // RegexSet-based implementation
    // group.bench_function("regexset", |b| {
    //     b.iter(|| wordcount_regexset(black_box(TEST_STRING)))
    // });

    // // Optimized with find_iter
    // group.bench_function("optimized_find", |b| {
    //     b.iter(|| wordcount_optimized(black_box(TEST_STRING)))
    // });

    // Pre-compiled regex (most realistic)
    group.bench_function("precompiled", |b| {
        let counter = WordCounter::new();
        b.iter(|| counter.count(black_box(TEST_STRING)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
