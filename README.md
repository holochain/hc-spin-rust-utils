# hc-spin-rust-utils

`@holochain/hc-spin-rust-utils` is a node add-on used by
- [@holochain/hc-spin](https://github.com/holochain/hc-spin)
- [kangaroo-electron](https://github.com/holochain/kangaroo-electron)

At the time of writing it contains two main pieces:
- A utility function `unpackAndSaveWebhapp` that takes a path to a .webhapp file, unpacks the .webhapp file into UI assets and .happ file and stores them at respective paths specified in the function arguments.
- A `ZomeCallSigner` object that connects to a running instance of lair keystore and exposes a method `signZomeCall` to have lair sign zome calls for a specified public key.



## Release process

To release a new version of `@holochain/hc-spin-rust-utils`, you may proceed as follows:

1. Update the crates in Cargo.toml as necessary and make any required changes in case of compilation errors.
2. (optional but recommended) Test the package locally by
    1. Building it: `npm run build`
    2. Linking it globally with yarn: `yarn link`
    3. Inside a local `hc-spin` repository, use the linked local package: `yarn link @holochain/hc-spin-rust-utils` and test that hc-spin works and zome calls succeed (you may want to unlink it again after testing with `yarn unlink @holochain/hc-spin-rust-utils`).
3. If testing succeeds, commit the changes and make a PR to main. This will trigger a CI test run that builds the package for all platforms but does not publish it.
4. If the CI run from step 3 succeeds, merge the PR. Then create a new version commit for the desired version directly on the main branch, using the associated npm command: `npm version [desired version]`, e.g. `npm version 0.700.0` and push to main. This will trigger the CI workflow again which will this time recognize from the commit that it is a version release and automatically publish the built packages to npm. The latter requires that a valid `NPM_TOKEN` has been added as a repository secret.


## NPM token

The npm token used to publish the packages to npm from CI is currently stored as a repository secret (`NPM_TOKEN`) in this repository. If the token expires, a new token needs to be created on npmjs.org and the content of the repository secret replaced with the newly generated token.
