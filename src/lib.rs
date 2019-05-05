#![feature(test)]
#[cfg(test)]
extern crate test;

pub mod dyndeser;
pub mod openapi;
pub mod reflection;

#[cfg(test)]
mod tests {
    use test::Bencher;

    static TEST_CASE: &str = include_str!("data/buy_browse_v1_beta_oas3.json");

    #[test]
    fn compare_test() {
        let mut reflection_openapi = crate::openapi::OpenApi::default();
        crate::dyndeser::read_json(TEST_CASE, &mut reflection_openapi).unwrap();

        let serde_openapi: crate::openapi::OpenApi = serde_json::from_str(TEST_CASE).unwrap();

        assert_eq!(serde_openapi, reflection_openapi);
    }

    #[bench]
    fn bench_serde(bencher: &mut Bencher) {
        bencher.iter(|| {
            let input = test::black_box(TEST_CASE);
            let parsed: crate::openapi::OpenApi = serde_json::from_str(input).unwrap();
            test::black_box(parsed)
        })
    }

    #[bench]
    fn bench_dynamic(bencher: &mut Bencher) {
        bencher.iter(|| {
            let input = test::black_box(TEST_CASE);
            let mut parsed = crate::openapi::OpenApi::default();
            crate::dyndeser::read_json(input, &mut parsed).unwrap();
            test::black_box(parsed)
        })
    }
}
