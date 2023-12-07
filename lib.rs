#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, PSP34, PSP34Metadata)]
#[openbrush::contract]
pub mod dropspace_sale {
    use ink::primitives::AccountId as Address;
<<<<<<< HEAD
    use ink_prelude::format;
    use ink_prelude::string::String as PreludeString;
    use openbrush::{
        contracts::psp34::{psp34, PSP34Error},
        modifiers,
        traits::Storage,
    };

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
        withdraw_wallet: Option<Address>,
        dev_wallet: Option<Address>,
        sale_time: u64,
=======
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
		withdraw_wallet: Option<Address>,
		dev_wallet: Option<Address>,
		sale_time: u64
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(
<<<<<<< HEAD
            name: PreludeString,
            symbol: PreludeString,
            base_uri: PreludeString,
            mint_per_tx: u128,
            mint_price: u128,
            mint_fee: u128,
            supply_limit: u128,
            withdraw_wallet: Option<Address>,
            dev_wallet: Option<Address>,
            sale_time: u64,
        ) -> Self {
            let mut _instance = Self {
                base_uri,
                supply_limit,
                mint_per_tx,
                mint_price,
                mint_fee,
                withdraw_wallet,
                dev_wallet,
                sale_time,
                ..Default::default()
            };

            ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
            let collection_id = PSP34::collection_id(&_instance);
            metadata::Internal::_set_attribute(
                &mut _instance,
                collection_id.clone(),
                String::from("name"),
                String::from(name),
            );
            metadata::Internal::_set_attribute(
                &mut _instance,
                collection_id,
                String::from("symbol"),
                String::from(symbol),
            );
=======
			name: PreludeString, 
			symbol: PreludeString,
			base_uri: PreludeString,
			mint_per_tx: u128,
			mint_price: u128,
			mint_fee: u128,
			supply_limit: u128,
			withdraw_wallet: Address,
			dev_wallet: Address,
			sale_time: u64
		) -> Self {
            let mut _instance = Self { 
				base_uri: base_uri,
				supply_limit: supply_limit,
				mint_per_tx: mint_per_tx,
				mint_price: mint_price,
				mint_fee: mint_fee,
				withdraw_wallet: Some(withdraw_wallet),
				dev_wallet: Some(dev_wallet),
				sale_time: sale_time,
				..Default::default() 
			};
			ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
			let collection_id = PSP34::collection_id(&_instance);
			metadata::Internal::_set_attribute(&mut _instance, collection_id.clone(), String::from("name"), String::from(name));
			metadata::Internal::_set_attribute(&mut _instance, collection_id, String::from("symbol"), String::from(symbol));
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b
			_instance
        }

        fn mint_token(&mut self) -> Result<(), PSP34Error> {
<<<<<<< HEAD
            let current_supply: u128 = psp34::PSP34::total_supply(self);
            psp34::Internal::_mint_to(self, Self::env().caller(), Id::U128(current_supply))?;
            Ok(())
        }

