#![allow(dead_code)]

use crate::*;

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct State {
    // Admin authority
    pub admin: Pubkey,                                   // 32
    pub state_nonce: u8,                                 // 1
    pub serum_nonce: u8,                                 // 1
    pub mint_auth_nonce: u8,                             // 1
    pub num_underlyings: u8,                             // 1
    pub num_flex_underlyings: u8,                        // 1
    pub _null: [u8; 7],                                  // 7
    pub strike_initialization_threshold_seconds: u32,    // 4
    pub pricing_frequency_seconds: u32,                  // 4
    pub liquidator_liquidation_percentage: u32,          // 4
    pub insurance_vault_liquidation_percentage: u32,     // 4
    pub native_d1_trade_fee_percentage: u64,             // 8
    pub native_d1_underlying_fee_percentage: u64,        // 8
    pub native_whitelist_underlying_fee_percentage: u64, // 8
    pub native_deposit_limit: u64,                       // 8
    pub expiration_threshold_seconds: u32,               // 4
    pub position_movement_fee_bps: u8,                   // 1
    pub margin_concession_percentage: u8,                // 1
    pub treasury_wallet_nonce: u8,                       // 1
    pub native_option_trade_fee_percentage: u64,         // 8
    pub native_option_underlying_fee_percentage: u64,    // 8
    pub referrals_admin: Pubkey,                         // 32
    pub referrals_rewards_wallet_nonce: u8,              // 1
    pub max_perp_delta_age: u16,                         // 2
    pub secondary_admin: Pubkey,                         // 32
    pub vault_nonce: u8,                                 // 1
    pub insurance_vault_nonce: u8,                       // 1
    pub deprecated_total_insurance_vault_deposits: u64,  // 8
    pub native_withdraw_limit: u64,                      // 8
    pub withdraw_limit_epoch_seconds: u32,               // 4
    pub native_open_interest_limit: u64,                 // 8
    pub halt_states: [HaltStateV2; MAX_PERP_MARKETS],    // 18 * 5 = 90
    pub _padding: [u8; 338],                             // 338
} // 1000

#[zero_copy(unsafe)]
#[repr(packed)]
pub struct Product {
    // Serum market
    pub market: Pubkey,
    pub strike: Strike,
    // Tracks whether the market has been wiped after expiration
    pub dirty: bool,
    pub kind: Kind,
} // 32 + 9 + 1 + 1 = 43 bytes

#[zero_copy(unsafe)]
#[repr(packed)]
pub struct Strike {
    is_set: bool,
    value: u64,
}

impl Strike {
    pub fn is_set(&self) -> bool {
        self.is_set
    }

    pub fn get_strike(&self) -> Result<u64> {
        if !self.is_set() {
            return Err(error!(ZetaError::ProductStrikeUninitialized));
        }
        Ok(self.value)
    }
}

#[zero_copy(unsafe)]
#[derive(Default)]
#[repr(packed)]
pub struct PricingParameters {
    pub option_trade_normalizer: AnchorDecimal, // 16
    pub future_trade_normalizer: AnchorDecimal, // 16
    pub max_volatility_retreat: AnchorDecimal,  // 16
    pub max_interest_retreat: AnchorDecimal,    // 16
    pub max_delta: u64,                         // 8
    pub min_delta: u64,                         // 8
    pub min_volatility: u64,                    // 8
    pub max_volatility: u64,                    // 8
    pub min_interest_rate: i64,                 // 8
    pub max_interest_rate: i64,                 // 8
} // 112

#[zero_copy(unsafe)]
#[derive(Debug, Default)]
#[repr(packed)]
pub struct MarginParameters {
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
} // 16 bytes.

#[zero_copy(unsafe)]
#[derive(Debug, Default)]
#[repr(packed)]
pub struct PerpParameters {
    pub min_funding_rate_percent: i64, // 8
    pub max_funding_rate_percent: i64, // 8
    pub impact_cash_delta: u64,        // 8
} // 24

