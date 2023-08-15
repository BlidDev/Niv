# Overview:

This chapter will exaplain how to quickly get everything up and running for you to code in Nest.

___
## Precompiled binary
Precompiled binaries of Nest can be found and dowloaded [here](www.youtube.com/c/BlidDev), once dowloading the file you're good to go continue on to the next tutorials.

___
## Building Nest from source


**Before starting**: Nest requires `sfml-rs` to be installed in order to get compiled succesfuly, so please make sure you follow the [sfml-rs installation guide](https://github.com/jeremyletang/rust-sfml/wiki) for your OS before building and running Nest.

Retrieve a copy of the git repo by dowloading an archive **.zip** file and extracting it or by running the following line in the terminal:

```Bash
git clone https://github.com/BlidDev/Nest
```

Then `cd` into the folder:

```Nest
cd Nest
```

And simply run:
```Nest
cargo build --release
```

**The `Nest` executeable file will be located in `target/release/YOUR_TARGET/nest(.exe)`**

Or you can just build an run it directly with:
```Nest
cargo run -- -f path/to/script.nst
```
