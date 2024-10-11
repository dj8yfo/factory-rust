use cargo_near_build::BuildOpts;
use near_workspaces::types::{AccountId, KeyType, NearToken, SecretKey};
use serde_json::json;

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _e = env_logger::try_init();
    let sandbox = near_workspaces::sandbox().await?;
    let artifact = cargo_near_build::build(BuildOpts::default(), None)?;

    let contract_wasm = std::fs::read(artifact.path)?;
    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let alice = sandbox
        .create_tla(
            "alice".parse().unwrap(),
            SecretKey::from_random(KeyType::ED25519),
        )
        .await?
        .unwrap();

    let bob = sandbox.dev_create_account().await?;

    let res = contract
        .call("create_factory_subaccount_and_deploy")
        .args_json(json!({"name": "donation_for_alice", "beneficiary": alice.id()}))
        .max_gas()
        .deposit(NearToken::from_near(5))
        .transact()
        .await?;

    assert!(res.is_success());

    let sub_accountid: AccountId = format!("donation_for_alice.{}", contract.id())
        .parse()
        .unwrap();

    let res = bob
        .view(&sub_accountid, "get_beneficiary")
        .args_json({})
        .await?;

    assert_eq!(res.json::<AccountId>()?, alice.id().clone());

    let res = bob
        .call(&sub_accountid, "donate")
        .args_json({})
        .max_gas()
        .deposit(NearToken::from_near(5))
        .transact()
        .await?;

    assert!(res.is_success());

    Ok(())
}

#[tokio::test]
async fn docker_test_meta() -> Result<(), Box<dyn std::error::Error>> {
    let _e = env_logger::try_init();
    let sandbox = near_workspaces::sandbox().await?;
    let artifact = cargo_near_build::docker::build(Default::default())?;

    let contract_wasm = std::fs::read(artifact.path)?;
    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let alice = sandbox
        .create_tla(
            "alice".parse().unwrap(),
            SecretKey::from_random(KeyType::ED25519),
        )
        .await?
        .unwrap();

    let bob = sandbox.dev_create_account().await?;

    let res = contract
        .call("create_factory_subaccount_and_deploy")
        .args_json(json!({"name": "donation_for_alice", "beneficiary": alice.id()}))
        .max_gas()
        .deposit(NearToken::from_near(5))
        .transact()
        .await?;

    assert!(res.is_success());

    let sub_accountid: AccountId = format!("donation_for_alice.{}", contract.id())
        .parse()
        .unwrap();

    let res = bob
        .view(&sub_accountid, "get_beneficiary")
        .args_json({})
        .await?;

    assert_eq!(res.json::<AccountId>()?, alice.id().clone());

    let res = bob
        .call(&sub_accountid, "donate")
        .args_json({})
        .max_gas()
        .deposit(NearToken::from_near(5))
        .transact()
        .await?;

    assert!(res.is_success());

    Ok(())
}
