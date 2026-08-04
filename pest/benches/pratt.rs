// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pest::pratt_parser::{Affix, Assoc, ConstPrattParser, Op, PrattParser, PrattParserOps};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Rule {
    A0 = 51037,
    A1 = 25942,
    A2 = 60110,
    A3 = 55613,
    A4 = 16828,
    A5 = 18115,
    A6 = 45332,
    A7 = 24437,
    A8 = 30676,
    A9 = 64324,
    B0 = 29091,
    B1 = 33302,
    B2 = 16456,
    B3 = 31973,
    B4 = 9453,
    B5 = 29006,
    B6 = 39777,
    B7 = 36706,
    B8 = 60606,
    B9 = 36548,
    C0 = 51211,
    C1 = 27690,
    C2 = 9493,
    C3 = 26720,
    C4 = 17148,
    C5 = 44851,
    C6 = 25713,
    C7 = 17295,
    C8 = 12738,
    C9 = 61099,
}

const RULES: &[Rule] = &[
    Rule::A0,
    Rule::A1,
    Rule::A2,
    Rule::A3,
    Rule::A4,
    Rule::A5,
    Rule::A6,
    Rule::A7,
    Rule::A8,
    Rule::A9,
    Rule::B0,
    Rule::B1,
    Rule::B2,
    Rule::B3,
    Rule::B4,
    Rule::B5,
    Rule::B6,
    Rule::B7,
    Rule::B8,
    Rule::B9,
    Rule::C0,
    Rule::C1,
    Rule::C2,
    Rule::C3,
    Rule::C4,
    Rule::C5,
    Rule::C6,
    Rule::C7,
    Rule::C8,
    Rule::C9,
];

fn build_runtime() -> PrattParser<Rule> {
    dbg!(size_of::<Rule>());
    let mut parser = PrattParser::new();
    for &rule in RULES {
        parser = parser.op(Op::infix(rule, Assoc::Left));
    }
    parser
}

fn build_const() -> ConstPrattParser<Rule, 30> {
    let mut ops = [(Rule::A0, Affix::Infix(Assoc::Left), 0u32); 30];
    for (i, &rule) in RULES.iter().enumerate() {
        ops[i] = (rule, Affix::Infix(Assoc::Left), (i as u32) + 1);
    }
    ConstPrattParser::new_const(ops)
}

fn benchmark(b: &mut Criterion) {
    let runtime = build_runtime();
    let const_ = build_const();

    b.bench_function("pratt_runtime_lookup", |b| {
        b.iter(|| {
            for &rule in RULES {
                black_box(runtime.get(&rule));
            }
        })
    });

    b.bench_function("pratt_const_lookup", |b| {
        b.iter(|| {
            for &rule in RULES {
                black_box(const_.get(&rule));
            }
        })
    });
}

criterion_group!(benchmarks, benchmark);
criterion_main!(benchmarks);
