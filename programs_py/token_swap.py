# token_swap
# Built with Seahorse v0.2.2


from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

# a 1:1 token swap with a premium mint reserve and a normal mint reserve
# a premium mint reserve can be linked to many normal mints for a 1:1 exchange of tokens

class PremiumMintReserve(Account):
  # the premium mint of the reserve
  premium_mint: Pubkey
  # the token account which stores the premium mint tokens
  premium_account: Pubkey
  # the timestamp at which the reserve went live
  go_live_timestamp: i64
  # the timestamp at which the reserve was created
  initialization_timestamp: i64
  # the no of normal mints this premium mint is eligible to be exchanged with...
  normal_mints: u32
  # the creator of the reserve
  creator: Pubkey
  #seed string for seeds
  random_hash: str
  #bump generated when creating this PremiumMintReserve PDA
  bump: u8
  # bump generated when creating premium token account
  token_bump: u8

class NormalMintReserve(Account):
  #the premium mint reserve this normal mint reserve is related to...
  premium_mint_reserve_acc: Pubkey
  # normal mint pubkey
  normal_mint: Pubkey
  # the token account to store normal mint tokens
  normal_token_account: Pubkey
  # the timestamp at which this reserve went live
  go_live_ts: i64
  # the timestamp at which this reserve was created
  initialization_ts: i64
  #bump generated when creating this NormalMintReserve PDA
  bump: u8
  # bump generated when creating normal token account
  token_bump: u8

# creating the premium mint reserve
@instruction
def create_premium_mint_reserve(
  payer: Signer, 
  premium_mint_reserve_acc: Empty[PremiumMintReserve],
  premium_mint: TokenMint,
  premium_account: Empty[TokenAccount], 
  go_live_timestamp: i64, 
  clock: Clock,
  normal_mints: u32, 
  random_hash: str,
):
 bump = premium_mint_reserve_acc.bump()
 premium_mint_reserve_acc = premium_mint_reserve_acc.init(
  payer = payer, 
  seeds = ['premium-reserve', premium_mint, random_hash],
 )
 token_bump = premium_account.bump()
 premium_account = premium_account.init(
  payer = payer, 
  seeds = ['premium-tokens', 
  #premium_mint_reserve_acc.key()
  ], 
  mint = premium_mint, 
  authority = premium_mint_reserve_acc, 
 )
 premium_mint_reserve_acc.premium_mint = premium_mint.key()
 premium_mint_reserve_acc.premium_account = premium_account.key()
 premium_mint_reserve_acc.initialization_timestamp = clock.unix_timestamp()
 premium_mint_reserve_acc.random_hash  = random_hash
 premium_mint_reserve_acc.creator = payer.key()
 premium_mint_reserve_acc.bump = bump
 premium_mint_reserve_acc.token_bump = token_bump
 premium_mint_reserve_acc.normal_mints = normal_mints
 if go_live_timestamp < clock.unix_timestamp():
    premium_mint_reserve_acc.go_live_timestamp = clock.unix_timestamp()
 else:
    premium_mint_reserve_acc.go_live_timestamp = go_live_timestamp


# creating the normal mint reserve
@instruction
def create_normal_mint_reserve(
  payer: Signer, 
  normal_token_account: Empty[TokenAccount],
  normal_mint_reserve_acc: Empty[NormalMintReserve], 
  premium_mint_reserve_acc: PremiumMintReserve,
  normal_mint: TokenMint,
  go_live_ts: i64, 
  initialization_ts: i64, 
  clock: Clock,
):
 bump =  normal_mint_reserve_acc.bump()
 normal_mint_reserve_acc = normal_mint_reserve_acc.init(
  payer = payer,
  seeds = [
    'normal-mint-reserve', # normal_mint.key()], #premium_mint_reserve_acc.key()
   ]
 )

 token_bump = normal_token_account.bump()
 normal_token_account = normal_token_account.init(
  payer = payer,
  seeds = ['normal-token-account', 
  #normal_mint_reserve_acc.key()
  ],
  mint = normal_mint,
  authority  = normal_mint_reserve_acc, 
 )
 
 assert (premium_mint_reserve_acc.premium_mint != normal_mint.key()), "premium mint can't be the same as normal mint"
 assert (premium_mint_reserve_acc.creator == payer.key()), "creators do not match"
 
 normal_mint_reserve_acc.premium_mint_reserve_acc = premium_mint_reserve_acc.key()
 normal_mint_reserve_acc.normal_mint = normal_mint.key()
 normal_mint_reserve_acc.normal_token_account = normal_token_account.key()
 normal_mint_reserve_acc.initialization_ts = initialization_ts
 normal_mint_reserve_acc.bump = bump
 normal_mint_reserve_acc.token_bump = token_bump 
 if go_live_ts < clock.unix_timestamp():
  normal_mint_reserve_acc.go_live_ts = clock.unix_timestamp()
 else:
  normal_mint_reserve_acc.go_live_ts = go_live_ts


