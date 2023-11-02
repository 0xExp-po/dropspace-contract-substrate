#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(Ownable, PSP34, PSP34Metadata)]
#[openbrush::contract]
pub mod dropspace_sale {
    use ink::primitives::AccountId as Address;
	use ink_prelude::format;
    use openbrush::{
		traits::Storage, 
		contracts::psp34::{PSP34Error, psp34}, 
		modifiers
	};
	use ink_prelude::string::String as PreludeString;

    #[ink(storage)]
    #[derive(Default, Storage)]
	pub struct Contract {
    	#[storage_field]
		psp34: psp34::Data,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
		base_uri: PreludeString,
		supply_limit: u128,
		mint_per_tx: u128,
		mint_price: u128,
		mint_fee: u128,
		dev_wallet: Option<Address>,
		sale_time: u64
    }
    
    impl Contract {
        #[ink(constructor)]
        pub fn new(
			name: PreludeString, 
			symbol: PreludeString,
			base_uri: PreludeString,
			mint_per_tx: u128,
			mint_price: u128,
			mint_fee: u128,
			supply_limit: u128,
			dev_wallet: Address,
			sale_time: u64
		) -> Self {
            let mut _instance = Self { 
				base_uri: base_uri,
				supply_limit: supply_limit,
				mint_per_tx: mint_per_tx,
				mint_price: mint_price,
				mint_fee: mint_fee,
				dev_wallet: Some(dev_wallet),
				sale_time: sale_time,
				..Default::default() 
			};
			ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
			let collection_id = PSP34::collection_id(&_instance);
			metadata::Internal::_set_attribute(&mut _instance, collection_id.clone(), String::from("name"), String::from(name));
			metadata::Internal::_set_attribute(&mut _instance, collection_id, String::from("symbol"), String::from(symbol));
			_instance
        }

        fn mint_token(&mut self) -> Result<(), PSP34Error> {
			let current_supply: u128 = psp34::PSP34::total_supply(self);
			psp34::Internal::_mint_to(self, Self::env().caller(), Id::U128(current_supply))?;
            Ok(())
        }

		#[ink(message)]
		pub fn reserve(&mut self, amount: u128) -> Result<(), PSP34Error> {
			let current_supply: u128 = psp34::PSP34::total_supply(self);
			if current_supply.saturating_add(amount) > self.supply_limit {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::reserve: Supply limit reached")));
			}

			for _i in 0 .. amount {
				self.mint_token();
			}

			Ok(())
		}

		#[ink(message, payable)]
		pub fn buy(&mut self, amount: u128) -> Result<(), PSP34Error> {
			let total_price = amount.saturating_mul(self.mint_price.saturating_add(self.mint_fee));
			let current_supply: u128 = psp34::PSP34::total_supply(self);

			if self.env().block_timestamp() < self.sale_time {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Sale hasn't started yet")));
			}

			if current_supply.saturating_add(amount)> self.supply_limit {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Supply limit reached")));
			}

			if amount > self.mint_per_tx {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Can't exceed amount of mints per tx")));
			}

			if self.env().transferred_value() != total_price {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Wrong amount paid.")));
			}
			
			for _i in 0 .. amount {
				self.mint_token();
			}

			Ok(())
		}

		#[ink(message)]
		#[modifiers(only_owner)]
		pub fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
			self.base_uri = uri;
			Ok(())
		}

		#[ink(message)]
		#[modifiers(only_owner)]
		pub fn set_mint_per_tx(&mut self, mint_per_tx: u128) -> Result<(), PSP34Error> {
			self.mint_per_tx = mint_per_tx;
			Ok(())
		}

		#[ink(message)]
		#[modifiers(only_owner)]
		pub fn set_mint_price(&mut self, mint_price: u128) -> Result<(), PSP34Error> {
			self.mint_price = mint_price;
			Ok(())
		}

		#[ink(message)]
		#[modifiers(only_owner)]
		pub fn set_sale_time(&mut self, sale_time: u64) -> Result<(), PSP34Error> {
			self.sale_time = sale_time;
			Ok(())
		}

		#[ink(message)]
		#[modifiers(only_owner)]
		pub fn toggle_sale_active(&mut self) -> Result<(), PSP34Error> {
			if self.sale_time() != 0 {
				self.sale_time = 0;
			} else {
				self.sale_time = u64::MAX;
			}
			Ok(())
		}

		#[ink(message)]
		#[modifiers(only_owner)]
		pub fn set_supply_limit(&mut self, supply_limit: u128) -> Result<(), PSP34Error> {
			let current_supply: u128 = psp34::PSP34::total_supply(self);
			if current_supply > supply_limit {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::set_total_supply: Supply limit is lesser than current supply")));
			}
			self.supply_limit = supply_limit;
			Ok(())
		}

		#[ink(message)]
		pub fn token_uri(&self, token_id: u128) -> Result<PreludeString, PSP34Error> {
			let base_uri = self.base_uri.clone();
			Ok((format!("{base_uri}{token_id}")))
		}

		#[ink(message)]
        pub fn supply_limit(&self) -> u128 {
            self.supply_limit
        }

		#[ink(message)]
        pub fn mint_per_tx(&self) -> u128 {
            self.mint_per_tx
        }

		#[ink(message)]
        pub fn mint_price(&self) -> u128 {
            self.mint_price
        }

		#[ink(message)]
        pub fn mint_fee(&self) -> u128 {
            self.mint_fee
        }

