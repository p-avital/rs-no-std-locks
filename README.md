# Locking Mechanisms for `no_std`
This crate provides some basic locking mechanisms for embedded systems programming, similar in nature and use to `std::sync::Mutex`.

## TODO
For now, this crate is only built around monocore Cortex-M processors, using small interrupt-free sections to work with flags as if they were atomics.

However, I don't remember interrupt-free sections to be safe from other cores' interventions, so a next step would be to implement these types with atomics on architectures that support them.

Once more implementations are available, the `recommended` module should be built using conditional compilation to provide an easy way to provide the "best" alternative available on the target platform.