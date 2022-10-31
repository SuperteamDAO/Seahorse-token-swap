#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct NormalMintReserve {
    pub premium_mint_reserve_acc: Pubkey,
    pub normal_mint: Pubkey,
    pub normal_token_account: Pubkey,
    pub go_live_ts: i64,
    pub initialization_ts: i64,
    pub bump: u8,
    pub token_bump: u8,
}

impl<'info, 'entrypoint> NormalMintReserve {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedNormalMintReserve<'info, 'entrypoint>> {
        let premium_mint_reserve_acc = account.premium_mint_reserve_acc.clone();
        let normal_mint = account.normal_mint.clone();
        let normal_token_account = account.normal_token_account.clone();
        let go_live_ts = account.go_live_ts;
        let initialization_ts = account.initialization_ts;
        let bump = account.bump;
        let token_bump = account.token_bump;

        Mutable::new(LoadedNormalMintReserve {
            __account__: account,
            __programs__: programs_map,
            premium_mint_reserve_acc,
            normal_mint,
            normal_token_account,
            go_live_ts,
            initialization_ts,
            bump,
            token_bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedNormalMintReserve>) {
        let mut loaded = loaded.borrow_mut();
        let premium_mint_reserve_acc = loaded.premium_mint_reserve_acc.clone();

        loaded.__account__.premium_mint_reserve_acc = premium_mint_reserve_acc;

        let normal_mint = loaded.normal_mint.clone();

        loaded.__account__.normal_mint = normal_mint;

        let normal_token_account = loaded.normal_token_account.clone();

        loaded.__account__.normal_token_account = normal_token_account;

        let go_live_ts = loaded.go_live_ts;

        loaded.__account__.go_live_ts = go_live_ts;

        let initialization_ts = loaded.initialization_ts;

        loaded.__account__.initialization_ts = initialization_ts;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;

        let token_bump = loaded.token_bump;

        loaded.__account__.token_bump = token_bump;
    }
}

#[derive(Debug)]
pub struct LoadedNormalMintReserve<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, NormalMintReserve>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub premium_mint_reserve_acc: Pubkey,
    pub normal_mint: Pubkey,
    pub normal_token_account: Pubkey,
    pub go_live_ts: i64,
    pub initialization_ts: i64,
    pub bump: u8,
    pub token_bump: u8,
}

#[account]
#[derive(Debug)]
pub struct PremiumMintReserve {
    pub premium_mint: Pubkey,
    pub premium_account: Pubkey,
    pub go_live_timestamp: i64,
    pub initialization_timestamp: i64,
    pub normal_mints: u32,
    pub authority: Pubkey,
    pub seed_string: String,
    pub bump: u8,
    pub token_bump: u8,
}

impl<'info, 'entrypoint> PremiumMintReserve {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedPremiumMintReserve<'info, 'entrypoint>> {
        let premium_mint = account.premium_mint.clone();
        let premium_account = account.premium_account.clone();
        let go_live_timestamp = account.go_live_timestamp;
        let initialization_timestamp = account.initialization_timestamp;
        let normal_mints = account.normal_mints;
        let authority = account.authority.clone();
        let seed_string = account.seed_string.clone();
        let bump = account.bump;
        let token_bump = account.token_bump;

