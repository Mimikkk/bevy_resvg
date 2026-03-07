# Changelog

## [2.0.0] - 2026-03-07

### Features

1. Explicitly state which files are supported2. Add SVG type for UI rendering

### Bugfixes

1. Properly handle all `AssetEvent`s

- Replace single monolithic event handler system with dedicated handlers
  for each `AssetEvent` variant.
- Add support for the `Removed` and `Unused` events.
- Properly handle the `Modified` event, which didn't actually update the
  `Sprite before`

| Footer | Value |
| -- | -- |
| NOTE | `Added` isn't actually handled, but it isn't supposed to either.
We react to `LoadedWithDependencies` instead. |
| Fix | #1 |

2. *(README)* Correct factual error about bevy_svg

### Other

1. `all-asset-events` into `main`

### Documentation

1. Check off more `AssetEvent`s on todo-list2. Update status for `bevy_svg`3. Update our support for hot-reloading4. Add comparison with Bevy Vello

| Footer | Value |
| -- | -- |
| Link | https://github.com/linebender/bevy_vello |

5. Add JIT to Todo list6. Include note about Inkscape SVGs7. Add UI example8. *(README)* Update `README.md` to mention UI9. *(spelling)* Fix spelling mistake in doccomment

### Performance

1. Use `HashSet` instead of `Vec` for events

`HashSet`s are supposedly faster than `Vec`s for `contain` calls,
although I haven't actually tested it.2. Check if asset id list is empty before loop

### Styling

1. *(sumi)* Sort allowed types array2. *(README)* Fix hard-wrapping in README

### Miscellaneous Tasks

1. *(sumi)* Add `merge` as commit type2. Add `debug` as bevy dev feature3. Enable default features for bevy dev builds4. Add helper event function

## [1.0.1] - 2026-01-24

### Bugfixes

1. Make `zoom.rs` example compile

### Other

1. *(release)* Release v1.0.1

### Documentation

1. Add badges2. Clarify how SLoC is counted3. Add documentation for each SvgError variant4. Update SLoC count

### Styling

1. Make clippy like my `README.md` file2. Remove superfluous `default` call in zoom

### Miscellaneous Tasks

1. Restrict visibility of internal method2. Add git-cliff3. Exclude `cliff.toml` file4. Configure git-cliff5. Exclude `release.toml` file6. Don't put chore(release) in `CHANGELOG.md`

## [1.0.0] - 2026-01-24

### Features

1. Create a boilerplate SVG-loading plugin2. Add initial working version3. Relax dependencies4. Warn when unimplemented events are emitted5. Make internal types private6. Add example for what happens when you zoom7. Add prelude

### Refactor

1. [*BREAKING*] Rename types to be less technical

### Documentation

1. Document code2. Rewrite documentation to remove dead links3. Add `CONTRIBUTING.md`4. Add content to `README.md`5. Add migration guide from bevy_svg6. Remove unsupported angle brackets for links

### Styling

1. *(Cargo.toml)* Sort `package` field2. Sort `description` field3. Sort `exclude` field

### Miscellaneous Tasks

1. Initial commit2. Release version 1.0.03. Add categories, keywords and a description4. Exclude `sumi.toml` file from <crates.io>
