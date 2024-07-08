use crate::manifest::yaml::ContractDetails;
use crate::types::code::Code;
use crate::{
    helpers::camel_to_snake,
    manifest::yaml::{Contract, Network},
};
use ethers::addressbook::Address;

use super::networks_bindings::network_provider_fn_name;

fn generate_contract_code(
    contract_name: &str,
    contract_details: &ContractDetails,
    abi_location: &str,
    network: &Network,
) -> Code {
    if let Some(address) = contract_details.address() {
        let code = format!(
            r#"
            abigen!({contract_name}, "{contract_path}");

            pub fn {contract_fn_name}_contract() -> {contract_name}<Arc<Provider<RetryClient<Http>>>> {{
                let address: Address = "{contract_address}"
                .parse()
                .unwrap();

                {contract_name}::new(address, Arc::new({network_fn_name}().clone()))
            }}
        "#,
            contract_name = contract_name,
            contract_fn_name = camel_to_snake(contract_name),
            // TODO - FIX THIS
            //contract_address = address,
            contract_address = Address::zero(),
            network_fn_name = network_provider_fn_name(network),
            contract_path = abi_location
        );
        Code::new(code)
    } else {
        Code::blank()
    }
}

fn generate_contracts_code(contracts: &[Contract], networks: &[Network]) -> Code {
    let network_imports: Vec<String> = networks.iter().map(network_provider_fn_name).collect();
    let mut output = Code::new(format!(
        r#"
        /// THIS IS A GENERATED FILE. DO NOT MODIFY MANUALLY.
        ///
        /// This file was auto generated by rindexer - https://github.com/joshstevens19/rindexer.
        /// Any manual changes to this file will be overwritten.
        
        use super::networks::{{{}}};
        use std::sync::Arc;
        use ethers::{{contract::abigen, abi::Address, providers::{{Provider, Http, RetryClient}}}};
        "#,
        network_imports.join(", ")
    ));

    let mut code = Code::blank();

    for contract in contracts {
        for details in &contract.details {
            if let Some(network) = networks.iter().find(|&n| n.name == details.network) {
                code.push_str(&generate_contract_code(
                    &contract.name,
                    details,
                    &contract.abi,
                    network,
                ));
            }
        }
    }

    output.push_str(&code);

    output
}

pub fn generate_context_code(contracts: &Option<Vec<Contract>>, networks: &[Network]) -> Code {
    if let Some(contracts) = contracts {
        generate_contracts_code(contracts, networks)
    } else {
        Code::blank()
    }
}
