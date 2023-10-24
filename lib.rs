#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(Ownable, PSP34, PSP34Metadata)]
#[openbrush::contract]
pub mod dropspace_sale {
    use ink::primitives::AccountId as Address;
	use ink_prelude::format;
    use openbrush::{traits::Storage, contracts::psp34::PSP34Error};
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
		current_supply: u128,
		base_uri: PreludeString,
		total_supply: u128,
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
			total_supply: u128,
			mint_per_tx: u128,
			mint_price: u128,
			mint_fee: u128,
			dev_wallet: Address,
			sale_time: u128
		) -> Self {
            let mut _instance = Self { 
				current_supply: 0, 
				base_uri: base_uri,
				total_supply: total_supply,
				mint_per_tx: mint_per_tx,
				mint_price: mint_price,
				mint_fee: mint_fee,
				dev_wallet: Some(dev_wallet),
				sale_time: u64::MAX,
				..Default::default() 
			};
			ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
			let collection_id = PSP34::collection_id(&_instance);
			metadata::Internal::_set_attribute(&mut _instance, collection_id.clone(), String::from("name"), String::from(name));
			metadata::Internal::_set_attribute(&mut _instance, collection_id, String::from("symbol"), String::from(symbol));
			_instance
        }

        fn mint_token(&mut self) -> Result<(), PSP34Error> {
			self.current_supply = self.current_supply.saturating_add(1);
			psp34::Internal::_mint_to(self, Self::env().caller(), Id::U128(self.current_supply))?;
            Ok(())
        }

		#[ink(message)]
		pub fn reserve(&mut self, amount: u128) -> Result<(), PSP34Error> {
			if self.current_supply.saturating_add(amount) > self.total_supply {
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

			if self.env().block_timestamp() < self.sale_time {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::buy: Sale hasn't started yet")));
			}

			if self.current_supply.saturating_add(amount)> self.total_supply {
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
		pub fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
			self.base_uri = uri;
			Ok(())
		}

		#[ink(message)]
		pub fn set_mint_per_tx(&mut self, mint_per_tx: u128) -> Result<(), PSP34Error> {
			self.mint_per_tx = mint_per_tx;
			Ok(())
		}

		#[ink(message)]
		pub fn set_mint_price(&mut self, mint_price: u128) -> Result<(), PSP34Error> {
			self.mint_price = mint_price;
			Ok(())
		}

		#[ink(message)]
		pub fn set_sale_time(&mut self, sale_time: u64) -> Result<(), PSP34Error> {
			self.sale_time = sale_time;
			Ok(())
		}

		#[ink(message)]
		pub fn set_total_supply(&mut self, total_supply: u128) -> Result<(), PSP34Error> {
			if total_supply < self.total_supply {
				return Err(PSP34Error::Custom(String::from("DropspaceSale::set_total_supply: Total supply is lesser than current")));
			}
			self.mint_fee = total_supply;
			Ok(())
		}

		#[ink(message)]
		pub fn token_uri(&self, token_id: u128) -> Result<PreludeString, PSP34Error> {
			let base_uri = self.base_uri.clone();
			Ok((format!("{base_uri}{token_id}")))
		}
    }
}