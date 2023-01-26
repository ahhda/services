use crate::domain::eth;

pub mod balancer;
pub mod swapr;
pub mod uniswap;
pub mod zeroex;

/// A source of liquidity which can be used by the solver.
#[derive(Debug, Clone)]
pub struct Liquidity {
    pub id: Id,
    /// Estimation of gas needed to use this liquidity on-chain.
    pub gas: eth::Gas,
    pub kind: Kind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(pub usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Id> for usize {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl PartialEq<usize> for Id {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

/// A limit input amount.
#[derive(Clone, Copy, Debug)]
pub struct MaxInput(pub eth::Asset);

/// An exact output amount.
#[derive(Clone, Copy, Debug)]
pub struct ExactOutput(pub eth::Asset);

/// Data tied to a particular liquidity instance, specific to the kind of
/// liquidity.
///
/// This contains relevant data for encoding interactions for the given
/// liquidity, as well as state required by the solver engine.
#[derive(Debug, Clone)]
pub enum Kind {
    UniswapV2(uniswap::v2::Pool),
    UniswapV3(uniswap::v3::Pool),
    BalancerV2Stable(balancer::stable::Pool),
    BalancerV2Weighted(balancer::weighted::Pool),
    Swapr(swapr::Pool),
    ZeroEx(zeroex::LimitOrder),
}