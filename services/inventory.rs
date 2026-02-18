use sqlx::PgPool;

use crate::{domain::{block::Block, tree::TreeData}, repositories::{block_repo::BlockRepository, tree_repo::TreeRepository}};

pub struct InventoryService {
    pub block_repo: BlockRepository,
    pub tree_repo: TreeRepository,
}

impl InventoryService {
    pub fn new(pool: PgPool) -> Self {
        let block_repo = BlockRepository::new(pool.clone());
        let tree_repo = TreeRepository::new(pool.clone());

        Self {
            block_repo,
            tree_repo
        }
    }

    pub async fn block_info(&self) -> anyhow::Result<Vec<Block>> {
       let blk_list = self.block_repo.get_all().await?;

        Ok(blk_list)
    }

    pub async fn tree_info(&self) -> anyhow::Result<Vec<TreeData>> {
        let trees = self.tree_repo.get_all().await?;

        Ok(trees)
    }
}
