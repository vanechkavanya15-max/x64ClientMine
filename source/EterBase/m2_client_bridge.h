#ifndef __INC_M2_CLIENT_BRIDGE_H__
#define __INC_M2_CLIENT_BRIDGE_H__

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#pragma comment(lib, "m2_client_bridge.lib")

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Timer & Sync (A1)
// ============================================================================
uint32_t m2_timer_init(void);
uint32_t m2_timer_get_msec(void);
void     m2_timer_set_server_msec(uint32_t server_msec);
uint32_t m2_timer_get_server_msec(void);
uint32_t m2_timer_get_frame_msec(void);
void     m2_timer_set_frame_msec(void);

// ============================================================================
// CRC32 & Math (A1)
// ============================================================================
uint32_t m2_crc32(const uint8_t* buf, size_t len);
uint32_t m2_case_crc32(const uint8_t* buf, size_t len);
void     m2_srandom(uint32_t seed);
uint32_t m2_random(void);
float    m2_frandom(float low, float high);
int32_t  m2_random_range(int32_t from, int32_t to);

// ============================================================================
// TEA Cipher (A1)
// ============================================================================
void     m2_tea_code(uint32_t sz, uint32_t sy, const uint32_t* key, uint32_t* dest);
void     m2_tea_decode(uint32_t sz, uint32_t sy, const uint32_t* key, uint32_t* dest);
size_t   m2_tea_encrypt(uint8_t* dest, const uint8_t* src, const uint32_t* key, size_t size);
size_t   m2_tea_decrypt(uint8_t* dest, const uint8_t* src, const uint32_t* key, size_t size);

// ============================================================================
// VFS & Archive Pack Manager (A2)
// ============================================================================
bool     m2_vfs_exists(const char* path);
uint8_t* m2_vfs_load_file(const char* path, size_t* out_size);
void     m2_vfs_free(uint8_t* data, size_t size);
void     m2_vfs_free_buffer(uint8_t* ptr, size_t size);
bool     m2_vfs_normalize_path(const char* in_path, char* out_buf, size_t out_buf_len);
bool     m2_vfs_pack_manager_register_pack(const char* pack_name, const char* root_dir);
void     m2_vfs_pack_manager_clear(void);
size_t   m2_vfs_pack_manager_get_count(void);
bool     m2_vfs_verify_crc32(const uint8_t* data, size_t len, uint32_t expected_crc);

// ============================================================================
// Network Protocol & CG Packet Senders (A3)
// ============================================================================
bool     m2_net_send_attack(uint32_t target_vid, uint8_t attack_type);
bool     m2_net_send_chat(uint8_t chat_type, const char* msg_ptr);
bool     m2_net_send_move(float x, float y, float rot, uint8_t move_type);
bool     m2_net_send_pickup(uint32_t vid);
bool     m2_net_send_use_item(uint8_t window, uint16_t slot);

// ============================================================================
// Game Logic, Player & Actor State (A4)
// ============================================================================
bool     m2_player_get_position(float* out_x, float* out_y, float* out_rot);
int64_t  m2_player_get_status(uint32_t status_type);
uint32_t m2_player_get_item_vnum(uint8_t window, uint16_t slot);
uint32_t m2_player_get_item_count(uint8_t window, uint16_t slot);
uint32_t m2_find_nearest_actor(uint32_t actor_type, float max_distance);
float    m2_actor_get_distance(const void* vid);

// ============================================================================
// String Utilities & Sanitization
// ============================================================================
bool     m2_string_sanitize(const char* in_str, char* out_buf, size_t out_len);
bool     m2_string_strip_colors(const char* in_str, char* out_buf, size_t out_len);
bool     m2_string_normalize(const char* in_str, char* out_buf, size_t out_len);

// ============================================================================
// Engine Lifecycle
// ============================================================================
void*    m2_engine_init(void);
void     m2_engine_shutdown(void* ctx);

#ifdef __cplusplus
}
#endif

#endif // __INC_M2_CLIENT_BRIDGE_H__
