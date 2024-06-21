/// THIS IS A GENERATED FILE. DO NOT MODIFY MANUALLY.
///
/// This file was auto generated by rindexer - https://github.com/joshstevens19/rindexer.
/// Any manual changes to this file will be overwritten.
use super::erc20_filter_abi_gen::rindexer_erc20_filter_gen::{self, RindexerERC20FilterGen};
use ethers::{
    abi::Address,
    providers::{Http, Provider, RetryClient},
    types::{Bytes, H256},
};
use rindexer_core::{
    async_trait, generate_random_id,
    generator::event_callback_registry::{
        ContractInformation, EventCallbackRegistry, EventInformation, EventResult, FactoryDetails,
        FilterDetails, NetworkContract, TxInformation,
    },
    manifest::yaml::{Contract, ContractDetails},
    provider::JsonRpcCachedProvider,
    AsyncCsvAppender, FutureExt, PostgresClient,
};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::{any::Any, sync::Arc};

pub type ApprovalData = rindexer_erc20_filter_gen::ApprovalFilter;

#[derive(Debug, Clone)]
pub struct ApprovalResult {
    pub event_data: ApprovalData,
    pub tx_information: TxInformation,
}

pub type TransferData = rindexer_erc20_filter_gen::TransferFilter;

#[derive(Debug, Clone)]
pub struct TransferResult {
    pub event_data: TransferData,
    pub tx_information: TxInformation,
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[async_trait]
trait EventCallback {
    async fn call(&self, events: Vec<EventResult>);
}

pub struct EventContext<TExtensions>
where
    TExtensions: Send + Sync,
{
    pub database: Arc<PostgresClient>,
    pub csv: Arc<AsyncCsvAppender>,
    pub extensions: Arc<TExtensions>,
}

// didn't want to use option or none made harder DX
// so a blank struct makes interface nice
pub struct NoExtensions {}
pub fn no_extensions() -> NoExtensions {
    NoExtensions {}
}

pub fn transfer_handler<TExtensions, F, Fut>(
    custom_logic: F,
) -> Arc<
    dyn for<'a> Fn(&'a Vec<TransferResult>, Arc<EventContext<TExtensions>>) -> BoxFuture<'a, ()>
        + Send
        + Sync,
>
where
    TransferResult: Clone + 'static,
    F: for<'a> Fn(Vec<TransferResult>, Arc<EventContext<TExtensions>>) -> Fut
        + Send
        + Sync
        + 'static
        + Clone,
    Fut: Future<Output = ()> + Send + 'static,
    TExtensions: Send + Sync + 'static,
{
    Arc::new(move |results, context| {
        let custom_logic = custom_logic.clone();
        let results = results.clone();
        let context = Arc::clone(&context);
        async move { (custom_logic)(results, context).await }.boxed()
    })
}

type TransferEventCallbackType<TExtensions> = Arc<
    dyn for<'a> Fn(&'a Vec<TransferResult>, Arc<EventContext<TExtensions>>) -> BoxFuture<'a, ()>
        + Send
        + Sync,
>;

pub struct TransferEvent<TExtensions>
where
    TExtensions: Send + Sync + 'static,
{
    callback: TransferEventCallbackType<TExtensions>,
    context: Arc<EventContext<TExtensions>>,
}

impl<TExtensions> TransferEvent<TExtensions>
where
    TExtensions: Send + Sync + 'static,
{
    pub async fn handler<F, Fut>(closure: F, extensions: TExtensions) -> Self
    where
        TransferResult: Clone + 'static,
        F: for<'a> Fn(Vec<TransferResult>, Arc<EventContext<TExtensions>>) -> Fut
            + Send
            + Sync
            + 'static
            + Clone,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let csv = AsyncCsvAppender::new("/Users/joshstevens/code/rindexer/rindexer_demo/./generated_csv/ERC20Filter/erc20filter-transfer.csv".to_string());
        if !Path::new("/Users/joshstevens/code/rindexer/rindexer_demo/./generated_csv/ERC20Filter/erc20filter-transfer.csv").exists() {
            csv.append_header(vec!["contract_address".into(), "from".into(), "to".into(), "value".into(), "tx_hash".into(), "block_number".into(), "block_hash".into(), "network".into()])
                .await
                .unwrap();
        }

        Self {
            callback: transfer_handler(closure),
            context: Arc::new(EventContext {
                database: Arc::new(PostgresClient::new().await.unwrap()),
                csv: Arc::new(csv),
                extensions: Arc::new(extensions),
            }),
        }
    }
}

