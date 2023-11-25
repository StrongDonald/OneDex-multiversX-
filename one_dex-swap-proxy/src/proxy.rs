multiversx_sc::imports!();

#[multiversx_sc::proxy]
pub trait OneDexSwapProxy {
    #[only_owner]
    #[endpoint(setConfig)]
    fn set_config(
        &self,
        total_fee_percent: u64,
        special_fee_percent: u64,
        staking_reward_fee_percent: u64,
        treasury_address: ManagedAddress,
        staking_reward_address: ManagedAddress,
        burner_address: ManagedAddress,
        unwrap_address: ManagedAddress,
        registering_cost: BigUint,
    );

    #[only_owner]
    #[endpoint(addMainPair)]
    fn add_main_pair(&self, token: TokenIdentifier);

    #[only_owner]
    #[endpoint(removeMainPair)]
    fn remove_main_pair(&self, token: TokenIdentifier);

    #[endpoint(createPair)]
    fn create_pair(&self, first_token_id: TokenIdentifier, second_token_id: TokenIdentifier);

    #[payable("EGLD")]
    #[endpoint(enableSwap)]
    fn enable_swap(&self, pair_id: u32);

    #[payable("EGLD")]
    #[endpoint(issueLpToken)]
    fn issue_lp_token(&self, pair_id: u32);

    #[endpoint(setLpTokenLocalRoles)]
    fn set_lp_token_local_roles(&self, pair_id: u32);

    #[payable("*")]
    #[endpoint(addInitialLiquidity)]
    fn add_initial_liquidity(&self);

    #[payable("*")]
    #[endpoint(addLiquidity)]
    fn add_liquidity(&self, first_token_amount_min: BigUint, second_token_amount_min: BigUint);

    #[payable("*")]
    #[endpoint(removeLiquidity)]
    fn remove_liquidity(
        &self,
        first_token_amount_min: BigUint,
        second_token_amount_min: BigUint,
        unwrap_required: bool,
    );

    #[payable("*")]
    #[endpoint(swapMultiTokensFixedInput)]
    fn swap_multi_tokens_fixed_input(
        &self,
        amount_out_min: BigUint,
        unwrap_required: bool,
        path_args: MultiValueEncoded<TokenIdentifier>,
    );

    #[view(getLastPairId)]
    fn get_last_pair_id(&self) -> u32;

    #[view(getPairLpTokenId)]
    fn get_pair_lp_token_id(&self, pair_id: u32) -> TokenIdentifier;

    #[view(getPairLpTokenTotalSupply)]
    fn get_pair_lp_token_total_supply(&self, pair_id: u32) -> BigUint;

    #[view(getPairFirstTokenReserve)]
    fn get_pair_first_token_reserve(&self, pair_id: u32) -> BigUint;

    #[view(getPairSecondTokenReserve)]
    fn get_pair_second_token_reserve(&self, pair_id: u32) -> BigUint;

    #[view(getEquivalent)]
    fn get_equivalent(
        &self,
        token_in: TokenIdentifier,
        token_out: TokenIdentifier,
        amount_in: BigUint,
    ) -> BigUint;

    #[view(getAmountOut)]
    fn get_amount_out(
        &self,
        token_in: TokenIdentifier,
        token_out: TokenIdentifier,
        amount_in: BigUint,
    ) -> BigUint;

    #[view(getMainPairTokens)]
    fn get_main_pair_tokens(&self) -> MultiValueEncoded<TokenIdentifier>;
}
