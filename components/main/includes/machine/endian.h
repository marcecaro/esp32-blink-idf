/*
 * Custom wrapper for machine/endian.h to fix include issues
 * when generating Rust bindings from C/C++ code
 */
#pragma once

#ifndef _MACHINE_ENDIAN_H_
#define _MACHINE_ENDIAN_H_

// Standard endian macros
#define __BYTE_ORDER__ __ORDER_LITTLE_ENDIAN__
#define BYTE_ORDER __BYTE_ORDER__
#define LITTLE_ENDIAN __ORDER_LITTLE_ENDIAN__
#define BIG_ENDIAN __ORDER_BIG_ENDIAN__
#define PDP_ENDIAN __ORDER_PDP_ENDIAN__

// Byte swapping functions - simplified implementations
#ifndef __ASSEMBLER__

#include <stdint.h>

// Byte swap functions
#define __bswap16(_x) ((uint16_t)((((uint16_t)(_x)) >> 8) | \
                        (((uint16_t)(_x)) << 8)))

#define __bswap32(_x) ((uint32_t)(((__bswap16(((uint32_t)(_x)) >> 16)) | \
                        ((__bswap16(((uint32_t)(_x)))) << 16))))

#define __bswap64(_x) ((uint64_t)(((__bswap32(((uint64_t)(_x)) >> 32)) | \
                        ((__bswap32(((uint64_t)(_x)))) << 32))))

// Host to/from network byte order (big endian)
#if BYTE_ORDER == BIG_ENDIAN
#define htons(x) (x)
#define htonl(x) (x)
#define ntohs(x) (x)
#define ntohl(x) (x)
#else /* LITTLE_ENDIAN */
#define htons(x) __bswap16(x)
#define htonl(x) __bswap32(x)
#define ntohs(x) __bswap16(x)
#define ntohl(x) __bswap32(x)
#endif

#endif /* !__ASSEMBLER__ */

#endif /* _MACHINE_ENDIAN_H_ */
