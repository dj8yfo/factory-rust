use cargo_near_build::extended::BuildScriptOpts;

fn main() {
    let _e = env_logger::Builder::new().parse_default_env().try_init();

    let opts = cargo_near_build::extended::BuildOptsExtended {
        workdir: "../product-donation",
        env: vec![
            // unix path of target contract from root of repo
            (cargo_near_build::env_keys::nep330::CONTRACT_PATH, "product-donation")
        ], 
        build_opts: cargo_near_build::BuildOpts::default(),
        build_script_opts: BuildScriptOpts {
            result_env_key: Some("BUILD_RS_SUB_BUILD_ARTIFACT_1"),
            rerun_if_changed_list: vec!["../product-donation", "../Cargo.toml", "../Cargo.lock"],
            build_skipped_when_env_is: vec![
                // shorter build for `cargo check`
                ("PROFILE", "debug"),
                (cargo_near_build::env_keys::BUILD_RS_ABI_STEP_HINT, "true"),
            ],
            distinct_target_dir: Some("../target/build-rs-product-donation"),
            stub_path: Some("../target/stub.bin"),
        },
    };
    cargo_near_build::extended::build(opts).expect("sub-contract build error");
}
