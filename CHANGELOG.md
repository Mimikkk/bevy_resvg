# Changelog

## [2.2.0] - 2026-03-07

### Features

1. Add target render size

### Documentation

1. *(release)* Don't skip any proper commits
2. *(release)* Correct whitespace in changelog
3. Update SLoC and complexity count

### Miscellaneous Tasks

1. *(sumi)* Add `revert` as commit type
2. Temporary commit
3. *(deps)* Don't require patch version

### Revert

1. Chore: don't put chore(release) in `CHANGELOG.md`

<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>Refs</td>
<td>a2c0f9d5fbd28edd84d6f2fe2e2718f05bd1bb6a</td>
</tr>
</table>

## [2.1.0] - 2026-03-07

### Features

1. Add colour tinting support

Parts taken from [生于斯](https://github.com/shengyusi-SYS)'s
[fork of this repository](https://github.com/shengyusi-SYS/bevy_svg_ui)

### Documentation

1. *(examples)* Add color examples
2. *(README)* Update `README.md` for new examples
3. *(README,style)* Remove `.rs` from example list
4. *(release)* Use html table instead of md table

Markdown tables break when the value is multi-line. `HTML` tables do
not.

5. *(release)* Simplify whitespace
6. *(release)* Update `CHANGELOG.md` to new format

### Miscellaneous Tasks

1. *(release)* V2.1.0

## [2.0.0] - 2026-03-07

### Features

1. Explicitly state which files are supported
2. Add SVG type for UI rendering

### Bugfixes

1. Properly handle all `AssetEvent`s

- Replace single monolithic event handler system with dedicated handlers
  for each `AssetEvent` variant.
- Add support for the `Removed` and `Unused` events.
- Properly handle the `Modified` event, which didn't actually update the
  `Sprite before`

<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>NOTE</td>
<td>`Added` isn't actually handled, but it isn't supposed to either.
We react to `LoadedWithDependencies` instead.</td>
</tr>
<tr>
<td>Fix</td>
<td>#1</td>
</tr>
</table>

2. *(README)* Correct factual error about bevy_svg

### Other

1. `all-asset-events` into `main`

### Documentation

1. Check off more `AssetEvent`s on todo-list
2. Update status for `bevy_svg`
3. Update our support for hot-reloading
4. Add comparison with Bevy Vello

<table>
<tr>
<th>Token</th>
<th>Value</th>
</tr>
<tr>
<td>Link</td>
<td>https://github.com/linebender/bevy_vello</td>
</tr>
</table>

5. Add JIT to Todo list
6. Include note about Inkscape SVGs
7. Add UI example
8. *(README)* Update `README.md` to mention UI
9. *(spelling)* Fix spelling mistake in doccomment
10. *(release)* Don't use h4
11. *(release)* Update `CHANGELOG.md` format

### Performance

1. Use `HashSet` instead of `Vec` for events

`HashSet`s are supposedly faster than `Vec`s for `contain` calls,
although I haven't actually tested it.

2. Check if asset id list is empty before loop

### Styling

1. *(sumi)* Sort allowed types array
2. *(README)* Fix hard-wrapping in README

### Miscellaneous Tasks

1. *(sumi)* Add `merge` as commit type
2. *(deps)* Bump dependencies
3. *(deps)* Don't require patch version
4. Add `debug` as bevy dev feature
5. Enable default features for bevy dev builds
6. Add helper event function
7. *(release)* Ignore non-conventional commits
8. *(release)* Create release script
9. *(release)* V2.0.0

## [1.0.1] - 2026-01-24

### Bugfixes

1. Make `zoom.rs` example compile

### Other

1. *(release)* Release v1.0.1

### Documentation

1. Add badges
2. Clarify how SLoC is counted
3. Add documentation for each SvgError variant
4. Update SLoC count
5. *(release)* Create changelog for v1.0.0
6. *(release)* Update `CHANGELOG.md` for 1.0.1

### Styling

1. Make clippy like my `README.md` file
2. Remove superfluous `default` call in zoom

### Miscellaneous Tasks

1. Restrict visibility of internal method
2. Add git-cliff
3. Exclude `cliff.toml` file
4. Configure git-cliff
5. *(release)* Add `release.toml` file
6. Exclude `release.toml` file
7. *(release)* Add more release configurations
8. Don't put chore(release) in `CHANGELOG.md`

## [1.0.0] - 2026-01-24

### Features

1. Create a boilerplate SVG-loading plugin
2. Add initial working version
3. Relax dependencies
4. Warn when unimplemented events are emitted
5. Make internal types private
6. Add example for what happens when you zoom
7. Add prelude

### Refactor

1. [*BREAKING*] Rename types to be less technical

### Documentation

1. Document code
2. Rewrite documentation to remove dead links
3. Add `CONTRIBUTING.md`
4. Add content to `README.md`
5. Add migration guide from bevy_svg
6. Remove unsupported angle brackets for links

### Styling

1. *(Cargo.toml)* Sort `package` field
2. Sort `description` field
3. Sort `exclude` field

### Miscellaneous Tasks

1. Initial commit
2. Release version 1.0.0
3. Add categories, keywords and a description
4. Exclude `sumi.toml` file from <crates.io>
