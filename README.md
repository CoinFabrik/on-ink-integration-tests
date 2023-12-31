# On ink! integration tests

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

<p align="center">
  <img src="/assets/blacksmith_flattening_the_anvil_V1.png" alt="With a properly flattened anvil, quality tools can be built." width="300" center  />
</p>

We have discovered that integration tests for ink! contracts lack some of the functionalities found in E2E testing.

Integration tests run significantly faster than E2E tests. If a full range of functionalities were provided, it could reduce testing and QA times. 

We noticed these missing functionalities in integration tests while developing fuzzing detectors for our vulnerability analyzer, Scout. During this work, we identified 2 functions with implementation differences, default_account() and set_contract_storage(), and we also observed 3 functions which were unimplemented for integration tests. This initial identification was observed in our grant application form for the grant #1875.

Ideally, each test in E2E should have its counterpart in Integration. This implies that the set of tests per function is unique and should be replicated in both environments. This idea will be reviewed in the next milestone, as achieving such a level of thoroughness is not planned for this instance.

To run a test, navigate to the corresponding directory and run `cargo test` for the off-chain test, or `cargo test --features e2e-tests` for the on-chain, E2E test. You'll need to already have Rust set up on your system.
