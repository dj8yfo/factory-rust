use cargo_near::BuildScriptOpts;

fn main() {
    let _e = env_logger::Builder::new().parse_default_env().try_init();

    let opts = cargo_near::BuildOptsExtended {
        workdir: "../product-donation",
        env: vec![("NEP330_BUILD_INFO_CONTRACT_PATH", "product-donation")], // unix path of target contract from root of repo
        build_opts: cargo_near::BuildOpts::default(),
        build_script_opts: BuildScriptOpts {
            result_env_key: Some("BUILD_RS_SUB_BUILD_ARTIFACT_1"),
            rerun_if_changed_list: vec!["../product-donation", "../Cargo.toml", "../Cargo.lock"],
            build_skipped_when_env_is: vec![
                ("PROFILE", "debug"),
                ("CARGO_NEAR_ABI_GENERATION", "true"),
            ],
            distinct_target_dir: Some("../target/build-rs-product-donation"),
            stub_path: Some("../target/stub.bin"),
        },
    };
    cargo_near::build_extended(opts).expect("sub-contract build error");
}