        Mutable::new(LoadedPremiumMintReserve {
            __account__: account,
            __programs__: programs_map,
            premium_mint,
            premium_account,
            go_live_timestamp,
            initialization_timestamp,
            normal_mints,
            authority,
            seed_string,
            bump,
            token_bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedPremiumMintReserve>) {
        let mut loaded = loaded.borrow_mut();
        let premium_mint = loaded.premium_mint.clone();

        loaded.__account__.premium_mint = premium_mint;

        let premium_account = loaded.premium_account.clone();

        loaded.__account__.premium_account = premium_account;

        let go_live_timestamp = loaded.go_live_timestamp;

        loaded.__account__.go_live_timestamp = go_live_timestamp;

        let initialization_timestamp = loaded.initialization_timestamp;

        loaded.__account__.initialization_timestamp = initialization_timestamp;

        let normal_mints = loaded.normal_mints;

        loaded.__account__.normal_mints = normal_mints;

        let authority = loaded.authority.clone();

        loaded.__account__.authority = authority;

        let seed_string = loaded.seed_string.clone();

        loaded.__account__.seed_string = seed_string;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;

        let token_bump = loaded.token_bump;

        loaded.__account__.token_bump = token_bump;
    }
}

#[derive(Debug)]
pub struct LoadedPremiumMintReserve<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, PremiumMintReserve>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub premium_mint: Pubkey,
    pub premium_account: Pubkey,
    pub go_live_timestamp: i64,
    pub initialization_timestamp: i64,
    pub normal_mints: u32,
    pub authority: Pubkey,
    pub seed_string: String,
    pub bump: u8,
    pub token_bump: u8,
}

pub fn create_normal_mint_reserve_handler<'info>(
    mut auth: SeahorseSigner<'info, '_>,
    mut normal_token_account: Empty<SeahorseAccount<'info, '_, TokenAccount>>,
    mut normal_mint_reserve_acc: Empty<Mutable<LoadedNormalMintReserve<'info, '_>>>,
    mut premium_mint_reserve_acc: Mutable<LoadedPremiumMintReserve<'info, '_>>,
    mut normal_mint: SeahorseAccount<'info, '_, Mint>,
    mut go_live_ts: i64,
    mut initialization_ts: i64,
    mut clock: Sysvar<'info, Clock>,
) -> () {
    let mut bump = normal_mint_reserve_acc.bump.unwrap();
    let mut normal_mint_reserve_acc = normal_mint_reserve_acc.account.clone();
    let mut token_bump = normal_token_account.bump.unwrap();
    let mut normal_token_account = normal_token_account.account.clone();

    if !(premium_mint_reserve_acc.borrow().premium_mint != normal_mint.key()) {
        panic!("premium mint can't be the same as normal mint");
    }

    if !(premium_mint_reserve_acc.borrow().authority == auth.key()) {
        panic!("authorities do not match");
    }

    assign!(
        normal_mint_reserve_acc
            .borrow_mut()
            .premium_mint_reserve_acc,
        premium_mint_reserve_acc.borrow().__account__.key()
    );

    assign!(
        normal_mint_reserve_acc.borrow_mut().normal_mint,
        normal_mint.key()
    );

    assign!(
        normal_mint_reserve_acc.borrow_mut().normal_token_account,
        normal_token_account.key()
    );

    assign!(
        normal_mint_reserve_acc.borrow_mut().initialization_ts,
        initialization_ts
    );

    assign!(normal_mint_reserve_acc.borrow_mut().bump, bump);

    assign!(normal_mint_reserve_acc.borrow_mut().token_bump, token_bump);

    if go_live_ts < clock.unix_timestamp {
        assign!(
            normal_mint_reserve_acc.borrow_mut().go_live_ts,
            clock.unix_timestamp
        );
    } else {
        assign!(normal_mint_reserve_acc.borrow_mut().go_live_ts, go_live_ts);
    }
}

pub fn swap_premium_tokens_for_normal_tokens_handler<'info>(
    mut source_authority: SeahorseSigner<'info, '_>,
    mut premium_mint_reserve_acc: Mutable<LoadedPremiumMintReserve<'info, '_>>,
    mut normal_mint_reserve_acc: Mutable<LoadedNormalMintReserve<'info, '_>>,
    mut normal_token_account: SeahorseAccount<'info, '_, TokenAccount>,
    mut premium_account: SeahorseAccount<'info, '_, TokenAccount>,
    mut source: SeahorseAccount<'info, '_, TokenAccount>,
    mut destination: SeahorseAccount<'info, '_, TokenAccount>,
    mut clock: Sysvar<'info, Clock>,
    mut amount: u64,
) -> () {
    if !(premium_mint_reserve_acc.borrow().premium_account == premium_account.key()) {
        panic!("Invalid premium token account");
    }

    if !(normal_mint_reserve_acc.borrow().premium_mint_reserve_acc
        == premium_mint_reserve_acc.borrow().__account__.key())
    {
        panic!("The premium and the normal reserves are not related");
    }

    if !(normal_mint_reserve_acc.borrow().normal_token_account == normal_token_account.key()) {
        panic!("Invalid normal token account");
    }

    if !(premium_mint_reserve_acc.borrow().go_live_timestamp < clock.unix_timestamp) {
        panic!("Premium reserve not live yet");
    }

    if !(normal_mint_reserve_acc.borrow().go_live_ts < clock.unix_timestamp) {
        panic!("Normal reserve not live yet");
    }

    let mut normal_amount = normal_token_account.amount;

    if !(normal_amount >= amount) {
        panic!("Token amount too low to swap");
    }

    token::transfer(
        CpiContext::new(
            source.programs.get("token_program"),
            token::Transfer {
                from: source.to_account_info(),
                authority: source_authority.to_account_info(),
                to: premium_account.to_account_info(),
            },
        ),
        amount,
    )
    .unwrap();

    token::transfer(
        CpiContext::new(
            normal_token_account.programs.get("token_program"),
            token::Transfer {
                from: normal_token_account.to_account_info(),
                authority: normal_mint_reserve_acc
                    .borrow()
                    .__account__
                    .to_account_info(),
                to: destination.to_account_info(),
            },
        ),
        amount,
    )
    .unwrap();
}

