# ug

A tool for generating and parsing UUIDs.

## Installation

Clone this repository, `cd` into the repository, and run the following command:

```shell
cargo install --path .
```

## Usage

Generate a v4 UUID:

```shell
$ ug
bad78a0e-d43a-4999-9a54-f36964f54aa5
```

Or explicitly request v4:

```shell
$ ug v4
8cf87ffb-8f26-48cc-88d4-9be35cecf98f
```

Format a UUID with uppercase letters:

```shell
$ ug -u
CF4070FE-8EDF-4840-9BBF-16DE5F4CD847

```

Output a binary UUID to a file:

```shell
$ ug -b | tee uuid.bin | hexyl
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 74 5d 6c 61 3d cc 4f 77 ┊ 84 28 1a cb 82 77 4f b0 │t]la=×Ow┊×(•××wO×│
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
```

Then read that UUID back and print it as text:

```shell
$ ug < uuid.bin
745d6c61-3dcc-4f77-8428-1acb82774fb0
```

Convert a string-formatted UUID into binary:

```shell
$ echo 0f1878cc-7a14-4f6c-b82f-7c623bc3764a | ug -b | hexyl
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 0f 18 78 cc 7a 14 4f 6c ┊ b8 2f 7c 62 3b c3 76 4a │••x×z•Ol┊×/|b;×vJ│
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
```

Generate a v5 UUID from some input:

```shell
$ echo -n hello world | ug v5
25024589-1a7c-5625-bdb2-81143473d4d3
```

Show help:

```shell
$ ug --help
USAGE: ug [OPTIONS] [VERSION]

Arguments:
  [VERSION]        Which version of UUID to use. Options are v4 (default) and v5.

Options:
  -l, --lowercase  Output UUID using lowercase letters (the default).
  -u, --uppercase  Output UUID using uppercase letters.
  -b, --binary     Output UUID in binary format.
  -h, --help       Print this help message.
```
