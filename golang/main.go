package main

import (
	"fmt"
	"sync"
	"time"
)

// A MemoryCache stores key/value pairs in-memory. Keys are strings.
type MemoryCache struct {
	storage sync.Map
}

func NewMemoryCache() *MemoryCache {
	// No need to initialize like we would a standard map; from the
	// `sync` docs: "The zero Map is empty and ready for use."
	return &MemoryCache{}
}

// Set unconditionally sets a key in the cache to the given value. The
// key will be removed after the given ttl has elapsed.
func (mc *MemoryCache) Set(key string, value interface{}, ttl time.Duration) {
	mc.storage.Store(key, value)
	time.AfterFunc(ttl, func() {
		mc.storage.Delete(key)
	})
	// The underlying Store operation always succeeds, and the delayed
	// Delete as well, so there's no need for error tracking here.
	return
}

// GetOrSet returns the existing value for the key if
// present. Otherwise it stores the given value with the provided
// expiration and returns the given value. The loaded result is true
// if the value was present, false otherwise.
func (mc *MemoryCache) GetOrSet(key string, value interface{}, ttl time.Duration) (actual any, loaded bool) {
	val, loaded := mc.storage.LoadOrStore(key, value)
	if !loaded {
		time.AfterFunc(ttl, func() {
			mc.storage.Delete(key)
		})
	}

	return val, loaded
}

// Get returns the value stored in the cache for the given key, or nil
// if no value is stored. The ok result is true if the key was found
// in the cache, false otherwise.
func (mc *MemoryCache) Get(key string) (value any, ok bool) {
	return mc.storage.Load(key)
}

// Expire immediately removes the given key from the cache, returning
// the value if it was present, or nil if no value was stored. The
// loaded result is true if the key was present in the cache, false
// otherwise.
func (mc *MemoryCache) Expire(key string) (value any, loaded bool) {
	return mc.storage.LoadAndDelete(key)
}

// Refresh sets the TTL for the given key, if it is present, returning
// true if the key was present (and thus updated), false otherwise.
func (mc *MemoryCache) Refresh(key string, ttl time.Duration) (refreshed bool) {
	value, ok := mc.Get(key)
	if ok {
		mc.Set(key, value, ttl)
		return true
	}
	return false
}

// ExpireAll expires all the cache entries, resulting in an empty cache.
func (mc *MemoryCache) ExpireAll() {
	mc.storage.Clear()
}

func main() {
	cache := NewMemoryCache()

	fmt.Println("Setting up cache...")
	cache.Set("UltimateAnswer", 42, time.Until(time.Now().Add(5*time.Minute)))
	cache.Set("Spock", "Live long and prosper.", time.Until(time.Now().Add(10*time.Second)))

	fmt.Println("Searching for answers...")
	value, ok := cache.Get("UltimateAnswer")
	if !ok {
		fmt.Println("Failed to answer the ultimate question.")
	} else {
		fmt.Println("The answer is, of course, ", value, ".")
	}
	cache.Expire("UltimateAnswer")

	fmt.Println("Searching for Spock...")
	time.Sleep(15 * time.Second)
	value, found := cache.GetOrSet("Spock", "Live long and prosper.", time.Until(time.Now().Add(5*time.Minute)))
	if found {
		fmt.Println("Found Spock, that was unexpected!")
	} else {
		fmt.Println("Spock not found, releasing Genesis device.")
		fmt.Println(value)
	}
}