		#[ink(message)]
        pub fn dev_wallet(&self) -> Option<Address> {
            self.dev_wallet
        }

		#[ink(message)]
        pub fn sale_time(&self) -> u64 {
            self.sale_time
        }

		#[ink(message)]
		pub fn sale_active(&self) -> bool {
			self.sale_time <= self.env().block_timestamp()
		}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
	use dropspace_sale::Contract as Contract;
    use openbrush::{
		traits::Storage, 
		contracts::psp34::{PSP34Error, psp34}, 
		modifiers
	};

	fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
		ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
	}

	#[ink::test]
    fn new_works() {
        let accounts = default_accounts();
		
        let contract = Contract::new(
            "Test".to_string(),
            "TST".to_string(),
            "https://example.com/token/".to_string(),
            10,
            1000,
            10,
            100000,
            accounts.alice,
            12345678,
        );
        assert_eq!(contract.supply_limit(), 100000);
        assert_eq!(contract.mint_per_tx(), 10);
        assert_eq!(contract.mint_price(), 1000);
        assert_eq!(contract.mint_fee(), 10);
        assert_eq!(contract.dev_wallet(), Some(accounts.alice));
        assert_eq!(contract.sale_time(), 12345678);
    }

	#[ink::test]
    fn reserve_works() {
        let accounts = default_accounts();
        let mut contract = Contract::new(
            "Test".to_string(),
            "TST".to_string(),
            "https://example.com/token/".to_string(),
            10,
            1000,
            10,
            100000,
            accounts.alice,
            12345678,
        );

        assert_eq!(contract.reserve(5), Ok(()));
        assert_eq!(psp34::PSP34::total_supply(&contract), 5);
        assert_eq!(contract.reserve(100001), Err(PSP34Error::Custom(String::from("DropspaceSale::reserve: Supply limit reached"))));
    }

	#[ink::test]
	fn buy_works() {
		let accounts = default_accounts();
		let mut contract = Contract::new(
			"Test".to_string(),
			"TST".to_string(),
			"https://example.com/token/".to_string(),
			10,
			1000,
			10,
			100000,
			accounts.alice,
			0, // set sale time to 0 for testing
		);
	
		ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.bob, 100_000_000);
		ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob); // Setting the caller for the next contract call
		ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(5_050); // Simulating value transferred for the next call
	
		assert_eq!(contract.buy(5), Ok(()));
		assert_eq!(psp34::PSP34::total_supply(&contract), 5);
		assert_eq!(contract.buy(100001), Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Supply limit reached"))));
		assert_eq!(contract.buy(15), Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Can't exceed amount of mints per tx"))));
		assert_eq!(contract.buy(4), Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Wrong amount paid."))));
	}
	
    #[ink::test]
    fn setters_work() {
        let accounts = default_accounts();
        let mut contract = Contract::new(
            "Test".to_string(),
            "TST".to_string(),
            "https://example.com/token/".to_string(),
            10,
            1000,
            10,
            100000,
            accounts.alice,
            12345678,
        );

        assert_eq!(contract.set_base_uri("https://newuri.com/token/".to_string()), Ok(()));
        assert_eq!(contract.set_mint_per_tx(20), Ok(()));
        assert_eq!(contract.set_mint_price(2000), Ok(()));
        assert_eq!(contract.set_sale_time(87654321), Ok(()));
        assert_eq!(contract.set_supply_limit(50000), Ok(()));
        assert_eq!(contract.reserve(5), Ok(()));
        assert_eq!(contract.set_supply_limit(1), Err(PSP34Error::Custom(String::from("DropspaceSale::set_total_supply: Supply limit is lesser than current supply"))));
    }

	#[ink::test]
	fn sale_active_works() {
		let accounts = default_accounts();
		let mut contract = Contract::new(
			"Test".to_string(),
			"TST".to_string(),
			"https://example.com/token/".to_string(),
			10,
			1000,
			10,
			100000,
			accounts.alice,
			12345678, // set future sale time for testing
		);

		// Before the sale time, sale should not be active
		assert_eq!(contract.sale_active(), false);

		// Set the block timestamp to simulate sale time passing
		ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(12345679);

		// After the sale time, sale should be active
		assert_eq!(contract.sale_active(), true);
	}

	#[ink::test]
	fn toggle_sale_active_works() {
		let accounts = default_accounts();
		let mut contract = Contract::new(
			"Test".to_string(),
			"TST".to_string(),
			"https://example.com/token/".to_string(),
			10,
			1000,
			10,
			100000,
			accounts.alice,
			1234567890, // set future sale time for testing
		);

		// Ensure that only the owner can toggle sale active
		ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
		assert_eq!(
			contract.toggle_sale_active(),
			Err(PSP34Error::Custom(String::from("O::CallerIsNotOwner")))
		);

		// Simulate the owner calling the function
		ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

		// Initially, the sale should not be active
		assert_eq!(contract.sale_active(), false);

		// Toggle sale active, which should set the sale time to 0
		assert_eq!(contract.toggle_sale_active(), Ok(()));
		assert_eq!(contract.sale_time(), 0);
		assert_eq!(contract.sale_active(), true);

		// Toggle sale active again, which should set the sale time to u64::MAX
		assert_eq!(contract.toggle_sale_active(), Ok(()));
		assert_eq!(contract.sale_time(), u64::MAX);
		assert_eq!(contract.sale_active(), false);
	}
}

