# Rebound: A simple vector editor

The orignal idea for the application was from the excellent [Dotgrid](https://hundredrabbits.itch.io/dotgrid) tool from [100r](https://100r.co/site/home.html). 
It supports many of the features offered by that app, although not as a web-app, 
but is also dirverging to support some of my research interests in musical interface 
design an in particular my work on [SVG Interfaces](https://muses-dmi.github.io/svg_interfaces/overview/). More to come on this soon.

# Key bindings


# Releases

## Version 0.1

   * [Mac OS (10.11)]("releases/Rebound-Installer.dmg")

# Rust

It is implemented using stable [Rust](https://www.rust-lang.org/) using the Amethyst game engine. Why use a game engine for something that is not a game? Well mostly because I like it and I'm trying to explore different ideas to better understand the engine itself. It worked out fine, although I'm sure it was probably not the best approach given the application area. However, as the feature expand more towards support for the SVG interfaces, then Amethyst plays a more key role.

To install Rust go you need simply to install [Rustup](https://rustup.rs/) and if you already have Rust installed, then you can update with the command rustup update.

# Building

# Building OS Installers

To build OS specfic application bundles [cargo bundle](https://github.com/burtonageo/cargo-bundle) is used and needs to be installed.

## Mac OS

Set the feature *osx_bundle* in ```Cargo.toml```, simply uncomment and comment the appropate lines.

To build the app bundle simple run the command:

```bash
cargo bundle --release
```

The resulting app is in 

```bash
./target/release/bundle/osx/
```

A Mac OS installer (i.e. *.dmg*) can be created using the comamnd It uses the [create-dmg](https://github.com/create-dmg/create-dmg) (installed via *brew*). Simple run the command:

```
source create_dmg.sh
```

which will create ```Rebound-Installer.dmg``` in the directory:

```bash
./target/release/bundle/osx/
```

# LICENSE

Licensed under any of

Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
Mozilla Public License 2.0

at your option.

Dual MIT/Apache2 is strictly more permissive.