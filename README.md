Advent of Code 2021
===================

These are my solutions for the 2021 edition of the [**Advent of Code**](https://adventofcode.com).  
The solutions are all implemented using both [**Rust**](https://rust-lang.org) and [**SOM (Simple Object Machine)**](https://som-st.github.io).  

How to run
----------

Here is how you can run each of the [Rust](https://rust-lang.org) solutions:

```bash
# NOTE: You can add the '--release' flag to these commands to build with optimizations enabled.

# This will run the solution for day 1.
cargo run -- 1

# This will run the solution for day 2.
cargo run -- 2

# And so on, up to day 24 ...
```

Here is how you can run each of the SOM solutions using [SOM-java](https://github.com/SOM-st/som-java) or [SOM-rs](https://github.com/Hirevo/som-rs):

```bash
# using `som-java` for day 1 and day 2:
${SOM_JAVA_DIR}/som.sh -cp ${SOM_CORE_LIB_DIR}:som AoC 1
${SOM_JAVA_DIR}/som.sh -cp ${SOM_CORE_LIB_DIR}:som AoC 2

# using `som-rs` for day 1 and day 2:
som-interpreter-bc -c ${SOM_CORE_LIB_DIR} som -- AoC 1
som-interpreter-bc -c ${SOM_CORE_LIB_DIR} som -- AoC 2
```

You can also call each day using their class more directly, the following way:

```bash
# using `som-java` for day 1 and day 2:
${SOM_JAVA_DIR}/som.sh -cp ${SOM_CORE_LIB_DIR}:som Day01
${SOM_JAVA_DIR}/som.sh -cp ${SOM_CORE_LIB_DIR}:som Day02

# using `som-rs` for day 1 and day 2:
som-interpreter-bc -c ${SOM_CORE_LIB_DIR} som -- Day01
som-interpreter-bc -c ${SOM_CORE_LIB_DIR} som -- Day02
```
