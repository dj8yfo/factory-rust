
fn main() {
    let build_opts = cargo_near::BuildOpts::new(
        false, false, false, false, false, None, None, None, false, None,
    );
    let opts = cargo_near::build_rs::SubBuildOpts {
        workdir: "../product-donation",
        metadata_contract_path: "product-donation",
        build_command: ("cargo near build".to_string(), build_opts),

        distinct_target_dir: "../target/sub-build-product-donation",
        skipped_profiles: vec!["debug"],
        stub_path: "../target/stub.bin",
        rerun_if_changed_list: vec!["../product-donation", "../Cargo.toml"],
    };
    let result_env_key = "BUILD_RS_SUB_BUILD_ARTIFACT_1".to_string();
    cargo_near::build_rs::process_sub_build(opts, result_env_key).expect("sub build error");
}



