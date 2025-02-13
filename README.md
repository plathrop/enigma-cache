# enigma-cache
A toy in-memory cache.

# Design

This implementation assumes each key will be written once and read
many times, rather than being mutated in place often. With this in
mind we use sync.Map as our underlying storage to provide thread
safety. sync.Map is optimized for the access pattern we're
assuming. If our access pattern changes, we would need to consider
swapping to a different implementation.

## Improvements

In the future some things I might add:

* Tests! For something this simple, in the time I had, a simple
  exercise in main() was what I went for, but in the future a suite of
  unit tests is definitely called for before this is used in an
  application.
* Different cache types using different backends, and a generic interface for them.
* Store TTLs alongside the keys in a way that allows them to be
  retrieved or potentially updated.
* Better expiration handling. The current implementation simply fires
  off a goroutine using Time.AfterFunc to delete the key. There are a
  few potential pitfalls with this and I'd consider separately
  tracking TTLs and perhaps using a dedicated reaper goroutine. This
  fits in with the previous point.
* Cache metrics. Any serious cache implementation would allow for
  querying cache hits/misses, etc.
* A server wrapper.
