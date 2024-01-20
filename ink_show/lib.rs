#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ink_show {

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Roles {
        SuperAdmin,
        Moderator,
        User
    }

    #[ink(storage)]
    pub struct InkShow {
        value: Roles,
    }

    impl InkShow {
        #[ink(constructor)]
        pub fn new(init_value: Roles) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `Role` value to `User`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Roles::User)
        }

        /// Simply returns the current value of our user role.
        #[ink(message)]
        pub fn get(&self) -> Roles {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let ink_show = InkShow::default();
            assert_eq!(ink_show.get(), Roles::User);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut ink_show = InkShow::new(Roles::SuperAdmin);
            assert_eq!(ink_show.get(), Roles::SuperAdmin);
        }
    }

    // #[cfg(all(test, feature = "e2e-tests"))]
    // mod e2e_tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// A helper function used for calling contract messages.
    //     use ink_e2e::build_message;

    //     /// The End-to-End test `Result` type.
    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    //     /// We test that we can upload and instantiate the contract using its default constructor.
    //     #[ink_e2e::test]
    //     async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let constructor = InkShowRef::default();

    //         // When
    //         let contract_account_id = client
    //             .instantiate("ink_show", &ink_e2e::alice(), constructor, 0, None)
    //             .await
    //             .expect("instantiate failed")
    //             .account_id;

    //         // Then
    //         let get = build_message::<InkShowRef>(contract_account_id.clone())
    //             .call(|ink_show| ink_show.get());
    //         let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), false));

    //         Ok(())
    //     }

    //     /// We test that we can read and write a value from the on-chain contract contract.
    //     #[ink_e2e::test]
    //     async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let constructor = InkShowRef::new(false);
    //         let contract_account_id = client
    //             .instantiate("ink_show", &ink_e2e::bob(), constructor, 0, None)
    //             .await
    //             .expect("instantiate failed")
    //             .account_id;

    //         let get = build_message::<InkShowRef>(contract_account_id.clone())
    //             .call(|ink_show| ink_show.get());
    //         let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), false));

    //         // When
    //         let flip = build_message::<InkShowRef>(contract_account_id.clone())
    //             .call(|ink_show| ink_show.flip());
    //         let _flip_result = client
    //             .call(&ink_e2e::bob(), flip, 0, None)
    //             .await
    //             .expect("flip failed");

    //         // Then
    //         let get = build_message::<InkShowRef>(contract_account_id.clone())
    //             .call(|ink_show| ink_show.get());
    //         let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), true));

    //         Ok(())
    //     }
    // }
}
