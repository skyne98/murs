# Module
Modules reside in a library and consist of a `module.toml` file and a number of units.

## Unit
Units are [Markdown](https://www.markdownguide.org/cheat-sheet/) files containing small, digestible pieces of a system or lore. Each of those units is named according to their file name without an extension (`.md`).

## Patching
`module.toml` contains a `[patch]` section which contains a list of module overrides. Those overrides are taken into account only if the current module is the top-level module being packaged.

`abstract = true` will mark your module as an abstract module — a module without an implementation. This will put this module into the `Abstract modules` section of the generated book, informing users about missing implementations.

`implements = <module-name>` will mark your module as an implementation of some other module. A module needs to be an implementation of a different module in order to "patch" it.