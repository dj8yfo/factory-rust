import? 'local.just'

factory_contract := "repro-fct-14.testnet"

# cleanup in symlinked dir
create-factory-dev-acc:
    near account create-account sponsor-by-faucet-service {{factory_contract}} autogenerate-new-keypair save-to-keychain network-config testnet create

deploy-factory: create-factory-dev-acc
    cd factory && cargo near deploy {{factory_contract}} without-init-call network-config testnet sign-with-keychain send

test-meta-factory:
    near contract call-function as-read-only {{factory_contract}} contract_source_metadata json-args {} network-config testnet now