        #[ink(message)]
        pub fn reserve(&mut self, amount: u128) -> Result<(), PSP34Error> {
            let current_supply: u128 = psp34::PSP34::total_supply(self);
            if current_supply.saturating_add(amount) > self.supply_limit {
                return Err(PSP34Error::Custom(String::from(
                    "DropspaceSale::reserve: Supply limit reached",
                )));
            }
=======
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
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b

            for _i in 0..amount {
                let __ = self.mint_token();
            }

            Ok(())
        }

<<<<<<< HEAD
        #[ink(message, payable)]
        pub fn buy(&mut self, amount: u128) -> Result<(), PSP34Error> {
            let total_price = amount.saturating_mul(self.mint_price.saturating_add(self.mint_fee));
            let current_supply: u128 = psp34::PSP34::total_supply(self);
=======
		#[ink(message, payable)]
		pub fn buy(&mut self, amount: u128) -> Result<(), PSP34Error> {
			let total_price = amount.saturating_mul(self.mint_price.saturating_add(self.mint_fee));
			let current_supply: u128 = psp34::PSP34::total_supply(self);
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b

            if self.env().block_timestamp() < self.sale_time {
                return Err(PSP34Error::Custom(String::from(
                    "DropspaceSale::buy: Sale hasn't started yet",
                )));
            }

<<<<<<< HEAD
            if current_supply.saturating_add(amount) > self.supply_limit {
                return Err(PSP34Error::Custom(String::from(
                    "DropspaceSale::buy: Supply limit reached",
                )));
            }
=======
			if current_supply.saturating_add(amount)> self.supply_limit {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Supply limit reached")));
			}
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b

            if amount > self.mint_per_tx {
                return Err(PSP34Error::Custom(String::from(
                    "DropspaceSale::buy: Can't exceed amount of mints per tx",
                )));
            }

<<<<<<< HEAD
            if self.env().transferred_value() < total_price {
                return Err(PSP34Error::Custom(String::from(
                    "DropspaceSale::buy: Wrong amount paid.",
                )));
            }

            for _i in 0..amount {
                let __ = self.mint_token();
            }

            if let Some(dev_wallet) = self.dev_wallet {
                self.env()
                    .transfer(dev_wallet, amount.saturating_mul(self.mint_fee))
                    .map_err(|_| {
                        PSP34Error::Custom(String::from("Transfer to dev wallet failed"))
                    })?;
            } else {
                return Err(PSP34Error::Custom(String::from("Developer wallet not set")));
            }

            if let Some(withdraw_wallet) = self.withdraw_wallet {
                self.env()
                    .transfer(withdraw_wallet, amount.saturating_mul(self.mint_price))
                    .map_err(|_| {
                        PSP34Error::Custom(String::from("Transfer to owner wallet failed"))
                    })?;
            } else {
                return Err(PSP34Error::Custom(String::from("Owner wallet not set")));
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
                return Err(PSP34Error::Custom(String::from(
                    "DropspaceSale::set_total_supply: Supply limit is lesser than current supply",
                )));
            }
            self.supply_limit = supply_limit;
            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_withdraw_wallet(
            &mut self,
            withdraw_wallet: Option<Address>,
        ) -> Result<(), PSP34Error> {
            self.withdraw_wallet = withdraw_wallet;
            Ok(())
        }

        #[ink(message)]
        pub fn token_uri(&self, token_id: u128) -> Result<PreludeString, PSP34Error> {
            let base_uri = self.base_uri.clone();
            Ok(format!("{base_uri}{token_id}"))
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
        pub fn get_account_balance(&self) -> u128 {
            self.env().balance()
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
        pub fn withdraw_wallet(&self) -> Option<Address> {
            self.withdraw_wallet
        }

        #[ink(message)]
        pub fn sale_time(&self) -> u64 {
            self.sale_time
        }

        #[ink(message)]
        pub fn sale_active(&self) -> bool {
            self.sale_time <= self.env().block_timestamp()
        }

		#[ink(message)]
        pub fn base_uri(&self) -> PreludeString {
            self.base_uri.clone()
        }

		#[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw(&mut self) -> Result<(), PSP34Error> {
            let contract_balance = self.get_account_balance();

            if contract_balance > 0 {
                match self.env().transfer(Self::env().caller(), contract_balance) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(PSP34Error::Custom(String::from("Withdrawal failed"))),
                }
            } else {
                Err(PSP34Error::Custom(String::from("No funds to withdraw")))
            }
        }
=======
			if self.env().transferred_value() <= total_price {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Wrong amount paid.")));
			}
			
			for _i in 0 .. amount {
				self.mint_token();
			}

			if let Some(dev_wallet) = self.dev_wallet {
				self.env().transfer(dev_wallet, amount.saturating_mul(self.mint_fee))
					.map_err(|_| PSP34Error::Custom(String::from("Transfer to dev wallet failed")))?;
			} else {
				return Err(PSP34Error::Custom(String::from("Developer wallet not set")));
			}
		
			if let Some(withdraw_wallet) = self.withdraw_wallet {
				self.env().transfer(withdraw_wallet, amount.saturating_mul(self.mint_price))
					.map_err(|_| PSP34Error::Custom(String::from("Transfer to owner wallet failed")))?;
			} else {
				return Err(PSP34Error::Custom(String::from("Owner wallet not set")));
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
		#[modifiers(only_owner)]
		pub fn set_withdraw_wallet(&mut self, withdraw_wallet: Option<Address>) -> Result<(), PSP34Error> {
			self.withdraw_wallet = withdraw_wallet;
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
        pub fn get_account_balance(&self) -> u128 {
            self.env().balance()
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
        pub fn withdraw_wallet(&self) -> Option<Address> {
            self.withdraw_wallet
        }

		#[ink(message)]
        pub fn sale_time(&self) -> u64 {
            self.sale_time
        }

		#[ink(message)]
		pub fn sale_active(&self) -> bool {
			self.sale_time <= self.env().block_timestamp()
		}

		#[ink(message)]
		#[modifiers(only_owner)]
		pub fn withdraw(&mut self) -> Result<(), PSP34Error> {
			let contract_balance = self.get_account_balance();

			if contract_balance > 0 {
				match self.env().transfer(Self::env().caller(), contract_balance) {
					Ok(_) => Ok(()),
					Err(_) => Err(PSP34Error::Custom(String::from("Withdrawal failed"))),
				}
			} else {
				Err(PSP34Error::Custom(String::from("No funds to withdraw")))
			}
		}
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b
    }
}

#[cfg(test)]
mod tests {
<<<<<<< HEAD
	#[rustfmt::skip]
    use super::*;
    use dropspace_sale::Contract;
    use ink::{env::DefaultEnvironment as Environment, primitives::AccountId};
    use openbrush::contracts::psp34::{psp34, PSP34Error, Data};
	use openbrush::contracts::psp34::extensions::metadata::psp34metadata_external::PSP34Metadata;
	
    fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
        ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
    }

    struct ContractParam {
		name: String,
        symbol: String,
		base_uri: String,
        supply_limit: u128,
        mint_per_tx: u128,
        mint_price: u128,
        mint_fee: u128,
        withdraw_wallet: Option<AccountId>,
		dev_wallet: Option<AccountId>,
		sale_time: u64,
    }

	impl Default for ContractParam {
		fn default() -> ContractParam {
			ContractParam {
				name:"Test".to_string(),
				symbol: "TST".to_string(),
				base_uri:  "https://example.com/token/".to_string(),
				supply_limit: 100000,
				mint_per_tx: 10,
				mint_price: 1000,
				mint_fee: 10,
				withdraw_wallet: None,
				dev_wallet: None,
				sale_time: 0,
			}
		}
	}

	fn get_contract(args: &ContractParam) -> Contract {
		return Contract::new(
            args.name.to_string(),
            args.symbol.to_string(),
            args.base_uri.to_string(),
            args.mint_per_tx,
            args.mint_price,
            args.mint_fee,
            args.supply_limit,
            args.withdraw_wallet,
            args.dev_wallet,
            args.sale_time,
        );
    }

    #[ink::test]
    fn new_works() {
        let accounts = default_accounts();

		let params = ContractParam {
			withdraw_wallet: Some(accounts.django),
			dev_wallet: Some(accounts.alice),
			..Default::default()
		};
		let contract = get_contract(&params);

		// ink::env::debug_println!("--> Test started <--");

		assert_eq!(contract.supply_limit(), params.supply_limit);
        assert_eq!(contract.mint_per_tx(), params.mint_per_tx);
        assert_eq!(contract.mint_price(), params.mint_price);
        assert_eq!(contract.mint_fee(), params.mint_fee);
        assert_eq!(contract.dev_wallet(), params.dev_wallet);
        assert_eq!(contract.withdraw_wallet(), params.withdraw_wallet);
        assert_eq!(contract.sale_time(), params.sale_time);
        assert_eq!(contract.sale_active(), true);
		
		let id = psp34::PSP34::collection_id(&contract);
        assert_eq!(PSP34Metadata::get_attribute(&contract, id.clone(), String::from("name")), Some(params.name));
		assert_eq!(PSP34Metadata::get_attribute(&contract, id.clone(), String::from("symbol")), Some(params.symbol));
		assert_eq!(contract.base_uri(), params.base_uri);

    }
	
    #[ink::test]
=======
    use super::*;
	use dropspace_sale::Contract as Contract;
	use ink::env::DefaultEnvironment as Environment;
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
			accounts.django,
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
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b
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
<<<<<<< HEAD
            Some(accounts.django),
            Some(accounts.alice),
=======
			accounts.django,
            accounts.alice,
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b
            12345678,
        );

        assert_eq!(contract.reserve(5), Ok(()));
        assert_eq!(psp34::PSP34::total_supply(&contract), 5);
<<<<<<< HEAD
        assert_eq!(
            contract.reserve(100001),
            Err(PSP34Error::Custom(String::from(
                "DropspaceSale::reserve: Supply limit reached"
            )))
        );
    }

    #[ink::test]
    fn buy_works() {
        let accounts: ink::env::test::DefaultAccounts<Environment> = default_accounts();
        let mut contract = Contract::new(
            "Test".to_string(),
            "TST".to_string(),
            "https://example.com/token/".to_string(),
            10,     // limit
            1000,   // price
            10,     // fee
            100000, // supply limit
            Some(accounts.django),
            Some(accounts.charlie),
            0, // set sale time to 0 for testing
        );

        ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
            accounts.bob,
            100_000_000,
        );
        ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.charlie, 0);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob); // Setting the caller for the next contract call

        assert_eq!(ink::env::pay_with_call!(contract.buy(5), 5_050), Ok(()));
        ink::env::debug_println!(
            " Dev Wallet (chalie) bal={:?}",
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(accounts.charlie)
        );

        assert_eq!(psp34::PSP34::total_supply(&contract), 5);
        assert_eq!(contract.get_account_balance(), 100_0_000);
        assert_eq!(
            contract.buy(100001),
            Err(PSP34Error::Custom(String::from(
                "DropspaceSale::buy: Supply limit reached"
            )))
        );
        assert_eq!(
            contract.buy(15),
            Err(PSP34Error::Custom(String::from(
                "DropspaceSale::buy: Can't exceed amount of mints per tx"
            )))
        );
        assert_eq!(
            ink::env::pay_with_call!(contract.buy(4), 4_039),
            Err(PSP34Error::Custom(String::from(
                "DropspaceSale::buy: Wrong amount paid."
            )))
        );
    }

