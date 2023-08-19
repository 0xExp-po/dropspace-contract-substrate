#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(Ownable, PSP34, PSP34Metadata)]
#[openbrush::contract]
pub mod my_psp34 {
    use ink::primitives::AccountId as Address;
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
		next_id: u32,
		base_uri: PreludeString,
		total_supply: u32,
		mint_per_tx: u32,
		mint_price: u32,
		mint_fee: u32,
		dev_wallet: Option<Address>,
		sale_time: u32
    }
    
    impl Contract {
        #[ink(constructor)]
        pub fn new(
			name: PreludeString, 
			symbol: PreludeString,
			base_uri: PreludeString,
			total_supply: u32,
			mint_per_tx: u32,
			mint_price: u32,
			mint_fee: u32,
			dev_wallet: Address,
		) -> Self {
            let mut _instance = Self { 
				next_id: 1, 
				base_uri: base_uri,
				total_supply: total_supply,
				mint_per_tx: mint_per_tx,
				mint_price: mint_price,
				mint_fee: mint_fee,
				dev_wallet: Some(dev_wallet),
				sale_time: u32::MAX,
				..Default::default() 
			};
			ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
			let collection_id = PSP34::collection_id(&_instance);
			metadata::Internal::_set_attribute(&mut _instance, collection_id.clone(), String::from("name"), String::from(name));
			metadata::Internal::_set_attribute(&mut _instance, collection_id, String::from("symbol"), String::from(symbol));
			_instance
        }

        fn mint_token(&mut self) -> Result<(), PSP34Error> {
			if (self.next_id < 100) {
				psp34::Internal::_mint_to(self, Self::env().caller(), Id::U32(self.next_id))?;
				self.next_id = self.next_id.saturating_add(1);
			}
            Ok(())
        }

		#[ink(message, payable)]
		pub fn buy(&mut self, amount: u64) -> Result<(), PSP34Error> {
			if self.env().transferred_value() != 1_000_000_000_000_000_000 {
				return Err(PSP34Error::Custom(String::from("BadMintValue")));
			}
		
			self.mint_token()
		}

		pub fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
			self.base_uri = uri;
			Ok(())
		}

		#[ink(message)]
		pub fn token_uri(&self, token_id: u64) -> Result<PreludeString, PSP34Error> {
			Ok((PreludeString::from("Hello")))
		}
    }
}