pub fn withdraw_premium_tokens_handler<'info>(
    mut authority: SeahorseSigner<'info, '_>,
    mut premium_mint_reserve_acc: Mutable<LoadedPremiumMintReserve<'info, '_>>,
    mut premium_account: SeahorseAccount<'info, '_, TokenAccount>,
    mut destination: SeahorseAccount<'info, '_, TokenAccount>,
    mut amount: u64,
) -> () {
    if !(premium_mint_reserve_acc.borrow().authority == authority.key()) {
        panic!("Invalid Authority");
    }

    if !(premium_mint_reserve_acc.borrow().premium_account == premium_account.key()) {
        panic!("Invalid Premium token account");
    }

    token::transfer(
        CpiContext::new(
            premium_account.programs.get("token_program"),
            token::Transfer {
                from: premium_account.to_account_info(),
                authority: premium_mint_reserve_acc
                    .borrow()
                    .__account__
                    .to_account_info(),
                to: destination.to_account_info(),
            },
        ),
        amount,
    )
    .unwrap();
}

pub fn withdraw_normal_tokens_handler<'info>(
    mut authority: SeahorseSigner<'info, '_>,
    mut premium_mint_reserve_acc: Mutable<LoadedPremiumMintReserve<'info, '_>>,
    mut normal_mint_reserve_acc: Mutable<LoadedNormalMintReserve<'info, '_>>,
    mut normal_token_account: SeahorseAccount<'info, '_, TokenAccount>,
    mut destination: SeahorseAccount<'info, '_, TokenAccount>,
    mut amount: u64,
) -> () {
    if !(premium_mint_reserve_acc.borrow().authority == authority.key()) {
        panic!("Invalid authority");
    }

    if !(normal_mint_reserve_acc.borrow().premium_mint_reserve_acc
        == premium_mint_reserve_acc.borrow().__account__.key())
    {
        panic!("The normal reserve and the premium reserve are not related");
    }

    if !(normal_mint_reserve_acc.borrow().normal_token_account == normal_token_account.key()) {
        panic!("Invalid normal_token account");
    }

    token::transfer(
        CpiContext::new(
            normal_token_account.programs.get("token_program"),
            token::Transfer {
                from: normal_token_account.to_account_info(),
                authority: normal_mint_reserve_acc
                    .borrow()
                    .__account__
                    .to_account_info(),
                to: destination.to_account_info(),
            },
        ),
        amount,
    )
    .unwrap();
}

