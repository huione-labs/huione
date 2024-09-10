use {
    huione_gossip::cluster_info::ClusterInfo,
    huione_runtime::bank_forks::BankForks,
    huione_sdk::pubkey::Pubkey,
    std::{
        collections::HashSet,
        sync::{Arc, RwLock},
    },
};

#[derive(Clone)]
pub struct AdminRpcRequestMetadataPostInit {
    pub cluster_info: Arc<ClusterInfo>,
    pub bank_forks: Arc<RwLock<BankForks>>,
    pub vote_account: Pubkey,
    pub repair_whitelist: Arc<RwLock<HashSet<Pubkey>>>,
}
