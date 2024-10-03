use cargo_near_build::{bon, extended};
use cargo_near_build::{BuildImplicitEnvOpts, BuildOpts};

fn main() {
    let _e = env_logger::Builder::new().parse_default_env().try_init();

    // directory of target sub-contract's crate
    let workdir = "../product-donation";
    // unix path to target sub-contract's crate from root of the repo
    let nep330_contract_path = "product-donation";

    let build_opts = BuildOpts::builder().no_default_features(true).build();

    let pwd = std::env::current_dir().expect("get pwd");
    let distinct_target = pwd.join("../target/build-rs-product-donation");
    let stub_path = pwd.join("../target/stub.bin");

    let build_implicit_env_opts = BuildImplicitEnvOpts::builder()
        .nep330_contract_path(nep330_contract_path)
        .cargo_target_dir(distinct_target.to_string_lossy())
        .build();

    let build_script_opts = extended::BuildScriptOpts::builder()
        .rerun_if_changed_list(bon::vec![workdir, "../Cargo.toml", "../Cargo.lock",])
        .build_skipped_when_env_is(vec![
            // shorter build for `cargo check`
            ("PROFILE", "debug"),
            (cargo_near_build::env_keys::BUILD_RS_ABI_STEP_HINT, "true"),
        ])
        .stub_path(stub_path.to_string_lossy())
        .result_env_key("BUILD_RS_SUB_BUILD_ARTIFACT_1")
        .build();

    let extended_opts = extended::BuildOptsExtended::builder()
        .workdir(workdir)
        .build_opts(build_opts)
        .build_implicit_env_opts(build_implicit_env_opts)
        .build_script_opts(build_script_opts)
        .build();

    cargo_near_build::extended::build(extended_opts).expect("sub-contract build error");
}
