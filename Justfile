import? 'local.just'

factory_contract := "repro-fct-15.testnet"
child_deploy_signer := "child-deploy-signer-4.testnet"
product_contract_name := "donation-product"
product_from_factory_contract := product_contract_name + "." + factory_contract
product_standalone_contract := "repro-fct-product-14.testnet"
factory_call_payload := "{ \"name\": \"" + product_contract_name + "\", \"beneficiary\": \"donatello2.testnet\"}"

create-factory-dev-acc:
    near account create-account sponsor-by-faucet-service {{factory_contract}} autogenerate-new-keypair save-to-keychain network-config testnet create

deploy-factory: create-factory-dev-acc
    cd factory && cargo near deploy {{factory_contract}} without-init-call network-config testnet sign-with-keychain send

test-meta-factory:
    near contract call-function as-read-only {{factory_contract}} contract_source_metadata json-args {} network-config testnet now

create-child-deploy-signer-acc:
    near account create-account sponsor-by-faucet-service {{child_deploy_signer}} autogenerate-new-keypair save-to-keychain network-config testnet create

deploy-from-factory: create-child-deploy-signer-acc
    sleep 30
    near contract call-function as-transaction {{factory_contract}} create_factory_subaccount_and_deploy json-args '{{factory_call_payload}}' prepaid-gas '300.0 Tgas' attached-deposit '1.7 NEAR' sign-as {{child_deploy_signer}} network-config testnet sign-with-keychain send

test-meta-product:
    near contract call-function as-read-only {{product_from_factory_contract}} contract_source_metadata json-args {} network-config testnet now

create-standalone-product-dev-acc:
    near account create-account sponsor-by-faucet-service {{product_standalone_contract}} autogenerate-new-keypair save-to-keychain network-config testnet create

deploy-product-standalone: create-standalone-product-dev-acc
    cd product-donation && cargo near deploy {{product_standalone_contract}} without-init-call network-config testnet sign-with-keychain send

test-meta-product-standalone:
    near contract call-function as-read-only {{product_standalone_contract}} contract_source_metadata json-args {} network-config testnet now

