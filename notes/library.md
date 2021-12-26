# Library
Any module exists in a library. This library is either declared explicitly in a `library.toml` file or assumed implicitly if you are working with a single module. Single `module.toml` without a library is assumed to be a single-module library.

In order to have multiple modules in one repository, you need to place a `library.toml` file in its root, filling in the paths to the modules.

## Links
To create a more vast ecosystem, libraries can add URLs of other libraries to their `links` section. Before resolution of any modules a library directed graph needs to be built. The roots of this graph are [the library of MURS](https://github.com/skyne98/murs-library) and libraries you optionally mentioned in your dependencies. This tree is then traversed until the leaves are reached, and all library repositories are checked-out.