#[zero_copy(unsafe)]
#[repr(packed)]
pub struct CrossMarginAccountInfo {
    initialized: bool,                // 1
    name: [u8; ACCOUNT_NAME_MAX_LEN], // 10
} // 11

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct CrossMarginAccountManager {
    pub nonce: u8,                                                     // 1
    pub authority: Pubkey,                                             // 32
    pub accounts: [CrossMarginAccountInfo; MAX_CROSS_MARGIN_ACCOUNTS], // 11 * 25 = 275
} // 308

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct CrossMarginAccount {
    pub authority: Pubkey,                                      // 32
    pub delegated_pubkey: Pubkey,                               // 32
    pub balance: u64,                                           // 8
    pub subaccount_index: u8,                                   // 1
    pub nonce: u8,                                              // 1
    pub force_cancel_flag: bool,                                // 1
    pub account_type: MarginAccountType,                        // 1
    pub open_orders_nonces: [u8; MAX_PERP_MARKETS],             // 5
    pub rebalance_amount: i64,                                  // 8
    pub last_funding_deltas: [AnchorDecimal; MAX_PERP_MARKETS], // 16 * 5 = 80
    pub product_ledgers: [ProductLedger; MAX_PERP_MARKETS],     // 5 * 40 = 305
    pub _padding: [u8; 2000],
} // 3509

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct PerpSyncQueue {
    pub nonce: u8,                   // 1
    pub head: u16,                   // 2
    pub length: u16,                 // 2
    pub queue: [AnchorDecimal; 600], // 16 * 600 = 9600
} // 9605

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct Pricing {
    pub nonce: u8,
    pub mark_prices: [u64; MAX_PERP_MARKETS],
    pub update_timestamps: [u64; MAX_PERP_MARKETS],
    pub funding_deltas: [AnchorDecimal; MAX_PERP_MARKETS],
    pub latest_funding_rates: [AnchorDecimal; MAX_PERP_MARKETS],
    pub latest_midpoints: [u64; MAX_PERP_MARKETS],
    pub oracles: [Pubkey; MAX_PERP_MARKETS],
    pub oracle_backup_feeds: [Pubkey; MAX_PERP_MARKETS],
    pub markets: [Pubkey; MAX_PERP_MARKETS],
    pub perp_sync_queues: [Pubkey; MAX_PERP_MARKETS],
    pub perp_parameters: [PerpParameters; MAX_PERP_MARKETS],
    pub margin_parameters: [MarginParameters; MAX_PERP_MARKETS],
    pub products: [Product; MAX_PERP_MARKETS],
    pub zeta_group_keys: [Pubkey; MAX_PERP_MARKETS],
    pub total_insurance_vault_deposits: u64,
    pub last_withdraw_timestamp: u64,
    pub net_outflow_sum: i64,
    pub halt_force_pricing: [bool; MAX_PERP_MARKETS],
    pub _padding: [u8; 2707],
} // 10232 which is the max for PDAs (incl 8 for discriminator)

#[zero_copy(unsafe)]
#[derive(Default)]
#[repr(packed)]
pub struct AnchorDecimal {
    pub flags: u32,
    pub hi: u32,
    pub lo: u32,
    pub mid: u32,
} // 16

#[account]
#[derive(Default)]
pub struct CrossOpenOrdersMap {
    pub user_key: Pubkey,
    pub subaccount_index: u8,
}

#[account]
#[derive(Default)]
pub struct OpenOrdersMap {
    pub user_key: Pubkey,
}

#[zero_copy(unsafe)]
#[derive(Default)]
#[repr(packed)]
pub struct Position {
    pub size: i64,
    pub cost_of_trades: u64,
} // 16

impl Position {
    pub fn size_abs(&self) -> u64 {
        self.size.abs() as u64
    }

    pub fn get_unrealized_pnl(&self, mark_price: u64) -> i64 {
        if self.size == 0 {
            0
        } else if self.size > 0 {
            (self.size as i128)
                .checked_mul(mark_price as i128)
                .unwrap()
                .checked_div(POSITION_PRECISION_DENOMINATOR as i128)
                .unwrap()
                .checked_sub(self.cost_of_trades as i128)
                .unwrap()
                .try_into()
                .unwrap()
        } else {
            (self.size as i128)
                .checked_mul(mark_price as i128)
                .unwrap()
                .checked_div(POSITION_PRECISION_DENOMINATOR as i128)
                .unwrap()
                .checked_add(self.cost_of_trades as i128)
                .unwrap()
                .try_into()
                .unwrap()
        }
    }
}

#[zero_copy(unsafe)]
#[derive(Default)]
#[repr(packed)]
pub struct OrderState {
    pub closing_orders: u64,
    pub opening_orders: [u64; 2],
} // 24

#[zero_copy(unsafe)]
#[derive(Default)]
#[repr(packed)]
pub struct ProductLedger {
    pub position: Position,
    pub order_state: OrderState,
} // 40

