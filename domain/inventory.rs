use crate::{domain::{block::Block, tree::TreeData}, services::inventory::InventoryService};

pub struct InventoryState {
    pub trees: Vec<TreeData>,
    pub stands: Vec<Block>,
}

impl InventoryState {
    pub async fn new(service: InventoryService) -> anyhow::Result<Self> {
        let trees = service.tree_info().await?;
        let stands = service.block_info().await?;

        Ok(Self {
            trees,
            stands,
        })

    }
}
