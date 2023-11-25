#![no_std]

pub mod proxy;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait OneDexSwapModule {
    fn get_ondex_swap_addr(&self) -> ManagedAddress {
        require!(
            !self.ondex_swap_addr().is_empty(),
            "OneDex Swap address not set"
        );

        let addr = self.ondex_swap_addr().get();

        require!(
            self.blockchain().is_smart_contract(&addr),
            "Invalid OneDexSwap address"
        );

        addr
    }

    #[only_owner]
    #[endpoint]
    fn set_ondex_swap_addr(&self, addr: &ManagedAddress) {
        require!(
            self.blockchain().is_smart_contract(addr),
            "Invalid OneDexSwap address"
        );

        self.ondex_swap_addr().set_if_empty(addr);
    }

    fn call_onedex_swap(&self) -> proxy::Proxy<Self::Api> {
        self.ondex_swap_proxy_obj(self.get_ondex_swap_addr())
    }

    #[proxy]
    fn ondex_swap_proxy_obj(&self, addr: ManagedAddress) -> proxy::Proxy<Self::Api>;

    #[view(oneDexSwapAddress)]
    #[storage_mapper("OneDexSwapModule::ondex_swap_addr")]
    fn ondex_swap_addr(&self) -> SingleValueMapper<ManagedAddress>;
}