#[async_trait]
impl<TExtensions> EventCallback for TransferEvent<TExtensions>
where
    TExtensions: Send + Sync,
{
    async fn call(&self, events: Vec<EventResult>) {
        // note some can not downcast because it cant decode
        // this happens on events which failed decoding due to
        // not having the right abi for example
        // transfer events with 2 indexed topics cant decode
        // transfer events with 3 indexed topics
        let result: Vec<TransferResult> = events
            .into_iter()
            .filter_map(|item| {
                item.decoded_data
                    .downcast::<TransferData>()
                    .ok()
                    .map(|arc| TransferResult {
                        event_data: (*arc).clone(),
                        tx_information: item.tx_information,
                    })
            })
            .collect();

        (self.callback)(&result, self.context.clone()).await;
    }
}

pub enum ERC20FilterEventType<TExtensions>
where
    TExtensions: 'static + Send + Sync,
{
    Transfer(TransferEvent<TExtensions>),
}

impl<TExtensions> ERC20FilterEventType<TExtensions>
where
    TExtensions: 'static + Send + Sync,
{
    pub fn topic_id(&self) -> &'static str {
        match self {
            ERC20FilterEventType::Transfer(_) => {
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            }
        }
    }

    pub fn event_name(&self) -> &'static str {
        match self {
            ERC20FilterEventType::Transfer(_) => "Transfer",
        }
    }

    pub fn index_event_in_order(&self) -> bool {
        match self {
            ERC20FilterEventType::Transfer(_) => false,
        }
    }

    pub fn contract_information(&self) -> Contract {
        Contract {
            name: "ERC20Filter".to_string(),
            details: vec![ContractDetails::new_with_filter(
                "polygon".to_string(),
                FilterDetails {
                    event_name: "Transfer".to_string(),
                    indexed_1: None,
                    indexed_2: None,
                    indexed_3: None,
                },
                Some(56399431.into()),
                None,
                Some(1000),
            )],
            abi: "/Users/joshstevens/code/rindexer/rindexer_demo/abis/erc20-abi.json".to_string(),
            include_events: None,
            index_event_in_order: None,
            dependency_events: None,
            reorg_safe_distance: false,
            generate_csv: true,
        }
    }

    fn get_provider(&self, network: &str) -> Arc<JsonRpcCachedProvider> {
        if network == "polygon" {
            super::super::super::networks::get_polygon_provider_cache()
        } else {
            panic!("Network not supported")
        }
    }

    fn contract(&self, network: &str) -> RindexerERC20FilterGen<Arc<Provider<RetryClient<Http>>>> {
        if network == "polygon" {
            let address: Address = "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap();
            RindexerERC20FilterGen::new(
                address,
                Arc::new(self.get_provider(network).get_inner_provider().clone()),
            )
        } else {
            panic!("Network not supported");
        }
    }

    fn decoder(
        &self,
        network: &str,
    ) -> Arc<dyn Fn(Vec<H256>, Bytes) -> Arc<dyn Any + Send + Sync> + Send + Sync> {
        let contract = self.contract(network);

        match self {
            ERC20FilterEventType::Transfer(_) => Arc::new(move |topics: Vec<H256>, data: Bytes| {
                match contract.decode_event::<TransferData>("Transfer", topics, data) {
                    Ok(filter) => Arc::new(filter) as Arc<dyn Any + Send + Sync>,
                    Err(error) => Arc::new(error) as Arc<dyn Any + Send + Sync>,
                }
            }),
        }
    }

    pub fn register(self, registry: &mut EventCallbackRegistry) {
        let topic_id = self.topic_id();
        let event_name = self.event_name();
        let index_event_in_order = self.index_event_in_order();
        let contract_information = self.contract_information();
        let contract = ContractInformation {
            name: contract_information.name,
            details: contract_information
                .details
                .iter()
                .map(|c| NetworkContract {
                    id: generate_random_id(10),
                    network: c.network.clone(),
                    cached_provider: self.get_provider(&c.network),
                    decoder: self.decoder(&c.network),
                    indexing_contract_setup: c.indexing_contract_setup(),
                    start_block: c.start_block,
                    end_block: c.end_block,
                    polling_every: c.polling_every,
                })
                .collect(),
            abi: contract_information.abi,
            reorg_safe_distance: contract_information.reorg_safe_distance,
        };

        let callback: Arc<dyn Fn(Vec<EventResult>) -> BoxFuture<'static, ()> + Send + Sync> =
            match self {
                ERC20FilterEventType::Transfer(event) => {
                    let event = Arc::new(event);
                    Arc::new(move |result| {
                        let event = event.clone();
                        async move { event.call(result).await }.boxed()
                    })
                }
            };

        registry.register_event(EventInformation {
            indexer_name: "BlahBaby".to_string(),
            event_name: event_name.to_string(),
            index_event_in_order,
            topic_id: topic_id.to_string(),
            contract,
            callback,
        });
    }
}
