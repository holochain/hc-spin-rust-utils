# hc-spin-rust-utils

`@holochain/hc-spin-rust-utils` is a node add-on used by
- [@holochain/hc-spin](https://github.com/holochain/hc-spin)
- [kangaroo-electron](https://github.com/holochain/kangaroo-electron)

At the time of writing it contains two main pieces:
- A utility function `unpackAndSaveWebhapp` that takes a path to a .webhapp file, unpacks the .webhapp file into UI assets and .happ file and stores them at respective paths specified in the function arguments.
- A `ZomeCallSigner` object that connects to a running instance of lair keystore and exposes a method `signZomeCall` to have lair sign zome calls for a specified public key.


## Supporting a new Holochain version

Once a new version of Holochain is released, support for it can be added by following these steps:

1. Update the crates in `Cargo.toml` as necessary and make any required changes in case of compilation errors. Do NOT change the package version in package.json. This will be done automatically when creating a new release.
2. Test the package locally by
    1. Building it: `npm run build`
    2. Linking it globally with yarn: `yarn link`
    3. Inside a local `hc-spin` repository, use the linked local package: `yarn link @holochain/hc-spin-rust-utils` and test that hc-spin works and zome calls succeed (you may want to unlink it again after testing with `yarn unlink @holochain/hc-spin-rust-utils`).
3. If testing succeeds, commit the changes and make a PR to `main` or `main-xx` where `xx` is the Holochain version branch, for example `main-06` contains new Holochain `v0.6.x` versions. This will trigger a CI test run that builds the package for all platforms but does not publish it.

## Release process

To release a new version of `@holochain/hc-spin-rust-utils`, you may proceed as follows:

1. Trigger the [Prepare Release](https://github.com/holochain/hc-spin-rust-utils/actions/workflows/release-prepare.yml) action with the desired new version number on the corresponding branch (`main` for the Holochain `main` branch and `main-xx` for older versions).
2. This will bump the versions, generate a changelog, and open a PR for these changes.
3. Review and manually edit the PR on the working branch where required.
4. Merge the PR with a rebase-merge. This will trigger the [Publish Release](https://github.com/holochain/hc-spin-rust-utils/actions/workflows/release-publish.yml) workflow which will build the packages and publish them to NPM via Trusted Publishers and to GitHub as a release with artifacts.
