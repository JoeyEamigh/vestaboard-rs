# vestaboard-rs

`vestaboard-rs` is a [VBML (Vestaboard Markup Language)](https://docs.vestaboard.com/docs/vbml) parser and api client for the [Vestaboard](https://vestaboard.com/). It supports the v2 read/write api, subscription api, and local api.

the full docs can be found at <https://docs.rs/vestaboard>

## features

- [x] VBML parser
- [x] read/write api
- [x] subscription api
- [x] local api
- [x] serialization and deserialization of Vestaboard messages
- [x] async/await
- [x] support for multiple Vestaboard sizes (if another size is released)

## installation

```sh
cargo add vestaboard -F full
```

## feature flags

- `full`: enables all features
- `parser`: enables the VBML parser (Vestaboard Markup Language) (default)
- `rw`: enables the read/write api
- `subscription`: enables the subscription api
- `local`: enables the local api

## helpful type and structs

- `board::FLAGSHIP_ROWS` and `board::FLAGSHIP_COLS`: the dimensions of the flagship Vestaboard
- `board::Board<ROWS, COLS>`: a type equivalent to `[[u8; COLS]; ROWS]` which represents a Vestaboard state
- `BoardData<ROWS, COLS>`: a struct that wraps a `Board<ROWS, COLS>` and has several helper methods
- `VBML`: a struct that represents a VBML message and can be parsed into a `BoardData<ROWS, COLS>`
- `Vestaboard<Config>`: the main struct that is used to interact with the Vestaboard api

## VBML usage

with ROWS and COLS specified:

```rust
let string = "{\"props\":{},\"style\":{},\"components\":[]}"; // any valid VBML string
let vbml: Vbml<6, 22> = string.parse().unwrap();

let board_data: Result<BoardData<6, 22>, VbmlError> = vbml.parse();
```

when ROWS and COLS are not specified, the default is `board::FLAGSHIP_ROWS` and `board::FLAGSHIP_COLS`:

```rust
let string = "{\"props\":{},\"style\":{},\"components\":[]}"; // any valid VBML string
let vbml: Vbml = string.parse().unwrap();

let board_data: Result<BoardData, VbmlError> = vbml.parse();
```

## api usage

```rust
use vestaboard::{Vestaboard, RWConfig, SubscriptionConfig, LocalConfig};

#[tokio::main]
async fn main() {
  let rw_config = RWConfig { read_write_key: "<YOUR_RW_API_KEY>" };
  let rw_api: Vestaboard<RWConfig> = Vestaboard::new_rw_api(rw_config);

  let subscription_config = SubscriptionConfig {
    api_key: "<YOUR_SUBSCRIPTION_API_KEY>",
    api_secret: "<YOUR_SUBSCRIPTION_API_SECRET>",
  };
  let subscription_api: Vestaboard<SubscriptionConfig> = Vestaboard::new_subscription_api(subscription_config);

  let local_config = LocalConfig {
    api_key: "<YOUR_LOCAL_API_KEY>",
    host: "<YOUR_VESTABOARD_IP_ADDRESS>",
  };
  let local_api: Vestaboard<LocalConfig> = Vestaboard::new_local_api(local_config);
}
```

(note that you must have the `Vestaboard<Config>` type specified due to [this rust issue](https://github.com/rust-lang/rust/issues/98931))

### read/write api

```rust
use vestaboard::{Vestaboard, RWConfig};

#[tokio::main]
async fn main() {
  let rw_config = RWConfig { read_write_key: "<YOUR_RW_API_KEY>" };
  let rw_api: Vestaboard<RWConfig> = Vestaboard::new_rw_api(rw_config);

  let message: Result<RWApiReadMessage, RWApiError> = rw_api.read().await;
  let write_res: Result<String, RWApiError> = rw_api.write(BoardData<ROWS, COLS>).await;
}
```

### subscription api

```rust
use vestaboard::{Vestaboard, SubscriptionConfig};

#[tokio::main]
async fn main() {
  let subscription_config = SubscriptionConfig {
    api_key: "<YOUR_SUBSCRIPTION_API_KEY>",
    api_secret: "<YOUR_SUBSCRIPTION_API_SECRET>",
  };
  let subscription_api: Vestaboard<SubscriptionConfig> = Vestaboard::new_subscription_api(subscription_config);

  let subscriptions: Result<SubscriptionsList, SubscriptionApiError> = subscription_api.get_subscriptions().await;
  let write_res: Result<SubscriptionMessageResponse, SubscriptionApiError> = subscription_api.write(BoardData<ROWS, COLS>).await;
}
```

### local api

```rust
use vestaboard::{Vestaboard, LocalConfig};

#[tokio::main]
async fn main() {
  // if you have not enabled the local api, you can use the following method to do so.
  // note that the local api can only be enabled once per board, so make sure to save
  // the resulting api key. to get the enablement token, visit https://www.vestaboard.com/local-api
  let local_api_enablement: Result<String, LocalApiError> = Vestaboard.get_local_api_key(
      Some("<YOUR_VESTABOARD_IP_ADDRESS>".parse().unwrap()),
      "<YOUR_LOCAL_API_ENABLEMENT_KEY>",
    ).await;

  let local_config = LocalConfig {
    api_key: "<YOUR_LOCAL_API_KEY>",
    ip_address: "<YOUR_VESTABOARD_IP_ADDRESS>".parse().unwrap(),
  };

  let local_api: Vestaboard<LocalConfig> = Vestaboard::new_local_api(local_config);

  let message: Result<BoardData<ROWS, COLS>, LocalApiError> = local_api.read().await;
  let write_res: Result<(), LocalApiError> = local_api.write(BoardData<ROWS, COLS>).await;
}
```

## benchmarks

this library is set up to be benchmarked against the official JavaScript VBML parsing library. benchmarks can be run using [just](https://github.com/casey/just) with the following command:

```sh
just bench
```

on a Ryzen 9 7950X on Arch Linux with Node v18.20.0 and rustc 1.77.2, the benchmark results are as follows:

| test name                                 | js μs/it | rs μs/it | difference | % faster |
| ----------------------------------------- | -------- | -------- | ---------- | -------- |
| Default Template                          | 21.23μs  | 0.40μs   | -20.83μs   | 5,338%   |
| Half Height Center                        | 19.07μs  | 0.40μs   | -18.68μs   | 4,792%   |
| Justify Left                              | 18.33μs  | 0.39μs   | -17.94μs   | 4,669%   |
| Justify Right                             | 19.07μs  | 0.39μs   | -18.68μs   | 4,838%   |
| Justify Center                            | 21.23μs  | 0.39μs   | -20.84μs   | 5,483%   |
| Justify Justified                         | 20.11μs  | 0.39μs   | -19.73μs   | 5,192%   |
| Align Center                              | 21.43μs  | 0.39μs   | -21.05μs   | 5,535%   |
| Align Top                                 | 18.98μs  | 0.39μs   | -18.58μs   | 4,812%   |
| Align Bottom                              | 21.39μs  | 0.39μs   | -21.00μs   | 5,511%   |
| Align Justified                           | 21.36μs  | 0.39μs   | -20.97μs   | 5,415%   |
| Justify Justified Align Justified         | 20.11μs  | 0.38μs   | -19.73μs   | 5,277%   |
| Split Align Justify                       | 24.49μs  | 0.42μs   | -24.08μs   | 5,897%   |
| Uneven Split                              | 23.85μs  | 0.41μs   | -23.43μs   | 5,770%   |
| Uneven Split 2                            | 21.64μs  | 0.35μs   | -21.29μs   | 6,244%   |
| Rev Split Align Justify                   | 24.55μs  | 0.42μs   | -24.13μs   | 5,850%   |
| Two Column                                | 21.61μs  | 0.35μs   | -21.26μs   | 6,153%   |
| All Justified                             | 27.32μs  | 0.42μs   | -26.90μs   | 6,490%   |
| Justified Right                           | 15.76μs  | 0.24μs   | -15.51μs   | 6,438%   |
| Centered Right                            | 15.69μs  | 0.24μs   | -15.45μs   | 6,472%   |
| 2x2x2x2 Grid                              | 37.68μs  | 0.76μs   | -36.91μs   | 4,933%   |
| 2x2 Neighbors                             | 18.90μs  | 0.37μs   | -18.53μs   | 5,046%   |
| Plain Text                                | 13.44μs  | 0.24μs   | -13.20μs   | 5,564%   |
| Centered                                  | 17.02μs  | 0.24μs   | -16.78μs   | 7,055%   |
| Newline                                   | 16.02μs  | 0.26μs   | -15.76μs   | 6,230%   |
| Character Codes                           | 227.70μs | 17.94μs  | -209.77μs  | 1,270%   |
| Character Codes with Characters           | 34.43μs  | 1.52μs   | -32.91μs   | 2,269%   |
| Dynamic Props                             | 14.48μs  | 0.65μs   | -13.82μs   | 2,223%   |
| Dynamic Props with Character Codes        | 17.75μs  | 1.13μs   | -16.62μs   | 1,571%   |
| Multiple Components                       | 49.01μs  | 2.34μs   | -46.68μs   | 2,099%   |
| Raw Characters                            | 0.90μs   | 0.04μs   | -0.86μs    | 2,126%   |
| Absolute Position Components              | 24.33μs  | 0.73μs   | -23.60μs   | 3,311%   |
| Complex Layout with Multiple Components   | 70.41μs  | 1.95μs   | -68.46μs   | 3,619%   |
| Complex Layout with Multiple Components 2 | 76.98μs  | 2.12μs   | -74.86μs   | 3,634%   |
| Diff Height Components Newline            | 30.00μs  | 0.60μs   | -29.41μs   | 5,028%   |
| JS Spec: Absolute Layout                  | 18.19μs  | 0.32μs   | -17.87μs   | 5,714%   |
| JS Spec: Absolute Layout 2                | 18.15μs  | 0.32μs   | -17.83μs   | 5,704%   |
| JS Spec: Absolute and Raw Components      | 12.08μs  | 0.48μs   | -11.59μs   | 2,496%   |

there is definitely further room for optimization, but the current performance is decent.