pub fn create_premium_mint_reserve_handler<'info>(
    mut auth: SeahorseSigner<'info, '_>,
    mut premium_mint_reserve_acc: Empty<Mutable<LoadedPremiumMintReserve<'info, '_>>>,
    mut premium_mint: SeahorseAccount<'info, '_, Mint>,
    mut premium_account: Empty<SeahorseAccount<'info, '_, TokenAccount>>,
    mut go_live_timestamp: i64,
    mut clock: Sysvar<'info, Clock>,
    mut normal_mints: u32,
    mut seed_string: String,
) -> () {
    let mut bump = premium_mint_reserve_acc.bump.unwrap();
    let mut premium_mint_reserve_acc = premium_mint_reserve_acc.account.clone();
    let mut token_bump = premium_account.bump.unwrap();
    let mut premium_account = premium_account.account.clone();

    assign!(
        premium_mint_reserve_acc.borrow_mut().premium_mint,
        premium_mint.key()
    );

    assign!(
        premium_mint_reserve_acc.borrow_mut().premium_account,
        premium_account.key()
    );

    assign!(
        premium_mint_reserve_acc
            .borrow_mut()
            .initialization_timestamp,
        clock.unix_timestamp
    );

    assign!(
        premium_mint_reserve_acc.borrow_mut().seed_string,
        seed_string
    );

    assign!(premium_mint_reserve_acc.borrow_mut().bump, bump);

    assign!(premium_mint_reserve_acc.borrow_mut().token_bump, token_bump);

    assign!(
        premium_mint_reserve_acc.borrow_mut().normal_mints,
        normal_mints
    );

    if go_live_timestamp < clock.unix_timestamp {
        assign!(
            premium_mint_reserve_acc.borrow_mut().go_live_timestamp,
            clock.unix_timestamp
        );
    } else {
        assign!(
            premium_mint_reserve_acc.borrow_mut().go_live_timestamp,
            go_live_timestamp
        );
    }
}

pub fn swap_normal_tokens_for_premium_tokens_handler<'info>(
    mut source_authority: SeahorseSigner<'info, '_>,
    mut premium_mint_reserve_acc: Mutable<LoadedPremiumMintReserve<'info, '_>>,
    mut normal_mint_reserve_acc: Mutable<LoadedNormalMintReserve<'info, '_>>,
    mut normal_token_account: SeahorseAccount<'info, '_, TokenAccount>,
    mut premium_account: SeahorseAccount<'info, '_, TokenAccount>,
    mut source: SeahorseAccount<'info, '_, TokenAccount>,
    mut destination: SeahorseAccount<'info, '_, TokenAccount>,
    mut clock: Sysvar<'info, Clock>,
    mut amount: u64,
) -> () {
    if !(premium_mint_reserve_acc.borrow().premium_account == premium_account.key()) {
        panic!("Invalid premium token account");
    }

    if !(normal_mint_reserve_acc.borrow().premium_mint_reserve_acc
        == premium_mint_reserve_acc.borrow().__account__.key())
    {
        panic!("The premium and normal reserves are not related");
    }

    if !(normal_mint_reserve_acc.borrow().normal_token_account == normal_token_account.key()) {
        panic!("Invalid child_storage");
    }

    if !(premium_mint_reserve_acc.borrow().go_live_timestamp < clock.unix_timestamp) {
        panic!("Premium reserve not live yet");
    }

    if !(normal_mint_reserve_acc.borrow().go_live_ts < clock.unix_timestamp) {
        panic!("Normal reserve not live yet");
    }

    let mut premium_amount = premium_account.amount;

    if !(premium_amount >= amount) {
        panic!("Token amount too low to swap");
    }

    token::transfer(
        CpiContext::new(
            source.programs.get("token_program"),
            token::Transfer {
                from: source.to_account_info(),
                authority: source_authority.to_account_info(),
                to: normal_token_account.to_account_info(),
            },
        ),
        amount,
    )
    .unwrap();

    token::transfer(
        CpiContext::new(
            premium_account.programs.get("token_program"),
            token::Transfer {
                from: premium_account.to_account_info(),
                authority: premium_mint_reserve_acc
                    .borrow()
                    .__account__
                    .to_account_info(),
                to: destination.to_account_info(),
            },
        ),
        amount,
    )
    .unwrap();
}
