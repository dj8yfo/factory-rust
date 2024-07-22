import? 'local.just'
default_args := ''

factory_contract := "repro-fct-37.testnet"
child_deploy_signer := "child-deploy-signer-37.testnet"
product_contract_name := "donation-product"
product_from_factory_contract := product_contract_name + "." + factory_contract
product_standalone_contract := "repro-fct-product-37.testnet"
factory_call_payload := "{ \"name\": \"" + product_contract_name + "\", \"beneficiary\": \"donatello2.testnet\"}"

create-factory-dev-acc:
    near account create-account sponsor-by-faucet-service {{factory_contract}} autogenerate-new-keypair save-to-keychain network-config testnet create || true

# additional_args can most often be `--no-docker`
deploy-factory additional_args=default_args: create-factory-dev-acc
    cd factory && cargo near deploy {{additional_args}} {{factory_contract}} without-init-call network-config testnet sign-with-keychain send

test-meta-factory:
    near contract call-function as-read-only {{factory_contract}} contract_source_metadata json-args {} network-config testnet now
    near contract download-abi {{factory_contract}} save-to-file deployed.abi.json network-config testnet now

create-child-deploy-signer-acc:
    near account create-account sponsor-by-faucet-service {{child_deploy_signer}} autogenerate-new-keypair save-to-keychain network-config testnet create || true

deploy-from-factory: create-child-deploy-signer-acc
    sleep 30
    near contract call-function as-transaction {{factory_contract}} create_factory_subaccount_and_deploy json-args '{{factory_call_payload}}' prepaid-gas '300.0 Tgas' attached-deposit '1.7 NEAR' sign-as {{child_deploy_signer}} network-config testnet sign-with-keychain send

test-meta-product:
    near contract call-function as-read-only {{product_from_factory_contract}} contract_source_metadata json-args {} network-config testnet now
    near contract download-abi {{product_from_factory_contract}} save-to-file deployed.abi.json network-config testnet now

create-standalone-product-dev-acc:
    near account create-account sponsor-by-faucet-service {{product_standalone_contract}} autogenerate-new-keypair save-to-keychain network-config testnet create || true

deploy-product-standalone: create-standalone-product-dev-acc
    cd product-donation && cargo near deploy {{product_standalone_contract}} without-init-call network-config testnet sign-with-keychain send

test-meta-product-standalone:
    near contract call-function as-read-only {{product_standalone_contract}} contract_source_metadata json-args {} network-config testnet now

show-wasm-hashes:
    near contract download-abi {{factory_contract}} save-to-file {{factory_contract}}.json network-config testnet now
    near contract download-abi {{product_from_factory_contract}} save-to-file {{product_from_factory_contract}}.json network-config testnet now
    near contract download-abi {{product_standalone_contract}} save-to-file {{product_standalone_contract}}.json network-config testnet now
    near contract download-wasm {{factory_contract}} save-to-file {{factory_contract}}.wasm network-config testnet now
    near contract download-wasm {{product_from_factory_contract}} save-to-file {{product_from_factory_contract}}.wasm network-config testnet now
    near contract download-wasm {{product_standalone_contract}} save-to-file {{product_standalone_contract}}.wasm network-config testnet now
    sha256sum *.wasm
