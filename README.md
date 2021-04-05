# dede
DErive DEref

there were already some macros for deriving `Deref`
but wasn't flexible enough

this macro supports structs with generic types and tuple structs

```rust
use dede::*;

#[derive(Deref, DerefMut)]
pub struct Foo {
	#[deref]
	bar: usize
}
```
