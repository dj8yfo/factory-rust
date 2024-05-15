use cargo_near::commands::build_command::BUILD_RS_ABI_STEP_HINT_ENV_KEY;
use cargo_near::commands::build_command::{NEP330_BUILD_CMD_ENV_KEY, NEP330_CONTRACT_PATH_ENV_KEY};
use cargo_near::util::CompilationArtifact;

macro_rules! print_warn {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

struct SubBuildArguments<'a> {
    workdir: &'a str,
    /// the desired value of `contract_path` from `BuildInfo`
    /// https://github.com/near/NEPs/blob/master/neps/nep-0330.md?plain=1#L155
    metadata_contract_path: &'a str,
    /// substitution export of `CARGO_TARGET_DIR`,
    /// which is required to avoid deadlock https://github.com/rust-lang/cargo/issues/8938
    /// should be a subfolder of `CARGO_TARGET_DIR` of package being built to work normally in
    /// docker builds
    ///
    /// if this path relative, the base is `workdir` field
    distinct_target_dir: &'a str,
}

fn create_stub_file(out_path: &std::path::Path) {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&out_path)
        .expect("unable to open/create data file");
}

fn main() {
    let profile = std::env::var("PROFILE").unwrap_or("unknown".to_string());
    print_warn!("`PROFILE` env set to `{}`", profile);

    // NOTE: skipping emitting output sub-build `*.wasm` may be helpful when interacting with `rust-analyzer/flycheck`,
    // `cargo check`, `bacon` and other dev-tools, running `cargo test --workspace`, etc.
    if profile == "debug" {
        print_warn!("No need to build factory's product contract during `debug` profile build");
    }
    if std::env::var(BUILD_RS_ABI_STEP_HINT_ENV_KEY).is_ok() {
        print_warn!("No need to build factory's product contract during ABI generation step");
    }

    if std::env::var(BUILD_RS_ABI_STEP_HINT_ENV_KEY).is_ok() || profile == "debug" {
        let stub_path = std::path::Path::new("../target/stub.bin");
        create_stub_file(stub_path);
        let stub_path = stub_path
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string();
        // TODO: replace `cargo:` -> `cargo::`, as the former is being deprecated since rust 1.77
        println!(
            "cargo:rustc-env={}={}",
            "BUILD_RS_SUB_BUILD_ARTIFACT", stub_path
        );
        return;
    }
    let artifact = {
        let args = SubBuildArguments {
            workdir: "../product-donation",
            metadata_contract_path: "product-donation",
            distinct_target_dir: "../target/sub-build-product-donation",
        };

        compile_near_artifact(args).expect("problem compiling factory's sub-build")
    };
    pretty_print(&artifact);
    println!(
        "cargo:rustc-env={}={}",
        "BUILD_RS_SUB_BUILD_ARTIFACT",
        artifact.path.into_string()
    );
    println!("cargo:rerun-if-changed=../product-donation");
    println!("cargo:rerun-if-changed=../Cargo.toml");
}

/// `CARGO_NEAR_BUILD_COMMAND` and `CARGO_NEAR_CONTRACT_PATH`
/// exports ensure, that contract, deployed from factory, produces the same metadata
/// as one, deployed by `cargo near deploy` from `product-donation` subfolder,
/// (in the context of docker builds)
///
/// `CARGO_TARGET_DIR` export is needed to avoid attempt to acquire same `target/<profile-path>/.cargo-lock`
/// as the `cargo` process, which is running the build-script
fn compile_near_artifact(
    args: SubBuildArguments,
) -> Result<CompilationArtifact, Box<dyn std::error::Error>> {
    let _tmp_workdir = tmp_env::set_current_dir(args.workdir)?;

    let _tmp_contract_path_env =
        tmp_env::set_var(NEP330_CONTRACT_PATH_ENV_KEY, args.metadata_contract_path);

    let (build_args, _tmp_build_cmd_env) = (
        cargo_near::BuildArgs::new(
            false, false, false, false, false, None, None, None, false, None,
        ),
        tmp_env::set_var(NEP330_BUILD_CMD_ENV_KEY, "cargo near build"),
    );

    let _tmp_cargo_target_env = tmp_env::set_var("CARGO_TARGET_DIR", args.distinct_target_dir);
    let artifact = cargo_near::run_build(build_args)?;

    Ok(artifact)
}

fn pretty_print(artifact: &CompilationArtifact) {
    let hash = artifact
        .compute_hash()
        .expect("problem hashing the artifact");

    print_warn!("");
    print_warn!("");
    print_warn!(
        "sub-build artifact path: {}",
        artifact.path.clone().into_string()
    );
    print_warn!("sub-build artifact hashsum: {}", hash);
    print_warn!("");
    print_warn!("");
}
