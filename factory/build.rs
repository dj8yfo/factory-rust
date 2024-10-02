use cargo_near_build::{extended::BuildScriptOpts, BuildImplicitEnvOpts};

fn main() {
    let _e = env_logger::Builder::new().parse_default_env().try_init();

    let regular_build_opts = cargo_near_build::BuildOpts {
        no_default_features: true,
        ..Default::default()
    };

    let opts = cargo_near_build::extended::BuildOptsExtended {
        workdir: "../product-donation".into(),
        build_opts: regular_build_opts,
        build_implicit_env_opts: BuildImplicitEnvOpts {
            nep330_contract_path: Some("product-donation".into()),
            cargo_target_dir: Some("../target/build-rs-product-donation".into()),
        },
        build_script_opts: BuildScriptOpts {
            result_env_key: Some("BUILD_RS_SUB_BUILD_ARTIFACT_1".into()),
            rerun_if_changed_list: vec![
                "../product-donation".into(),
                "../Cargo.toml".into(),
                "../Cargo.lock".into(),
            ],
            build_skipped_when_env_is: vec![
                // shorter build for `cargo check`
                ("PROFILE".into(), "debug".into()),
                (
                    cargo_near_build::env_keys::BUILD_RS_ABI_STEP_HINT.into(),
                    "true".into(),
                ),
            ],
            stub_path: Some("../target/stub.bin".into()),
        },
    };
    cargo_near_build::extended::build(opts).expect("sub-contract build error");
}
