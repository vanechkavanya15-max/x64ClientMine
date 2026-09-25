#ifndef __INC_M2_CLIENT_BRIDGE_H__
#define __INC_M2_CLIENT_BRIDGE_H__

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Timer & Sync
// ============================================================================
uint32_t m2_timer_init(void);
uint32_t m2_timer_get_msec(void);
void     m2_timer_set_server_msec(uint32_t server_msec);
uint32_t m2_timer_get_server_msec(void);
uint32_t m2_timer_get_frame_msec(void);
void     m2_timer_set_frame_msec(void);

// ============================================================================
// CRC32
// ============================================================================
uint32_t m2_crc32(const uint8_t* buf, size_t len);
uint32_t m2_case_crc32(const uint8_t* buf, size_t len);

// ============================================================================
// Random (Park & Miller LCG)
// ============================================================================
void     m2_srandom(uint32_t seed);
uint32_t m2_random(void);
float    m2_frandom(float low, float high);
int32_t  m2_random_range(int32_t from, int32_t to);

// ============================================================================
// TEA Cipher
// ============================================================================
void     m2_tea_code(uint32_t sz, uint32_t sy, const uint32_t* key, uint32_t* dest);
void     m2_tea_decode(uint32_t sz, uint32_t sy, const uint32_t* key, uint32_t* dest);
size_t   m2_tea_encrypt(uint8_t* dest, const uint8_t* src, const uint32_t* key, size_t size);
size_t   m2_tea_decrypt(uint8_t* dest, const uint8_t* src, const uint32_t* key, size_t size);

#ifdef __cplusplus
}
#endif

#endif // __INC_M2_CLIENT_BRIDGE_H__
