// Device-side malloc and free for AGX.
//
// The heap is one buffer of device memory per context (CU_LIMIT_MALLOC_HEAP_SIZE bytes, zeroed), whose address
// zluda_agx writes into each module's __zluda_agx_heap when it loads the module. It starts with a header: the bump
// offset of the space never allocated, its size, and a free list per size class. Blocks are powers of two from 32
// bytes, with 16 bytes before the 16-byte aligned memory a pointer points to: the block's class and, while it is
// free, the offset of the next free block. A freed block goes back to the list of its class and is reused by the
// next allocation of that class; nothing is ever merged or returned to the bump space.
//
// The lists are lock-free: the head is a 32-bit offset and a 32-bit count of the changes to it, swapped by a 64-bit
// compare-and-swap, so a thread that read a block's next offset before another thread took the block and freed it
// again fails its swap. malloc returns NULL once the heap has no room for a block.
//
// Compiled to LLVM IR by ptx/lib/gen_agx_tex.py (clang -target nvptx64) into the generated section of
// zluda_ptx_impl_agx.ll; regenerate after editing.

#include <stdint.h>

#define GLOBAL __attribute__((address_space(1)))
#define API __attribute__((always_inline))

enum { CLASSES = 32, MIN_CLASS = 5, BLOCK_HEADER = 16 };

struct heap {
    uint32_t bump;
    uint32_t size;
    uint32_t pad[2];
    uint64_t free[CLASSES];
};

// the heap's address, written by the runtime (externally initialized in the hand-written part)
extern GLOBAL struct heap *GLOBAL __zluda_agx_heap;

static uint32_t header_bytes(void) {
    return (sizeof(struct heap) + 15) & ~15u;
}

static GLOBAL uint32_t *block(GLOBAL struct heap *h, uint32_t offset) {
    return (GLOBAL uint32_t *)((GLOBAL uint8_t *)h + offset);
}

// (PTX's malloc and free take and return 64-bit generic addresses, which are device addresses here)
API uint64_t __zluda_ptx_impl_malloc(uint64_t size) {
    GLOBAL struct heap *h = __zluda_agx_heap;
    if (h == 0 || size > (1ull << 31) - BLOCK_HEADER)
        return 0;
    uint32_t class = MIN_CLASS;
    while ((1ull << class) < size + BLOCK_HEADER)
        class++;
    GLOBAL uint64_t *head = &h->free[class];
    uint64_t old = __atomic_load_n(head, __ATOMIC_ACQUIRE);
    uint32_t offset;
    for (;;) {
        offset = (uint32_t)old;
        if (offset == 0)
            break;
        uint32_t next = __atomic_load_n(&block(h, offset)[1], __ATOMIC_RELAXED);
        uint64_t new = ((old >> 32) + 1) << 32 | next;
        if (__atomic_compare_exchange_n(head, &old, new, 0, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE))
            break;
    }
    if (offset == 0) {
        uint32_t bytes = 1u << class;
        uint32_t room = __atomic_load_n(&h->size, __ATOMIC_RELAXED) - header_bytes();
        uint32_t start = __atomic_load_n(&h->bump, __ATOMIC_RELAXED);
        do {
            if (bytes > room - start)
                return 0;
        } while (!__atomic_compare_exchange_n(&h->bump, &start, start + bytes, 0, __ATOMIC_RELAXED,
                                              __ATOMIC_RELAXED));
        offset = header_bytes() + start;
    }
    block(h, offset)[0] = class;
    return (uint64_t)((GLOBAL uint8_t *)block(h, offset) + BLOCK_HEADER);
}

API void __zluda_ptx_impl_free(uint64_t pointer) {
    GLOBAL struct heap *h = __zluda_agx_heap;
    if (pointer == 0 || h == 0)
        return;
    uint32_t offset = (uint32_t)(pointer - (uint64_t)h) - BLOCK_HEADER;
    uint32_t class = block(h, offset)[0];
    if (class < MIN_CLASS || class >= CLASSES)
        return;
    GLOBAL uint64_t *head = &h->free[class];
    uint64_t old = __atomic_load_n(head, __ATOMIC_RELAXED);
    for (;;) {
        __atomic_store_n(&block(h, offset)[1], (uint32_t)old, __ATOMIC_RELAXED);
        uint64_t new = ((old >> 32) + 1) << 32 | offset;
        if (__atomic_compare_exchange_n(head, &old, new, 0, __ATOMIC_RELEASE, __ATOMIC_RELAXED))
            return;
    }
}
