mod arguments;
mod cached;
mod simulation;
mod web3;

use {
    anyhow::Result,
    model::{
        interaction::InteractionData,
        order::{Order, SellTokenSource},
    },
    primitive_types::{H160, U256},
};

pub use self::{
    arguments::{Arguments, Contracts, Strategy},
    cached::CachingBalanceFetcher,
    simulation::Balances as SimulationBalanceFetcher,
    web3::Web3BalanceFetcher,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Query {
    pub owner: H160,
    pub token: H160,
    pub source: SellTokenSource,
    pub interactions: Vec<InteractionData>,
}

impl Query {
    pub fn from_order(o: &Order) -> Self {
        Self {
            owner: o.metadata.owner,
            token: o.data.sell_token,
            source: o.data.sell_token_balance,
            interactions: o.interactions.pre.clone(),
        }
    }
}

#[derive(Debug)]
pub enum TransferSimulationError {
    InsufficientAllowance,
    InsufficientBalance,
    TransferFailed,
    Other(anyhow::Error),
}

impl From<anyhow::Error> for TransferSimulationError {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err)
    }
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait BalanceFetching: Send + Sync {
    // Returns the balance available to the allowance manager for the given owner
    // and token taking both balance as well as "allowance" into account.
    async fn get_balances(&self, queries: &[Query]) -> Vec<Result<U256>>;

    // Check that the settlement contract can make use of this user's token balance.
    // This check could fail if the user does not have enough balance, has not
    // given the allowance to the allowance manager or if the token does not
    // allow freely transferring amounts around for for example if it is paused
    // or takes a fee on transfer. If the node supports the trace_callMany we
    // can perform more extensive tests.
    async fn can_transfer(
        &self,
        query: &Query,
        amount: U256,
    ) -> Result<(), TransferSimulationError>;
}
