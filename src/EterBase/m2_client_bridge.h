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
bool     m2_net_send_item_move(uint8_t src_win, uint16_t src_cell, uint8_t dst_win, uint16_t dst_cell, uint8_t count);
bool     m2_net_send_whisper(const char* target, const char* msg);
bool     m2_net_send_shoot(uint8_t b_type);
bool     m2_net_send_use_skill(uint32_t skill_vnum, uint32_t target_vid);

// ============================================================================
// Network Callbacks & Observation Pump (A3.2)
// ============================================================================
typedef struct M2NetCallbacks {
    void (*on_char_spawn)(uint32_t vid, int32_t x, int32_t y, uint16_t race);
    void (*on_char_move)(uint32_t vid, int32_t x, int32_t y, uint8_t rot);
    void (*on_chat)(uint8_t chat_type, uint32_t sender_vid, const char* msg);
    void (*on_item_set)(uint8_t window, uint16_t cell, uint32_t vnum, uint8_t count);
    void (*on_damage)(uint32_t victim_vid, int32_t damage, uint8_t flag);
} M2NetCallbacks;

bool     m2_net_set_callbacks(const M2NetCallbacks* callbacks);
void     m2_net_observe_recv(const uint8_t* data, size_t len);

// ============================================================================
// Game Logic, Player & Actor State (A4)
// ============================================================================
bool     m2_player_get_position(float* out_x, float* out_y, float* out_rot);
int64_t  m2_player_get_status(uint32_t status_type);
uint32_t m2_player_get_item_vnum(uint8_t window, uint16_t slot);
uint32_t m2_player_get_item_count(uint8_t window, uint16_t slot);
int32_t  m2_player_get_hp(void);
int32_t  m2_player_get_max_hp(void);
bool     m2_player_is_alive(void);
uint32_t m2_find_nearest_actor(uint32_t actor_type, float max_distance);
float    m2_actor_get_distance(const void* vid);
bool     m2_mount_is_mounted(void);
uint32_t m2_mount_get_vnum(void);

// ============================================================================
// String Utilities & Sanitization
// ============================================================================
bool     m2_string_sanitize(const char* in_str, char* out_buf, size_t out_len);
bool     m2_string_strip_colors(const char* in_str, char* out_buf, size_t out_len);
bool     m2_string_normalize(const char* in_str, char* out_buf, size_t out_len);

// ============================================================================
// Engine Lifecycle (A1)
// ============================================================================
bool     m2_engine_init(void);
void     m2_engine_shutdown(void);

// ============================================================================
// Phase B2: Combat & Combo Engine
// ============================================================================
bool     m2_battle_can_attack(uint32_t attacker_vid, uint32_t victim_vid);
bool     m2_battle_combo_attack(float dir_rot);
uint32_t m2_battle_get_combo_index(void);
void     m2_battle_reset_combo(void);

// ============================================================================
// Phase B4: Image, DXT & Texture Decoders
// ============================================================================
bool     m2_image_decode_dxt(uint32_t dxt_type, const uint8_t* data_ptr, size_t len, uint32_t width, uint32_t height, uint8_t* out_buf);
bool     m2_image_decode_tga(const uint8_t* data_ptr, size_t len, uint32_t* out_width, uint32_t* out_height, uint8_t* out_buf, size_t max_buf_len);
bool     m2_image_apply_colorkey(uint8_t* rgba_buf, size_t len, uint8_t r, uint8_t g, uint8_t b);

// ============================================================================
// Phase B6: Effects, Particle Physics & Flying Projectiles
// ============================================================================
uint32_t m2_effect_create(uint32_t effect_id, float x, float y, float z);
bool     m2_effect_update(uint32_t instance_id, float dt);
bool     m2_effect_destroy(uint32_t instance_id);
uint32_t m2_flying_spawn(uint32_t fly_type, float sx, float sy, float sz, float tx, float ty, float tz, uint32_t target_vid);
bool     m2_flying_update(uint32_t fly_id, float dt, float* out_x, float* out_y, float* out_z, bool* out_hit);
bool     m2_particle_physics_step(float* pos_3f, float* vel_3f, float dt, float gravity, float air_resistance);
float    m2_flying_calculate_step(float* pos_3f, float* vel_3f, const float* acc_3f, float gravity, float dt);

// ============================================================================
// Phase B7: Camera 3D, Frustum Culling & View State
// ============================================================================
bool     m2_render_init(uint32_t width, uint32_t height);
void     m2_camera_set_target(float x, float y, float z);
void     m2_camera_rotate(float yaw, float pitch);
void     m2_camera_zoom(float delta_dist);
bool     m2_camera_get_eye(float* out_x, float* out_y, float* out_z);
bool     m2_render_draw_ui_quad(float x, float y, float w, float h, uint32_t color_argb);
bool     m2_frustum_build(const float* mat_16_floats);
int32_t  m2_frustum_test_sphere(float cx, float cy, float cz, float radius);
int32_t  m2_frustum_test_aabb(float min_x, float min_y, float min_z, float max_x, float max_y, float max_z);

// ============================================================================
// Phase B5: Movement & Terrain Collision
// ============================================================================
bool     m2_move_to(float target_x, float target_y);
bool     m2_move_to_direction(float dir_rot);
float    m2_get_height_at(float x, float y);
bool     m2_is_walkable(float x, float y);
void     m2_stop_movement(void);
float    m2_terrain_get_height(float local_x, float local_y);
bool     m2_terrain_is_blocked(uint16_t attr_x, uint16_t attr_y);
bool     m2_terrain_is_water(uint16_t attr_x, uint16_t attr_y);

#ifdef __cplusplus
}
#endif

#endif // __INC_M2_CLIENT_BRIDGE_H__
