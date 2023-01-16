// autocorrect: false

#[cfg(test)]
mod tests {
    use crate::*;
    use test::Bencher;

    static STEUP_ONCE: std::sync::Once = std::sync::Once::new();

    /// Load fixture file from `../tests/fixtures/`
    macro_rules! fixture {
        ($name:literal) => {
            include_str!(concat!("../tests/fixtures/", $name))
        };
    }

    fn setup() {
        STEUP_ONCE.call_once(|| {
            let config_str = include_str!("../tests/.autocorrectrc.test").to_owned();
            config::load(&config_str).unwrap();
        })
    }

    #[bench]
    fn bench_format_050(b: &mut Bencher) {
        b.iter(|| format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称"));
    }

    #[bench]
    fn bench_format_100(b: &mut Bencher) {
        b.iter(|| format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称，【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称"));
    }

    #[bench]
    fn bench_format_400(b: &mut Bencher) {
        b.iter(||  format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称，上调吉利汽车(00175)目标价12.58%，由15.9港元升至17.9港元，并维持吉汽为行业首选股，重申对其“买入”评级，坚信吉汽长远可成为行业赢家。 该行称，随着公司销量持续复苏及产品组合改善，预计今年销量可达148万辆，同比升9%，较公司原定目标销量141万辆为高。 该行又称称，上调公司今明两年每股盈利预测各13%及升毛利率0.1个百分点，以反映销量较预期高2%及产品组合改善，主要是由领克品牌带动。公司自去年8月开始已持续投资领克品牌及进行市场推广，带动领克销量环比有所改变，预期今明两年领克将占整体销量的11%及14%。 该行表示，由于低端国产车品牌在欠缺新车款及科技下，行业整合度将提升。另外，公司从去年第二季到12月为止，一直都积极推动经销商去库存，这将有利公司今年利润率复苏。"));
    }

    #[bench]
    fn bench_format_html(b: &mut Bencher) {
        let raw = fixture!("example.html");

        b.iter(|| format_for(raw, "html"));
    }

    #[bench]
    fn bench_format_json(b: &mut Bencher) {
        let raw = fixture!("example.json");

        b.iter(|| format_for(raw, "json"));
    }

    #[bench]
    fn bench_format_javascript(b: &mut Bencher) {
        let raw = fixture!("example.js");

        b.iter(|| format_for(raw, "javascript"));
    }

    #[bench]
    /// 2000 lines JSON, 4ms/iter
    fn bench_format_json_with_2k_lines(b: &mut Bencher) {
        let raw = fixture!("large.json");

        b.iter(|| format_for(raw, "json"));
    }

    #[bench]
    fn bench_halfwidth_full_english_100(b: &mut Bencher) {
        let raw = "Internal interface for communicating between a `proc_macro` client (a proc macro crate) and a `proc_macro` server (a compiler front-end).";

        b.iter(|| halfwidth::format_punctuation(raw));
    }

    #[bench]
    fn bench_spellcheck_050(b: &mut Bencher) {
        setup();
        b.iter(|| {
            spellcheck::format(
                "探索 apple 充满创新的世界，选购各式iphone、ipad、apple watch 和 mac",
            )
        });
    }

    #[bench]
    fn bench_spellcheck_100(b: &mut Bencher) {
        setup();
        b.iter(|| spellcheck::format("探索 apple 充满创新的世界，选购各式iphone、ipad、apple watch 和 mac、娱乐产品了，iphone 13 新款 - iphone SE 新款 ，并获得相关产品的专家支持服务。"));
    }

    #[bench]
    fn bench_spellcheck_400(b: &mut Bencher) {
        setup();
        b.iter(|| spellcheck::format("探索 apple 充满创新的世界，选购各式 iphone、ipad、apple watch 和 mac、娱乐产品了，iphone 13 新款 - iphone SE 新款 ，并获得相关产品的专家支持服务。通过 apple Trade In 换购计划，你可以用符合条件的智能手机来换购新 iphone，享受折抵优惠5。这样一来，你受益，地球也受益。现可在线加入 iphone 年年焕新计划，年年用上新 iphone，享受 AppleCare+ 服务计划，还可选择分期付款*。AirTag 是能帮你轻松追踪各种物品的高手。只要给钥匙串上
        挂一个，往背包里塞一个，在打开查找 app 时，除了能追踪自己的 Apple 设备之外，你还能看到钥匙和背包这些物品在哪里。只要放一个 AirTag，钱包在哪里这类问题会迎刃而解。通过查找 app 的全新“物品”标签页，都能让 AirTag 来指示物品位置。"));
    }

    #[bench]
    fn bench_markdown(b: &mut Bencher) {
        let raw = fixture!("example.md");
        setup();

        b.iter(|| format_for(raw, "markdown"))
    }

    #[bench]
    fn bench_lint_markdown(b: &mut Bencher) {
        setup();
        let markdown_raw = fixture!("example.md");
        b.iter(|| lint_for(markdown_raw, "markdown"));
    }

    #[bench]
    fn bench_lint_html(b: &mut Bencher) {
        setup();
        let html_raw = fixture!("example.html");
        b.iter(|| lint_for(html_raw, "html"));
    }

    #[bench]
    fn bench_lint_json(b: &mut Bencher) {
        setup();
        let json_raw = fixture!("example.json");
        b.iter(|| lint_for(json_raw, "json"));
    }

    #[bench]
    fn bench_lint_javascript(b: &mut Bencher) {
        setup();
        let js_raw = fixture!("example.js");
        b.iter(|| lint_for(js_raw, "javascript"));
    }

    #[bench]
    fn bench_lint_yaml(b: &mut Bencher) {
        setup();
        let yaml_raw = fixture!("example.yml");
        b.iter(|| lint_for(yaml_raw, "yaml"));
    }

    #[bench]
    fn bench_lint_to_json(b: &mut Bencher) {
        let markdown_raw = fixture!("example.md");
        b.iter(|| lint_for(markdown_raw, "markdown").to_json())
    }

    #[bench]
    fn bench_lint_to_diff(b: &mut Bencher) {
        let markdown_raw = fixture!("example.md");
        b.iter(|| lint_for(markdown_raw, "markdown").to_diff(true))
    }
}
