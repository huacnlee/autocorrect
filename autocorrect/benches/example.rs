// autocorrect-disable

use autocorrect::*;

static STEUP_ONCE: std::sync::Once = std::sync::Once::new();

use criterion::{criterion_group, criterion_main, Criterion};

fn setup() {
    STEUP_ONCE.call_once(|| {
        let config_str = include_str!("../../autocorrect/tests/.autocorrectrc.test").to_owned();
        autocorrect::config::load(&config_str).unwrap();
    })
}

fn bench_format(c: &mut Criterion) {
    c.bench_function("format_050", |b| {
        b.iter(|| format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称"));
    });

    c.bench_function("format_100", |b| {
        b.iter(|| format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称，【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称"));
    });

    c.bench_function("format_400", |b| {
        b.iter(|| format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称，上调吉利汽车(00175)目标价12.58%，由15.9港元升至17.9港元，并维持吉汽为行业首选股，重申对其“买入”评级，坚信吉汽长远可成为行业赢家。 该行称，随着公司销量持续复苏及产品组合改善，预计今年销量可达148万辆，同比升9%，较公司原定目标销量141万辆为高。 该行又称称，上调公司今明两年每股盈利预测各13%及升毛利率0.1个百分点，以反映销量较预期高2%及产品组合改善，主要是由领克品牌带动。公司自去年8月开始已持续投资领克品牌及进行市场推广，带动领克销量环比有所改变，预期今明两年领克将占整体销量的11%及14%。 该行表示，由于低端国产车品牌在欠缺新车款及科技下，行业整合度将提升。另外，公司从去年第二季到12月为止，一直都积极推动经销商去库存，这将有利公司今年利润率复苏。"));
    });
}

fn bench_format_html(c: &mut Criterion) {
    let raw = include_str!("./fixtures/example.html");

    c.bench_function("format_html", |b| {
        b.iter(|| format_for(raw, "html"));
    });
}

fn bench_format_json(c: &mut Criterion) {
    let raw = include_str!("./fixtures/example.json");

    c.bench_function("format_json", |b| {
        b.iter(|| format_for(raw, "json"));
    });
}

fn bench_format_javascript(c: &mut Criterion) {
    let raw = include_str!("./fixtures/example.js");

    c.bench_function("format_javascript", |b| {
        b.iter(|| format_for(raw, "javascript"));
    });
}

/// 2000 lines JSON, 4ms/iter
fn bench_format_json_with_2k_lines(c: &mut Criterion) {
    let raw = include_str!("./fixtures/large.json");

    c.bench_function("format_json_2k", |b| {
        b.iter(|| format_for(raw, "json"));
    });
}

/// 38 lines Jupyter file, 89.629 µs/iter
fn bench_format_jupyter(c: &mut Criterion) {
    let raw = include_str!("../tests/fixtures/jupyter.sm.ipynb");

    c.bench_function("format_jupyter", |b| {
        b.iter(|| format_for(raw, "jupyter"));
    });
}

fn bench_halfwidth_full_english_100(c: &mut Criterion) {
    let raw = "Internal interface for communicating between a `proc_macro` client (a proc macro crate) and a `proc_macro` server (a compiler front-end).";

    c.bench_function("halfwidth_english", |b| {
        b.iter(|| halfwidth::format_punctuation(raw));
    });
}

fn bench_spellcheck(c: &mut Criterion) {
    setup();

    // [1.6012 µs 1.6122 µs 1.6306 µs]
    c.bench_function("spellcheck_50", |b| {
        b.iter(|| {
            spellcheck::format(
                "探索 apple 充满创新的世界，选购各式iphone、ipad、apple watch 和 mac",
            )
        });
    });

    // [3.0968 µs 3.1696 µs 3.2653 µs]
    c.bench_function("spellcheck_100", |b| {
        b.iter(|| spellcheck::format("探索 apple 充满创新的世界，选购各式iphone、ipad、apple watch 和 mac、娱乐产品了，iphone 13 新款 - iphone SE 新款 ，并获得相关产品的专家支持服务。"));
    });

    // [10.136 µs 10.478 µs 10.898 µs]
    c.bench_function("spellcheck_400", |b| {
        b.iter(|| spellcheck::format("探索 apple 充满创新的世界，选购各式 iphone、ipad、apple watch 和 mac、娱乐产品了，iphone 13 新款 - iphone SE 新款 ，并获得相关产品的专家支持服务。通过 apple Trade In 换购计划，你可以用符合条件的智能手机来换购新 iphone，享受折抵优惠5。这样一来，你受益，地球也受益。现可在线加入 iphone 年年焕新计划，年年用上新 iphone，享受 AppleCare+ 服务计划，还可选择分期付款*。AirTag 是能帮你轻松追踪各种物品的高手。只要给钥匙串上
        挂一个，往背包里塞一个，在打开查找 app 时，除了能追踪自己的 Apple 设备之外，你还能看到钥匙和背包这些物品在哪里。只要放一个 AirTag，钱包在哪里这类问题会迎刃而解。通过查找 app 的全新“物品”标签页，都能让 AirTag 来指示物品位置。"));
    });
}

fn bench_markdown(c: &mut Criterion) {
    let raw = include_str!("./fixtures/example.md");
    setup();

    c.bench_function("format_markdown", |b| {
        b.iter(|| format_for(raw, "markdown"))
    });
}

fn bench_lint(c: &mut Criterion) {
    let markdown_raw = include_str!("./fixtures/example.md");
    let html_raw = include_str!("./fixtures/example.html");
    let json_raw = include_str!("./fixtures/example.json");
    let js_raw = include_str!("./fixtures/example.js");
    let yaml_raw = include_str!("./fixtures/example.yml");

    setup();

    c.bench_function("lint_markdown", |b| {
        b.iter(|| lint_for(markdown_raw, "markdown"))
    });
    c.bench_function("lint_json", |b| b.iter(|| lint_for(json_raw, "json")));
    c.bench_function("lint_html", |b| b.iter(|| lint_for(html_raw, "html")));
    c.bench_function("lint_javascript", |b| {
        b.iter(|| lint_for(js_raw, "javascript"))
    });
    c.bench_function("lint_yaml", |b| b.iter(|| lint_for(yaml_raw, "yaml")));
}

fn bench_lint_output(c: &mut Criterion) {
    let markdown_raw = include_str!("./fixtures/example.md");

    c.bench_function("lint_to_json", |b| {
        b.iter(|| lint_for(markdown_raw, "markdown").to_json())
    });
    c.bench_function("lint_to_diff", |b| {
        b.iter(|| lint_for(markdown_raw, "markdown").to_diff(false))
    });
}

criterion_group!(
    format_benches,
    bench_format,
    bench_format_html,
    bench_halfwidth_full_english_100,
    bench_format_json,
    bench_format_javascript,
    bench_format_json_with_2k_lines,
    bench_format_jupyter,
    bench_markdown,
);

criterion_group!(spellcheck_benches, bench_spellcheck);
criterion_group!(lint_benches, bench_lint, bench_lint_output);

criterion_main!(format_benches, spellcheck_benches, lint_benches);
