#[cfg(feature = "canbench-rs")]
mod benches {
    use crate::calls::profile_calls::add_profile;
    use canbench_rs::bench;
    use test::mocks::models::mock_post_profile;
    // use test::mocks::principals::member_test_id;

    #[bench]
    fn bench_add_profile() {
        // NOTE: the result is printed to prevent the compiler from optimizing the call away.
        // let principal = member_test_id();
        let profile = mock_post_profile();
        println!("{:?}", add_profile(profile));
    }
}
