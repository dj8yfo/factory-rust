
fn main() {
    let _e = env_logger::Builder::new().parse_default_env().try_init();
    let opts = cargo_near::BuildOptsExtended {
        // extract to Dir opts
        workdir: "../product-donation",
        stub_path: "../target/stub.bin",
        distinct_target_dir: "../target/sub-build-product-donation",
        // TODO: replace with override env argument map
        metadata_contract_path: "product-donation",
        build_opts: cargo_near::BuildOpts::default(),

        // TODO: extract these to BuildScriptOpts
        // TODO: rename to build_rs rerun_if_changed_list_build_script: 
        // TODO: emit these even on skipped builds
        rerun_if_changed_list: vec!["../product-donation", "../Cargo.toml", "../Cargo.lock"],
        // TODO: replace with env variables names to skip onto `skipped_env_variables`
        // these should be related to BuildScriptOpts
        skipped_profiles: vec!["debug"],
    };
    // TODO: move this to extended opts field as well
    let result_env_key = "BUILD_RS_SUB_BUILD_ARTIFACT_1".to_string();
    cargo_near::build_extended(opts, result_env_key).expect("sub build error");
}



