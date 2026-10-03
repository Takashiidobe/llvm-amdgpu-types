# Publishing

To publish a specific tag, e.g. v0.1.0:

```sh
git tag v0.1.0
git push origin v0.1.0
```

## Later releases

Update the version in `Cargo.toml`, then refresh the lockfile and check the crate:

```sh
cargo generate-lockfile
cargo test --locked
cargo publish --dry-run --locked
```

Commit and push the changes, then tag the same version:

```sh
git tag v0.1.1
git push origin v0.1.1
```

The release workflow checks that the tag matches `Cargo.toml`, tests and packages
the crate, then publishes using trusted publishing. Release runs are serialized,
and already published versions are skipped on reruns.
