#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod seahorse_util {
    use super::*;
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index > 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index > 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }
}

#[program]
mod token_swap {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    # [instruction (amount : u64)]
    pub struct SwapPremiumTokensForNormalTokens<'info> {
        #[account(mut)]
        pub source_authority: Signer<'info>,
        #[account(mut)]
        pub premium_mint_reserve_acc: Box<Account<'info, dot::program::PremiumMintReserve>>,
        #[account(mut)]
        pub normal_mint_reserve_acc: Box<Account<'info, dot::program::NormalMintReserve>>,
        #[account(mut)]
        pub normal_token_account: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub premium_account: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub source: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub destination: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub clock: Sysvar<'info, Clock>,
        pub token_program: Program<'info, Token>,
    }

    pub fn swap_premium_tokens_for_normal_tokens(
        ctx: Context<SwapPremiumTokensForNormalTokens>,
        amount: u64,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let source_authority = SeahorseSigner {
            account: &ctx.accounts.source_authority,
            programs: &programs_map,
        };

        let premium_mint_reserve_acc = dot::program::PremiumMintReserve::load(
            &mut ctx.accounts.premium_mint_reserve_acc,
            &programs_map,
        );

        let normal_mint_reserve_acc = dot::program::NormalMintReserve::load(
            &mut ctx.accounts.normal_mint_reserve_acc,
            &programs_map,
        );

        let normal_token_account = SeahorseAccount {
            account: &ctx.accounts.normal_token_account,
            programs: &programs_map,
        };

        let premium_account = SeahorseAccount {
            account: &ctx.accounts.premium_account,
            programs: &programs_map,
        };

        let source = SeahorseAccount {
            account: &ctx.accounts.source,
            programs: &programs_map,
        };

        let destination = SeahorseAccount {
            account: &ctx.accounts.destination,
            programs: &programs_map,
        };

        let clock = &ctx.accounts.clock.clone();

        swap_premium_tokens_for_normal_tokens_handler(
            source_authority.clone(),
            premium_mint_reserve_acc.clone(),
            normal_mint_reserve_acc.clone(),
            normal_token_account.clone(),
            premium_account.clone(),
            source.clone(),
            destination.clone(),
            clock.clone(),
            amount,
        );

        dot::program::PremiumMintReserve::store(premium_mint_reserve_acc);

        dot::program::NormalMintReserve::store(normal_mint_reserve_acc);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (amount : u64)]
    pub struct WithdrawNormalTokens<'info> {
        #[account(mut)]
        pub authority: Signer<'info>,
        #[account(mut)]
        pub premium_mint_reserve_acc: Box<Account<'info, dot::program::PremiumMintReserve>>,
        #[account(mut)]
        pub normal_mint_reserve_acc: Box<Account<'info, dot::program::NormalMintReserve>>,
        #[account(mut)]
        pub normal_token_account: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub destination: Box<Account<'info, TokenAccount>>,
        pub token_program: Program<'info, Token>,
    }

    pub fn withdraw_normal_tokens(ctx: Context<WithdrawNormalTokens>, amount: u64) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let authority = SeahorseSigner {
            account: &ctx.accounts.authority,
            programs: &programs_map,
        };

        let premium_mint_reserve_acc = dot::program::PremiumMintReserve::load(
            &mut ctx.accounts.premium_mint_reserve_acc,
            &programs_map,
        );

        let normal_mint_reserve_acc = dot::program::NormalMintReserve::load(
            &mut ctx.accounts.normal_mint_reserve_acc,
            &programs_map,
        );

        let normal_token_account = SeahorseAccount {
            account: &ctx.accounts.normal_token_account,
            programs: &programs_map,
        };

        let destination = SeahorseAccount {
            account: &ctx.accounts.destination,
            programs: &programs_map,
        };

        withdraw_normal_tokens_handler(
            authority.clone(),
            premium_mint_reserve_acc.clone(),
            normal_mint_reserve_acc.clone(),
            normal_token_account.clone(),
            destination.clone(),
            amount,
        );

        dot::program::PremiumMintReserve::store(premium_mint_reserve_acc);

        dot::program::NormalMintReserve::store(normal_mint_reserve_acc);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (amount : u64)]
    pub struct SwapNormalTokensForPremiumTokens<'info> {
        #[account(mut)]
        pub source_authority: Signer<'info>,
        #[account(mut)]
        pub premium_mint_reserve_acc: Box<Account<'info, dot::program::PremiumMintReserve>>,
        #[account(mut)]
        pub normal_mint_reserve_acc: Box<Account<'info, dot::program::NormalMintReserve>>,
        #[account(mut)]
        pub normal_token_account: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub premium_account: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub source: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub destination: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub clock: Sysvar<'info, Clock>,
        pub token_program: Program<'info, Token>,
    }

    pub fn swap_normal_tokens_for_premium_tokens(
        ctx: Context<SwapNormalTokensForPremiumTokens>,
        amount: u64,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let source_authority = SeahorseSigner {
            account: &ctx.accounts.source_authority,
            programs: &programs_map,
        };

        let premium_mint_reserve_acc = dot::program::PremiumMintReserve::load(
            &mut ctx.accounts.premium_mint_reserve_acc,
            &programs_map,
        );

        let normal_mint_reserve_acc = dot::program::NormalMintReserve::load(
            &mut ctx.accounts.normal_mint_reserve_acc,
            &programs_map,
        );

        let normal_token_account = SeahorseAccount {
            account: &ctx.accounts.normal_token_account,
            programs: &programs_map,
        };

        let premium_account = SeahorseAccount {
            account: &ctx.accounts.premium_account,
            programs: &programs_map,
        };

        let source = SeahorseAccount {
            account: &ctx.accounts.source,
            programs: &programs_map,
        };

        let destination = SeahorseAccount {
            account: &ctx.accounts.destination,
            programs: &programs_map,
        };

        let clock = &ctx.accounts.clock.clone();

        swap_normal_tokens_for_premium_tokens_handler(
            source_authority.clone(),
            premium_mint_reserve_acc.clone(),
            normal_mint_reserve_acc.clone(),
            normal_token_account.clone(),
            premium_account.clone(),
            source.clone(),
            destination.clone(),
            clock.clone(),
            amount,
        );

        dot::program::PremiumMintReserve::store(premium_mint_reserve_acc);

        dot::program::NormalMintReserve::store(normal_mint_reserve_acc);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (amount : u64)]
    pub struct WithdrawPremiumTokens<'info> {
        #[account(mut)]
        pub authority: Signer<'info>,
        #[account(mut)]
        pub premium_mint_reserve_acc: Box<Account<'info, dot::program::PremiumMintReserve>>,
        #[account(mut)]
        pub premium_account: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub destination: Box<Account<'info, TokenAccount>>,
        pub token_program: Program<'info, Token>,
    }

    pub fn withdraw_premium_tokens(ctx: Context<WithdrawPremiumTokens>, amount: u64) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let authority = SeahorseSigner {
            account: &ctx.accounts.authority,
            programs: &programs_map,
        };

        let premium_mint_reserve_acc = dot::program::PremiumMintReserve::load(
            &mut ctx.accounts.premium_mint_reserve_acc,
            &programs_map,
        );

        let premium_account = SeahorseAccount {
            account: &ctx.accounts.premium_account,
            programs: &programs_map,
        };

        let destination = SeahorseAccount {
            account: &ctx.accounts.destination,
            programs: &programs_map,
        };

        withdraw_premium_tokens_handler(
            authority.clone(),
            premium_mint_reserve_acc.clone(),
            premium_account.clone(),
            destination.clone(),
            amount,
        );

        dot::program::PremiumMintReserve::store(premium_mint_reserve_acc);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (go_live_timestamp : i64 , normal_mints : u32 , seed_string : String)]
    pub struct CreatePremiumMintReserve<'info> {
        #[account(mut)]
        pub payer: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: PremiumMintReserve > () + 8 , payer = payer , seeds = ["premium-reserve" . as_bytes () . as_ref () , premium_mint . key () . as_ref () , seed_string . as_bytes () . as_ref ()] , bump)]
        pub premium_mint_reserve_acc: Box<Account<'info, dot::program::PremiumMintReserve>>,
        #[account(mut)]
        pub premium_mint: Box<Account<'info, Mint>>,
        # [account (init , payer = payer , seeds = ["premium-tokens" . as_bytes () . as_ref ()] , bump , token :: mint = premium_mint , token :: authority = premium_mint_reserve_acc)]
        pub premium_account: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub clock: Sysvar<'info, Clock>,
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
        pub rent: Sysvar<'info, Rent>,
    }

    pub fn create_premium_mint_reserve(
        ctx: Context<CreatePremiumMintReserve>,
        go_live_timestamp: i64,
        normal_mints: u32,
        seed_string: String,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let payer = SeahorseSigner {
            account: &ctx.accounts.payer,
            programs: &programs_map,
        };

        let premium_mint_reserve_acc = Empty {
            account: dot::program::PremiumMintReserve::load(
                &mut ctx.accounts.premium_mint_reserve_acc,
                &programs_map,
            ),
            bump: ctx.bumps.get("premium_mint_reserve_acc").map(|bump| *bump),
        };

        let premium_mint = SeahorseAccount {
            account: &ctx.accounts.premium_mint,
            programs: &programs_map,
        };

        let premium_account = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.premium_account,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("premium_account").map(|bump| *bump),
        };

        let clock = &ctx.accounts.clock.clone();

        create_premium_mint_reserve_handler(
            payer.clone(),
            premium_mint_reserve_acc.clone(),
            premium_mint.clone(),
            premium_account.clone(),
            go_live_timestamp,
            clock.clone(),
            normal_mints,
            seed_string,
        );

        dot::program::PremiumMintReserve::store(premium_mint_reserve_acc.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (go_live_ts : i64 , initialization_ts : i64)]
    pub struct CreateNormalMintReserve<'info> {
        #[account(mut)]
        pub payer: Signer<'info>,
        # [account (init , payer = payer , seeds = ["normal-token-account" . as_bytes () . as_ref ()] , bump , token :: mint = normal_mint , token :: authority = normal_mint_reserve_acc)]
        pub normal_token_account: Box<Account<'info, TokenAccount>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: NormalMintReserve > () + 8 , payer = payer , seeds = ["normal-mint-reserve" . as_bytes () . as_ref ()] , bump)]
        pub normal_mint_reserve_acc: Box<Account<'info, dot::program::NormalMintReserve>>,
        #[account(mut)]
        pub premium_mint_reserve_acc: Box<Account<'info, dot::program::PremiumMintReserve>>,
        #[account(mut)]
        pub normal_mint: Box<Account<'info, Mint>>,
        #[account(mut)]
        pub clock: Sysvar<'info, Clock>,
        pub token_program: Program<'info, Token>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn create_normal_mint_reserve(
        ctx: Context<CreateNormalMintReserve>,
        go_live_ts: i64,
        initialization_ts: i64,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let payer = SeahorseSigner {
            account: &ctx.accounts.payer,
            programs: &programs_map,
        };

        let normal_token_account = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.normal_token_account,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("normal_token_account").map(|bump| *bump),
        };

        let normal_mint_reserve_acc = Empty {
            account: dot::program::NormalMintReserve::load(
                &mut ctx.accounts.normal_mint_reserve_acc,
                &programs_map,
            ),
            bump: ctx.bumps.get("normal_mint_reserve_acc").map(|bump| *bump),
        };

        let premium_mint_reserve_acc = dot::program::PremiumMintReserve::load(
            &mut ctx.accounts.premium_mint_reserve_acc,
            &programs_map,
        );

        let normal_mint = SeahorseAccount {
            account: &ctx.accounts.normal_mint,
            programs: &programs_map,
        };

        let clock = &ctx.accounts.clock.clone();

        create_normal_mint_reserve_handler(
            payer.clone(),
            normal_token_account.clone(),
            normal_mint_reserve_acc.clone(),
            premium_mint_reserve_acc.clone(),
            normal_mint.clone(),
            go_live_ts,
            initialization_ts,
            clock.clone(),
        );

        dot::program::NormalMintReserve::store(normal_mint_reserve_acc.account);

        dot::program::PremiumMintReserve::store(premium_mint_reserve_acc);

        return Ok(());
    }
}
