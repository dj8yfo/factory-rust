use cargo_near::commands::build_command::BUILD_RS_ABI_STEP_HINT_ENV_KEY;
use cargo_near::commands::build_command::{NEP330_BUILD_CMD_ENV_KEY, NEP330_CONTRACT_PATH_ENV_KEY};
use cargo_near::util::CompilationArtifact;

macro_rules! print_warn {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub struct SubBuildOpts<'a> {
    pub workdir: &'a str,
    /// the desired value of `contract_path` from `BuildInfo`
    /// https://github.com/near/NEPs/blob/master/neps/nep-0330.md?plain=1#L155
    pub metadata_contract_path: &'a str,
    /// the first of element in tuple `cargo near build...` command must correspond to 2nd element,
    /// the first element is used as an override for contract metadata field
    /// https://github.com/near/NEPs/blob/master/neps/nep-0330.md?plain=1#L156
    pub build_command: (String, cargo_near::BuildArgs),
    /// substitution export of `CARGO_TARGET_DIR`,
    /// which is required to avoid deadlock https://github.com/rust-lang/cargo/issues/8938
    /// should be a subfolder of `CARGO_TARGET_DIR` of package being built to work normally in
    /// docker builds
    ///
    /// if this path is relative, then the base is `workdir` field
    pub distinct_target_dir: &'a str,
    /// skipping emitting output sub-build `*.wasm` may be helpful when
    /// interacting with `rust-analyzer/flycheck`,
    /// `cargo check`, `bacon` and other dev-tools, running `cargo test --workspace`, etc.
    pub skipped_profiles: Vec<&'a str>,
    /// path of stub file, where a placeholder empty `wasm` output is emitted to
    pub stub_path: &'a str,
    /// list of paths for [`cargo:rerun-if-changed=`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
    /// instruction
    pub rerun_if_changed_list: Vec<&'a str>,
}

fn create_stub_file(out_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&out_path)?;
    Ok(())
}
fn main() {
    let build_opts = cargo_near::BuildArgs::new(
        false, false, false, false, false, None, None, None, false, None,
    );
    let opts = SubBuildOpts {
        workdir: "../product-donation",
        metadata_contract_path: "product-donation",
        build_command: ("cargo near build".to_string(), build_opts),

        distinct_target_dir: "../target/sub-build-product-donation",
        skipped_profiles: vec!["debug"],
        stub_path: "../target/stub.bin",
        rerun_if_changed_list: vec!["../product-donation", "../Cargo.toml"],
    };
    let result_env_key = "BUILD_RS_SUB_BUILD_ARTIFACT_1".to_string();
    process_sub_build(opts, result_env_key).expect("sub build error");
}

pub fn process_sub_build(
    args: SubBuildOpts,
    result_env_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    skip_or_compile(&args, &result_env_key)?;
    print_warn!(
        "Path to result artifact of build in `{}` is exported to `{}`",
        &args.workdir,
        result_env_key,
    );
    Ok(())
}

// TODO: replace `cargo:` -> `cargo::`, as the former is being deprecated since rust 1.77
// or handle both with `rustc_version`
fn skip_or_compile(
    args: &SubBuildOpts,
    result_env_key: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    if skip(args) {
        let stub_path = std::path::Path::new(&args.stub_path);
        create_stub_file(stub_path)?;
        let stub_path = stub_path
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string();
        print_warn!("Sub-build empty artifact stub written to: `{}`", stub_path);
        println!("cargo:rustc-env={}={}", result_env_key, stub_path);
    } else {
        let artifact = compile_near_artifact(&args)?;
        pretty_print(&artifact)?;
        println!(
            "cargo:rustc-env={}={}",
            result_env_key,
            artifact.path.into_string()
        );
        for path in args.rerun_if_changed_list.iter() {
            println!("cargo:rerun-if-changed={}", path);
        }
    }
    Ok(())
}

fn skip(args: &SubBuildOpts) -> bool {
    let profile = std::env::var("PROFILE").unwrap_or("unknown".to_string());
    print_warn!("`PROFILE` env set to `{}`", profile);

    if args.skipped_profiles.contains(&profile.as_str()) {
        print_warn!(
            "No need to build factory's product contract during `{}` profile build",
            profile
        );
        return true;
    }
    if std::env::var(BUILD_RS_ABI_STEP_HINT_ENV_KEY).is_ok() {
        print_warn!("No need to build factory's product contract during ABI generation step");
        return true;
    }
    false
}

/// `CARGO_NEAR_BUILD_COMMAND` and `CARGO_NEAR_CONTRACT_PATH`
/// exports ensure, that contract, deployed from factory, produces the same metadata
/// as one, deployed by `cargo near deploy` from `product-donation` subfolder,
/// (in the context of docker builds)
///
/// `CARGO_TARGET_DIR` export is needed to avoid attempt to acquire same `target/<profile-path>/.cargo-lock`
/// as the `cargo` process, which is running the build-script
fn compile_near_artifact(
    args: &SubBuildOpts,
) -> Result<CompilationArtifact, Box<dyn std::error::Error>> {
    let _tmp_workdir = tmp_env::set_current_dir(args.workdir)?;

    let _tmp_contract_path_env =
        tmp_env::set_var(NEP330_CONTRACT_PATH_ENV_KEY, args.metadata_contract_path);
    let _tmp_build_cmd_env = tmp_env::set_var(NEP330_BUILD_CMD_ENV_KEY, &args.build_command.0);

    let _tmp_cargo_target_env = tmp_env::set_var("CARGO_TARGET_DIR", args.distinct_target_dir);
    let artifact = cargo_near::run_build(args.build_command.1.clone())?;

    Ok(artifact)
}

fn pretty_print(artifact: &CompilationArtifact) -> Result<(), Box<dyn std::error::Error>> {
    let hash = artifact.compute_hash()?;

    print_warn!("");
    print_warn!("");
    print_warn!(
        "Sub-build artifact path: {}",
        artifact.path.clone().into_string()
    );
    print_warn!("Sub-build artifact hashsum: {}", hash);
    print_warn!("");
    print_warn!("");
    Ok(())
}