=======
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
			accounts.django,
			accounts.charlie,
			0, // set sale time to 0 for testing
		);
	
		ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.bob, 100_000_000);
		ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob); // Setting the caller for the next contract call		
		
		assert_eq!(ink::env::pay_with_call!(contract.buy(5), 5_050), Ok(()));
		assert_eq!(psp34::PSP34::total_supply(&contract), 5);
		assert_eq!(contract.get_account_balance(), 100_0_000);
		assert_eq!(contract.buy(100001), Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Supply limit reached"))));
		assert_eq!(contract.buy(15), Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Can't exceed amount of mints per tx"))));
		assert_eq!(contract.buy(4), Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Wrong amount paid."))));
	}
	
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b
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
<<<<<<< HEAD
            Some(accounts.django),
            Some(accounts.alice),
            12345678,
        );

        assert_eq!(
            contract.set_base_uri("https://newuri.com/token/".to_string()),
            Ok(())
        );
=======
			accounts.django,
            accounts.alice,
            12345678,
        );

        assert_eq!(contract.set_base_uri("https://newuri.com/token/".to_string()), Ok(()));
>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b
        assert_eq!(contract.set_mint_per_tx(20), Ok(()));
        assert_eq!(contract.set_mint_price(2000), Ok(()));
        assert_eq!(contract.set_sale_time(87654321), Ok(()));
        assert_eq!(contract.set_supply_limit(50000), Ok(()));
        assert_eq!(contract.reserve(5), Ok(()));
