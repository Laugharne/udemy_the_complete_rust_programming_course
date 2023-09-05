### 📖 Table of Contents

<!-- TOC -->

- [📖 Table of Contents](#-table-of-contents)
- [📍 Overview](#-overview)
- [🌲 Repository Tree](#-repository-tree)

<!-- /TOC -->

----

### 📍 Overview

As well as practical projects, nearly every section of the course has a dedicated student assignment to complete! Each assignments tests your new skills and helps give you the confidence to tackle your own projects going forward!

In this course I will cover:
- Common programming concepts (fundamental types, functions, control flow)
- Ownership and moves
- References
- Structs
- Enums and Patterns
- Error Handling
- Crates and Modules
- Traits and Generics
- Iterators
- Collections
- Concurrency
- Webassembly

And much, much more!

By the end of this course you will have started at the basics of programming in **Rust**

----

### 🌲 Repository Tree
```bash
.
├── README.md
├── section_01
├── section_02
│   ├── control_flow
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── data_types
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── exercice_1
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── functions
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── hello
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── variables
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_03
│   ├── exercice_2
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── ownership
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_04
│   ├── exercice_3
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── structs
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_05
│   ├── enums_pattern_matching
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── exercice_4
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_06
│   ├── exercice_5
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── exercice_5_2
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── generics_and_traits
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_07
│   └── todo
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           ├── lib.rs
│           ├── things_todo
│           │   └── items_completed.rs
│           └── things_todo.rs
├── section_08
│   ├── binary_heap
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── maps
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── sets
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── vectors
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_09
│   ├── catch
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── error.txt
│   │   └── src
│   │       └── main.rs
│   ├── panic
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── propagation
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── renamed.txt
│   │   └── src
│   │       └── main.rs
│   ├── result
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── unwrap
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_10
│   ├── assertions
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   └── testing
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── section_11
│   └── cli
│       ├── Cargo.lock
│       ├── Cargo.toml
│       ├── Copy.toml
│       ├── src
│       │   ├── find_and_replace.rs
│       │   └── main.rs
│       ├── test-mod.txt
│       └── test.txt
├── section_12
│   ├── closures
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── exercice_6
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── fn_traits
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── iterators
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── type_annotation_and_performance
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_13
│   ├── box_deref
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── exercice_7_1
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── exercice_7_2
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── rc_and_arc
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── refcell
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_14
│   ├── channels
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── channels_2
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── exercice_8
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── mutex_poisoning
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── rayon
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── rayon_2
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── send_and_sync
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── shared_state
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── shared_state_2
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── spawn_and_join
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── spawn_and_join_3
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_15
│   ├── exercice_9
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── macros
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_16
│   └── unsafe_code
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_17
│   ├── chat_103
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── bin
│   │       │   ├── client.rs
│   │       │   └── server
│   │       │       ├── chats_map.rs
│   │       │       ├── chats.rs
│   │       │       ├── connection.rs
│   │       │       └── main.rs
│   │       ├── lib.rs
│   │       └── utils.rs
│   ├── chat_105
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── bin
│   │       │   ├── client.rs
│   │       │   └── server
│   │       │       ├── chats_map.rs
│   │       │       ├── chats.rs
│   │       │       ├── connection.rs
│   │       │       └── main.rs
│   │       ├── lib.rs
│   │       └── utils.rs
│   └── future_async_await
│       ├── Cargo.lock
│       ├── Cargo.toml
│       ├── read.txt
│       └── src
│           └── main.rs
├── section_18
│   └── wasm-game-of-life
│       ├── Cargo.lock
│       ├── Cargo.toml
│       ├── LICENSE_APACHE
│       ├── LICENSE_MIT
│       ├── pkg
│       ├── README.md
│       ├── src
│       │   ├── lib.rs
│       │   └── utils.rs
│       ├── tests
│       │   └── web.rs
│       └── www
│           ├── bootstrap.js
│           ├── index.html
│           ├── index.js
│           ├── LICENSE-APACHE
│           ├── LICENSE-MIT
│           ├── package.json
│           ├── package-lock.json
│           ├── README.md
│           └── webpack.config.js
├── section_19
├── section_20
│   ├── assignment
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── factorial
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── fibonacci
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── palindrome
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_21
│   ├── bubble_sort
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── merge_sort
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── quick_sort
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── selection_sort
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── section_22
│   └── linked_list
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── section_23
│   └── stack_queue
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── section_24
│   └── bst
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── section_25
│   └── dynamic
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── section_26
│   └── graphs
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
└── tmp.txt

172 directories, 244 files
```

----