@instruction
def withdraw_premium_tokens(
  authority: Signer, 
  premium_mint_reserve_acc: PremiumMintReserve, 
  premium_account: TokenAccount,
  destination: TokenAccount,
  amount: u64,
):
 assert(premium_mint_reserve_acc.creator == authority.key()), "Invalid Authority"
 assert(premium_mint_reserve_acc.premium_account == premium_account.key()), "Invalid Premium token account"

 premium_account.transfer(
  authority = premium_mint_reserve_acc, 
  to = destination, 
  amount = amount,
 )

@instruction
def withdraw_normal_tokens(
  authority: Signer,
  premium_mint_reserve_acc: PremiumMintReserve, 
  normal_mint_reserve_acc: NormalMintReserve,
  normal_token_account: TokenAccount,
  destination: TokenAccount,
  amount: u64, 
 ):
  assert (premium_mint_reserve_acc.creator == authority.key()), "Invalid authority"
  assert (normal_mint_reserve_acc.premium_mint_reserve_acc == premium_mint_reserve_acc.key()), "The normal reserve and the premium reserve are not related"
  assert (normal_mint_reserve_acc.normal_token_account == normal_token_account.key()), "Invalid normal_token account"

  normal_token_account.transfer(
    authority = normal_mint_reserve_acc, 
    to = destination, 
    amount = amount,
  )

@instruction
def swap_premium_tokens_for_normal_tokens(
  source_authority: Signer,
  premium_mint_reserve_acc: PremiumMintReserve,
  normal_mint_reserve_acc: NormalMintReserve, 
  normal_token_account: TokenAccount,
  premium_account: TokenAccount, 
  source: TokenAccount, 
  destination: TokenAccount, 
  clock: Clock, 
  amount: u64,
):
 assert(premium_mint_reserve_acc.premium_account == premium_account.key()), "Invalid premium token account"
 assert(normal_mint_reserve_acc.premium_mint_reserve_acc == premium_mint_reserve_acc.key()), "The premium and the normal reserves are not related"
 assert(normal_mint_reserve_acc.normal_token_account == normal_token_account.key()), "Invalid normal token account"
 assert(premium_mint_reserve_acc.go_live_timestamp < clock.unix_timestamp()), "Premium reserve not live yet"
 assert(normal_mint_reserve_acc.go_live_ts < clock.unix_timestamp()), "Normal reserve not live yet"
 # since this is a 1:1 swap it's fine if we only check that the amount that the signer 
 # wants in return (here normal tokens) is less than or equal to the amount left in the
 # child storage 
 normal_amount = normal_token_account.amount()
 assert(normal_amount >= amount), "Token amount too low to swap"

 source.transfer(
  authority = source_authority,
  to = premium_account,
  amount = amount,
 )

 normal_token_account.transfer(
  authority = normal_mint_reserve_acc,
  to = destination,
  amount = amount,
 )

@instruction
def swap_normal_tokens_for_premium_tokens(
  source_authority: Signer,
  premium_mint_reserve_acc: PremiumMintReserve,
  normal_mint_reserve_acc: NormalMintReserve, 
  normal_token_account: TokenAccount,
  premium_account: TokenAccount, 
  source: TokenAccount, 
  destination: TokenAccount, 
  clock: Clock, 
  amount: u64,
):
  assert(premium_mint_reserve_acc.premium_account == premium_account.key()), "Invalid premium token account"
  assert(normal_mint_reserve_acc.premium_mint_reserve_acc == premium_mint_reserve_acc.key()), "The premium and normal reserves are not related"
  assert(normal_mint_reserve_acc.normal_token_account == normal_token_account.key()), "Invalid child_storage"
  assert(premium_mint_reserve_acc.go_live_timestamp < clock.unix_timestamp()), "Premium reserve not live yet"
  assert(normal_mint_reserve_acc.go_live_ts < clock.unix_timestamp()), "Normal reserve not live yet"
 # since this is a 1:1 swap it's fine if we only check that the amount that the signer 
 # wants in return (here premium tokens) is less than or equal to the amount left in the
 # parent storage 
  premium_amount = premium_account.amount()
  assert(premium_amount >= amount), "Token amount too low to swap"
 
  source.transfer(
    authority = source_authority,
    to = normal_token_account,
    amount = amount,
  )
  premium_account.transfer(
    authority = premium_mint_reserve_acc,
    to = destination,
    amount = amount,
  )