<<<<<<< HEAD
        assert_eq!(
            contract.set_supply_limit(1),
            Err(PSP34Error::Custom(String::from(
                "DropspaceSale::set_total_supply: Supply limit is lesser than current supply"
            )))
        );
    }

    #[ink::test]
    fn sale_active_works() {
        let accounts = default_accounts();
        let contract = Contract::new(
            "Test".to_string(),
            "TST".to_string(),
            "https://example.com/token/".to_string(),
            10,
            1000,
            10,
            100000,
            Some(accounts.django),
            Some(accounts.alice),
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
            Some(accounts.django),
            Some(accounts.alice),
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

    #[ink::test]
    fn withdraw_works() {
        let accounts = default_accounts();
        let mut contract = Contract::new(
            "Test".to_string(),
            "TST".to_string(),
            "https://example.com/token/".to_string(),
            10, // Assuming the price per token is 10
            1000,
            10,
            100000,
            Some(accounts.django),
            Some(accounts.charlie),
            0, // set sale time to 0 for testing
        );

        // Simulate buying a token
        ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 0);
        ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.charlie, 0);
        ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
            accounts.bob,
            100_000_000,
        );

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob); // Setting the caller for the next contract call
                                                                                  // Simulating value transferred for the next call - 10 tokens * 10 units each
        assert_eq!(ink::env::pay_with_call!(contract.buy(10), 10_100), Ok(()));
        assert_eq!(contract.get_account_balance(), 0);

        // Check that owner's balance has increased by 10000 units
        let dev_balance =
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(accounts.charlie)
                .unwrap_or_default();
        assert_eq!(dev_balance, 100);

        // Simulate the owner calling the withdraw function
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        assert_eq!(contract.withdraw(), Ok(()));

        let owner_balance =
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(accounts.alice)
                .unwrap_or_default();
        assert_eq!(owner_balance, 10000);
        assert_eq!(contract.get_account_balance(), 0);
    }
}
=======
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
			accounts.django,
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
			accounts.django,
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

	#[ink::test]
	fn withdraw_works() {
		let accounts = default_accounts();
		let mut contract = Contract::new(
			"Test".to_string(),
			"TST".to_string(),
			"https://example.com/token/".to_string(),
			10, // Assuming the price per token is 10
			1000,
			10,
			100000,
			accounts.django,
			accounts.charlie,
			0, // set sale time to 0 for testing
		);


		// Simulate buying a token
		ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 0);
		ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.charlie, 0);
		ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.bob, 100_000_000);
		
		ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob); // Setting the caller for the next contract call
		// Simulating value transferred for the next call - 10 tokens * 10 units each
		assert_eq!(ink::env::pay_with_call!(contract.buy(10), 10_100), Ok(()));
		assert_eq!(contract.get_account_balance(), 0);
		
		// Check that owner's balance has increased by 10000 units
		let dev_balance = ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(accounts.charlie).unwrap_or_default();
		assert_eq!(dev_balance, 100);

		// Simulate the owner calling the withdraw function
		ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
		assert_eq!(contract.withdraw(), Ok(()));

		let owner_balance = ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(accounts.alice).unwrap_or_default();
		assert_eq!(owner_balance, 10000);
		assert_eq!(contract.get_account_balance(), 0);
	}
}

>>>>>>> eca283c7a82996baad53f4e3f462e6d5133e778b