impl ProductLedger {
    pub fn get_initial_margin(
        &self,
        mark_price: u64,
        product: &Product,
        spot: u64,
        margin_parameters: &MarginParameters,
    ) -> u64 {
        let strike: u64 = match product.kind == Kind::Perp {
            true => 0,
            false => match product.strike.get_strike() {
                Ok(strike) => strike,
                Err(_) => return 0,
            },
        };

        let mut long_lots: u64 = self.order_state.opening_orders[BID_ORDERS_INDEX];
        let mut short_lots: u64 = self.order_state.opening_orders[ASK_ORDERS_INDEX];
        if self.position.size > 0 {
            long_lots = long_lots.checked_add(self.position.size_abs()).unwrap();
        } else if self.position.size < 0 {
            short_lots = short_lots.checked_add(self.position.size_abs()).unwrap();
        }

        let mut long_initial_margin: u128 = 0;
        let mut short_initial_margin: u128 = 0;

        if long_lots > 0 {
            long_initial_margin = (long_lots as u128)
                .checked_mul(
                    get_initial_margin_per_lot(
                        spot,
                        strike,
                        mark_price,
                        product.kind,
                        Side::Bid,
                        margin_parameters,
                    )
                    .unwrap()
                    .try_into()
                    .unwrap(),
                )
                .unwrap();
        }

        if short_lots > 0 {
            short_initial_margin = (short_lots as u128)
                .checked_mul(
                    get_initial_margin_per_lot(
                        spot,
                        strike,
                        mark_price,
                        product.kind,
                        Side::Ask,
                        margin_parameters,
                    )
                    .unwrap()
                    .try_into()
                    .unwrap(),
                )
                .unwrap();
        }

        if product.kind == Kind::Future || product.kind == Kind::Perp {
            if long_lots > short_lots {
                return long_initial_margin
                    .checked_div(POSITION_PRECISION_DENOMINATOR)
                    .unwrap()
                    .try_into()
                    .unwrap();
            } else {
                return short_initial_margin
                    .checked_div(POSITION_PRECISION_DENOMINATOR)
                    .unwrap()
                    .try_into()
                    .unwrap();
            }
        }

        long_initial_margin
            .checked_add(short_initial_margin)
            .unwrap()
            .checked_div(POSITION_PRECISION_DENOMINATOR)
            .unwrap()
            .try_into()
            .unwrap()
    }

    pub fn get_maintenance_margin(
        &self,
        mark_price: u64,
        product: &Product,
        spot: u64,
        margin_parameters: &MarginParameters,
    ) -> u64 {
        if self.position.size == 0 {
            return 0;
        }

        let strike: u64 = match product.kind == Kind::Perp {
            true => 0,
            false => match product.strike.get_strike() {
                Ok(strike) => strike,
                Err(_) => return 0,
            },
        };

        let maintenance_margin_per_lot = get_maintenance_margin_per_lot(
            spot,
            strike,
            mark_price,
            product.kind,
            self.position.size >= 0,
            margin_parameters,
        )
        .unwrap();

        (self.position.size_abs() as u128)
            .checked_mul(maintenance_margin_per_lot as u128)
            .unwrap()
            .checked_div(POSITION_PRECISION_DENOMINATOR)
            .unwrap() as u64
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Uninitialized = 0,
    Bid = 1,
    Ask = 2,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum MarginAccountType {
    Normal = 0,
    MarketMaker = 1,
}

#[zero_copy(unsafe)]
#[repr(packed)]
pub struct HaltStateV2 {
    halted: bool,         // 1
    timestamp: u64,       // 8
    spot_price: u64,      // 8
    market_cleaned: bool, // 1
} // 18

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum Kind {
    Uninitialized = 0,
    Call = 1,
    Put = 2,
    Future = 3,
    Perp = 4,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum OrderType {
    Limit = 0,
    PostOnly = 1,
    FillOrKill = 2,
    ImmediateOrCancel = 3,
    PostOnlySlide = 4,
}

// This enum is a direct mapping to the Zeta IDL:
// https://github.com/zetamarkets/sdk/tree/main/src/idl#L9990
#[repr(u8)]
#[derive(Debug, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum Asset {
    Solana = 0,
    Bitcoin = 1,
    Ethereum = 2,
    Aptos = 3,
    Arbitrum = 4,
    BNB = 5,
    Pyth = 6,
    Celestia = 7,
    Jito = 8,
    MillionBonk = 9,
    Sei = 10,
    Jupiter = 11,
    Dym = 12,
    Starknet = 13,
    Dogwifhat = 14,
    Render = 15,
    Tensor = 16,
    UNDEFINED = 255,
}

pub fn asset_from_byte(data: u8) -> Option<Asset> {
    match Asset::try_from_slice(&[data]) {
        Ok(a) => Some(a),
        Err(_) => None,
    }
}

pub fn asset_to_string(asset: Asset) -> String {
    match asset {
        Asset::Solana => "SOL".to_string(),
        Asset::Bitcoin => "BTC".to_string(),
        Asset::Ethereum => "ETH".to_string(),
        Asset::Aptos => "APT".to_string(),
        Asset::Arbitrum => "ARB".to_string(),
        Asset::BNB => "BNB".to_string(),
        Asset::Pyth => "PYTH".to_string(),
        Asset::Celestia => "TIA".to_string(),
        Asset::Jito => "JTO".to_string(),
        Asset::MillionBonk => "BONK".to_string(),
        Asset::Sei => "SEI".to_string(),
        Asset::Jupiter => "JUP".to_string(),
        Asset::Dym => "DYM".to_string(),
        Asset::Starknet => "STRK".to_string(),
        Asset::Dogwifhat => "WIF".to_string(),
        Asset::Render => "RNDR".to_string(),
        Asset::Tensor => "TNSR".to_string(),
        Asset::UNDEFINED => "UNDEFINED".to_string(),
    }
}

pub fn asset_multiplier(asset: Asset) -> u64 {
    match asset {
        Asset::MillionBonk => 1_000_000,
        _ => 1,
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum OrderCompleteType {
    Cancel = 0,
    Fill = 1,
    Booted = 2,
}
