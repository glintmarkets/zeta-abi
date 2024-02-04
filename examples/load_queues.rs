use std::borrow::Cow;

use anchor_lang::{AnchorDeserialize, Owner, ZeroCopy};
use anchor_spl::dex::serum_dex::state::{
    Event, EventQueueHeader, MarketStateV2, QueueHeader, ACCOUNT_HEAD_PADDING, ACCOUNT_TAIL_PADDING,
};
use arrayref::array_ref;
use bytemuck::{bytes_of, from_bytes, Pod};
use safe_transmute::{
    guard::SingleManyGuard, to_bytes::transmute_to_bytes, transmute_many, transmute_many_pedantic,
    transmute_one_pedantic,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use zeta_abi::{Asset, Pricing, ACTIVE_PERP_MARKETS, MAX_PERP_MARKETS};

pub fn derive_pricing_address() -> (Pubkey, u8) {
    let (address, bump) = Pubkey::find_program_address(&["pricing".as_ref()], &zeta_abi::id());

    (address, bump)
}

pub fn get_zero_copy_account<T: ZeroCopy + Owner>(account_data: &[u8]) -> Box<T> {
    let disc_bytes = array_ref![account_data, 0, 8];
    assert_eq!(disc_bytes, &T::discriminator());
    Box::new(*from_bytes::<T>(
        &account_data[8..std::mem::size_of::<T>() + 8],
    ))
}

pub fn remove_dex_account_padding<'a>(data: &'a [u8]) -> Cow<'a, [u64]> {
    let head = &data[..ACCOUNT_HEAD_PADDING.len()];
    if data.len() < ACCOUNT_HEAD_PADDING.len() + ACCOUNT_TAIL_PADDING.len() {
        panic!();
    }
    if head != ACCOUNT_HEAD_PADDING {
        panic!();
    }
    let tail = &data[data.len() - ACCOUNT_TAIL_PADDING.len()..];
    if tail != ACCOUNT_TAIL_PADDING {
        panic!();
    }
    let inner_data_range = ACCOUNT_HEAD_PADDING.len()..(data.len() - ACCOUNT_TAIL_PADDING.len());
    let inner: &'a [u8] = &data[inner_data_range];
    let words: Cow<'a, [u64]> = match transmute_many_pedantic::<u64>(inner) {
        Ok(word_slice) => Cow::Borrowed(word_slice),
        Err(transmute_error) => {
            let word_vec = transmute_error.copy().map_err(|e| e.without_src()).unwrap();
            Cow::Owned(word_vec)
        }
    };
    words
}

pub fn parse_dex_event_queue(data_words: &[u64]) -> (EventQueueHeader, &[Event], &[Event]) {
    let (header_words, event_words) =
        data_words.split_at(std::mem::size_of::<EventQueueHeader>() >> 3);
    let header: EventQueueHeader = transmute_one_pedantic(transmute_to_bytes(header_words))
        .map_err(|e| e.without_src())
        .unwrap();
    let events: &[Event] = transmute_many::<_, SingleManyGuard>(transmute_to_bytes(event_words))
        .map_err(|e| e.without_src())
        .unwrap();
    let (tail_seg, head_seg) = events.split_at(header.head() as usize);
    let head_len = head_seg.len().min(header.count() as usize);
    let tail_len = header.count() as usize - head_len;
    (header, &head_seg[..head_len], &tail_seg[..tail_len])
}

pub fn parse_dex_account<T: Pod>(data: &[u8]) -> T {
    let data_len = data.len() - 12;
    let (_, rest) = data.split_at(5);
    let (mid, _) = rest.split_at(data_len);
    *from_bytes(mid)
}

#[tokio::main]
async fn main() {
    let rpc_client = RpcClient::new("RPC_URL".to_string());

    let (pricing_pubkey, _) = derive_pricing_address();
    let pricing_account = rpc_client.get_account(&pricing_pubkey).await.unwrap();
    let pricing = get_zero_copy_account::<Pricing>(&pricing_account.data);

    let mark_prices = pricing.mark_prices;
    let markets = pricing.markets;

    for i in 0..ACTIVE_PERP_MARKETS {
        let asset = Asset::try_from_slice(&[i as u8]).unwrap();
        println!("{:?} {} {}", asset, markets[i], mark_prices[i]);
    }

    let initialized_markets = markets
        .iter()
        .filter(|m| m != &&Pubkey::default())
        .map(|m| m.clone())
        .collect::<Vec<Pubkey>>();

    let market_accounts = rpc_client
        .get_multiple_accounts(&initialized_markets)
        .await
        .unwrap();

    let mut event_queue_pubkeys = Vec::new();

    for market_account in market_accounts.iter() {
        if let Some(account) = market_account {
            let serum_market = parse_dex_account::<MarketStateV2>(&account.data);
            let eq = serum_market.event_q;
            event_queue_pubkeys.push(Pubkey::from(array_ref![bytes_of(&eq), 0, 32].clone()));
        }
    }

    let event_queues = rpc_client
        .get_multiple_accounts(&event_queue_pubkeys)
        .await
        .unwrap();

    for event_queue_account in event_queues.iter() {
        if let Some(account) = event_queue_account {
            let data = remove_dex_account_padding(&account.data);
            let (header, seg0, seg1) = parse_dex_event_queue(&data);

            println!(
                "h {} c {} s0 {} s1 {}",
                header.head(),
                header.count(),
                seg0.len(),
                seg1.len()
            );
        }
    }
}
