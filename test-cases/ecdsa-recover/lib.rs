#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ecdsa_recover {

    const _SIGNATURE: [u8; 65] = [
        195, 218, 227, 165, 226, 17, 25, 160, 37, 92, 142, 238, 4, 41, 244, 211, 18, 94, 131, 116,
        231, 116, 255, 164, 252, 248, 85, 233, 173, 225, 26, 185, 119, 235, 137, 35, 204, 251, 134,
        131, 186, 215, 76, 112, 17, 192, 114, 243, 102, 166, 176, 140, 180, 124, 213, 102, 117,
        212, 89, 89, 92, 209, 116, 17, 28,
    ];
    const _MESSAGE_HASH: [u8; 32] = [
        167, 124, 116, 195, 220, 156, 244, 20, 243, 69, 1, 98, 189, 205, 79, 108, 213, 78, 65, 65,
        230, 30, 17, 37, 184, 220, 237, 135, 1, 209, 101, 229,
    ];
    const _EXPECTED_COMPRESSED_PUBLIC_KEY: [u8; 33] = [
        3, 110, 192, 35, 209, 24, 189, 55, 218, 250, 100, 89, 40, 76, 222, 208, 202, 127, 31, 13,
        58, 51, 242, 179, 13, 63, 19, 22, 252, 164, 226, 248, 98,
    ];

    #[ink(storage)]
    pub struct ECDSARecover {}

    impl ECDSARecover {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn ecdsa_recover(&self, signature: [u8; 65], message_hash: [u8; 32]) -> [u8; 33] {
            self.env()
                .ecdsa_recover(&signature, &message_hash)
                .unwrap_or_else(|err| {
                    panic!("ecdsa_recover failed: {:?}", err);
                })
        }
    }

    impl Default for ECDSARecover {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn ecdsa_recover_works() {
            // Given
            let contract = ECDSARecover::new();

            // When
            let ecdsa_key = contract.ecdsa_recover(_SIGNATURE, _MESSAGE_HASH);

            // Then
            assert_eq!(ecdsa_key, _EXPECTED_COMPRESSED_PUBLIC_KEY);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn ecdsa_recover_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ECDSARecoverRef::new();
            let contract_acc_id = client
                .instantiate("ecdsa_recover", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let ecdsa_recover_call = build_message::<ECDSARecoverRef>(contract_acc_id)
                .call(|contract| contract.ecdsa_recover(_SIGNATURE, _MESSAGE_HASH));
            let ecdsa_recover_res = client
                .call(&ink_e2e::bob(), ecdsa_recover_call, 0, None)
                .await
                .expect("ecdsa_recover failed");

            // Then
            assert_eq!(
                ecdsa_recover_res.return_value(),
                _EXPECTED_COMPRESSED_PUBLIC_KEY
            );

            Ok(())
        }
    }
}
