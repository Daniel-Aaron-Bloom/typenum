/*!
This is a generated file. It defines aliases for many constants for your convenience.

For unsigned integers, the format is `U` followed by the number. We define aliases for

- Numbers 0 through 1024
- Powers of 2 below u64::MAX
- Powers of 10 below u64::MAX

These alias definitions look like this:

```rust
# use typenum::uint::{UInt, UTerm};
# use typenum::bit::{B0, B1};
type U5 = UInt<UInt<UInt<UTerm, B1>, B0>, B1>;
```

For positive signed integers, the format is `P` followed by the number and for negative
signed integers it is `N` followed by the number. For the signed integer zero, we use
`Z0`. We define aliases for

- Numbers -1024 through 1024
- Powers of 2 between i64::MIN and i64::MAX
- Powers of 10 between i64::MIN and i64::MAX

These alias definitions look like this:

```rust
# use typenum::uint::{UInt, UTerm};
# use typenum::int::{PInt, NInt};
# use typenum::bit::{B0, B1};
# #[allow(dead_code)]
struct Z0;
type P5 = PInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>>;
type N5 = NInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>>;
```

# Example
```rust
# #[allow(unused_imports)]
use typenum::consts::{U0, U1, U2, U3, U4, U5, U6};
# #[allow(unused_imports)]
use typenum::consts::{N3, N2, N1, Z0, P1, P2, P3};
# #[allow(unused_imports)]
use typenum::consts::{U774, N17, N10000, P1024, P4096};
```
*/

include!(concat!(env!("OUT_DIR"), "/consts.rs"));
