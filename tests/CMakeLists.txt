cmake_minimum_required(VERSION 3.19)

add_executable(test_c26_strangler_facade test_c26_strangler_facade.cpp)

target_include_directories(test_c26_strangler_facade PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)

target_link_libraries(test_c26_strangler_facade PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)

add_executable(test_c26_e2e_pandora_simulation test_c26_e2e_pandora_simulation.cpp)

target_include_directories(test_c26_e2e_pandora_simulation PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)

target_link_libraries(test_c26_e2e_pandora_simulation PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)

add_executable(test_c26_state_unification test_c26_state_unification.cpp)

target_include_directories(test_c26_state_unification PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)

target_link_libraries(test_c26_state_unification PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)

add_executable(test_c26_mem_pool test_c26_mem_pool.cpp)

target_include_directories(test_c26_mem_pool PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)

target_link_libraries(test_c26_mem_pool PRIVATE
    EterLib
    EterBase
    sodium
    mio
    ws2_32
)

add_executable(test_c26_radix_sort test_c26_radix_sort.cpp)
target_include_directories(test_c26_radix_sort PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_radix_sort PRIVATE
    EterLib
    EterBase
)

add_executable(test_c26_render_target_mgr test_c26_render_target_mgr.cpp)
target_include_directories(test_c26_render_target_mgr PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_render_target_mgr PRIVATE
    EterLib
    EterBase
)

add_executable(test_c26_skill_cooldown test_c26_skill_cooldown.cpp)
target_include_directories(test_c26_skill_cooldown PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_skill_cooldown PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_item_bonus test_c26_item_bonus.cpp)
target_include_directories(test_c26_item_bonus PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_item_bonus PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_skill_domain test_c26_skill_domain.cpp)
target_include_directories(test_c26_skill_domain PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_skill_domain PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_inventory_domain test_c26_inventory_domain.cpp)
target_include_directories(test_c26_inventory_domain PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_inventory_domain PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_actor_registry test_c26_actor_registry.cpp)
target_include_directories(test_c26_actor_registry PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_actor_registry PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_spatial_hash_grid test_c26_spatial_hash_grid.cpp)
target_include_directories(test_c26_spatial_hash_grid PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_spatial_hash_grid PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_zero_copy_ring_buffer test_c26_zero_copy_ring_buffer.cpp)
target_include_directories(test_c26_zero_copy_ring_buffer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_zero_copy_ring_buffer PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_quickslot_domain test_c26_quickslot_domain.cpp)
target_include_directories(test_c26_quickslot_domain PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_quickslot_domain PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_simd_culling test_c26_simd_culling.cpp)
target_include_directories(test_c26_simd_culling PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_simd_culling PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_world_context test_c26_world_context.cpp)
target_include_directories(test_c26_world_context PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
)
target_link_libraries(test_c26_world_context PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_domain_errors_events test_c26_domain_errors_events.cpp)
target_include_directories(test_c26_domain_errors_events PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
)
target_link_libraries(test_c26_domain_errors_events PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_instance_components 
    test_c26_instance_components.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/InstanceComponents/InstanceCombatComponent.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/AffectFlagContainer.cpp
)
target_include_directories(test_c26_instance_components PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
)
target_link_libraries(test_c26_instance_components PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_zero_overhead_state_cache 
    test_c26_zero_overhead_state_cache.cpp
    ${CMAKE_SOURCE_DIR}/src/Client/Graphics/ZeroOverheadStateCache.cpp
)
target_include_directories(test_c26_zero_overhead_state_cache PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)

add_executable(test_c26_dynamic_geometry_ring_buffer test_c26_dynamic_geometry_ring_buffer.cpp)
target_include_directories(test_c26_dynamic_geometry_ring_buffer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
)
target_link_libraries(test_c26_dynamic_geometry_ring_buffer PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_instance_physics_engine test_c26_instance_physics_engine.cpp)
target_include_directories(test_c26_instance_physics_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/vendor
)
target_link_libraries(test_c26_instance_physics_engine PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_instance_combat_engine test_c26_instance_combat_engine.cpp)
target_include_directories(test_c26_instance_combat_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/vendor
)
target_link_libraries(test_c26_instance_combat_engine PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_type_sanitization test_c26_type_sanitization.cpp)
target_include_directories(test_c26_type_sanitization PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/vendor
)
target_link_libraries(test_c26_type_sanitization PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_msdf_text_batcher test_c26_msdf_text_batcher.cpp)
target_include_directories(test_c26_msdf_text_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/vendor
)
target_link_libraries(test_c26_msdf_text_batcher PRIVATE
    Client2026
    EterBase
)

add_executable(test_c26_hardware_mesh_instancing test_c26_hardware_mesh_instancing.cpp)
target_include_directories(test_c26_hardware_mesh_instancing PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_SOURCE_DIR}/src/GameLib
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/vendor
)
target_link_libraries(test_c26_hardware_mesh_instancing PRIVATE
    Client2026
    GameLib
    EterBase
)

enable_testing()
add_test(NAME StranglerFacadeTest COMMAND test_c26_strangler_facade)
add_test(NAME PandoraSimulationTest COMMAND test_c26_e2e_pandora_simulation)
add_test(NAME StateUnificationTest COMMAND test_c26_state_unification)
add_test(NAME MemPoolTest COMMAND test_c26_mem_pool)
add_test(NAME RadixSort64Test COMMAND test_c26_radix_sort)
add_test(NAME RenderTargetMgrTest COMMAND test_c26_render_target_mgr)
add_test(NAME SkillCooldownTest COMMAND test_c26_skill_cooldown)
add_test(NAME SkillDomainTest COMMAND test_c26_skill_domain)
add_test(NAME ItemBonusTest COMMAND test_c26_item_bonus)
add_test(NAME InventoryDomainTest COMMAND test_c26_inventory_domain)
add_test(NAME QuickslotDomainTest COMMAND test_c26_quickslot_domain)
add_test(NAME ActorRegistryTest COMMAND test_c26_actor_registry)
add_test(NAME SpatialHashGridTest COMMAND test_c26_spatial_hash_grid)
add_test(NAME ZeroCopyRingBufferTest COMMAND test_c26_zero_copy_ring_buffer)
add_test(NAME SimdCullingTest COMMAND test_c26_simd_culling)
add_test(NAME WorldContextTest COMMAND test_c26_world_context)
add_test(NAME DomainErrorsEventsTest COMMAND test_c26_domain_errors_events)
add_test(NAME InstanceComponentsTest COMMAND test_c26_instance_components)
add_test(NAME ZeroOverheadStateCacheTest COMMAND test_c26_zero_overhead_state_cache)
add_test(NAME DynamicGeometryRingBufferTest COMMAND test_c26_dynamic_geometry_ring_buffer)
add_test(NAME InstancePhysicsEngineTest COMMAND test_c26_instance_physics_engine)
add_test(NAME InstanceCombatEngineTest COMMAND test_c26_instance_combat_engine)
add_test(NAME TypeSanitizationTest COMMAND test_c26_type_sanitization)
add_test(NAME MsdfTextBatcherTest COMMAND test_c26_msdf_text_batcher)
add_test(NAME HardwareMeshInstancingTest COMMAND test_c26_hardware_mesh_instancing)

add_executable(test_c26_protocol_pillar1 test_c26_protocol_pillar1.cpp)
target_include_directories(test_c26_protocol_pillar1 PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_protocol_pillar1 PRIVATE
    Client2026
    EterBase
)

add_test(NAME ProtocolPillar1Test COMMAND test_c26_protocol_pillar1)

add_executable(test_c26_modern_packet_dispatcher test_c26_modern_packet_dispatcher.cpp)
target_include_directories(test_c26_modern_packet_dispatcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_modern_packet_dispatcher PRIVATE
    Client2026
    EterBase
    ws2_32
)
add_test(NAME ModernPacketDispatcherTest COMMAND test_c26_modern_packet_dispatcher)

add_executable(test_c26_rhi_engine test_c26_rhi_engine.cpp)
target_include_directories(test_c26_rhi_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_rhi_engine PRIVATE
    Client2026
    EterBase
)

add_test(NAME RHIEngineTest COMMAND test_c26_rhi_engine)

add_executable(test_c26_phase_game_routers
    test_c26_phase_game_routers.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetCombatRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetItemRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetActorRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetShopRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetExchangeRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetQuestRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetPartyRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetGuildRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/PhaseGamePacketDispatcher.cpp
)
target_include_directories(test_c26_phase_game_routers PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_compile_definitions(test_c26_phase_game_routers PRIVATE
    TEST_MODE_DISABLE_STDAFX
)
target_link_libraries(test_c26_phase_game_routers PRIVATE
    EterBase
)
add_test(NAME PhaseGameRoutersTest COMMAND test_c26_phase_game_routers)

add_executable(test_c33_phase_auth_routers
    test_c33_phase_auth_routers.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetLoginRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetSelectRouter.cpp
)
target_include_directories(test_c33_phase_auth_routers PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_compile_definitions(test_c33_phase_auth_routers PRIVATE
    TEST_MODE_DISABLE_STDAFX
)
target_link_libraries(test_c33_phase_auth_routers PRIVATE
    EterBase
)
add_test(NAME PhaseAuthRoutersTest COMMAND test_c33_phase_auth_routers)

add_executable(test_c26_pybridge_fastcall test_c26_pybridge_fastcall.cpp)
target_include_directories(test_c26_pybridge_fastcall PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_pybridge_fastcall PRIVATE
    Client2026
    Python
    EterBase
    version
    ws2_32
)

add_test(NAME PyBridgeFastCallTest COMMAND test_c26_pybridge_fastcall)


add_executable(test_c26_player_controllers
    test_c26_player_controllers.cpp
    player_controller_mocks.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/PlayerControllers/PlayerMovementController.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/PlayerControllers/PlayerCombatController.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/PlayerControllers/PlayerTargetController.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/PlayerControllers/PlayerSkillExecutor.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/PlayerControllers/PlayerItemController.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/PlayerControllers/PlayerPKController.cpp
)
target_include_directories(test_c26_player_controllers PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_compile_definitions(test_c26_player_controllers PRIVATE
    TEST_MODE_DISABLE_STDAFX
)
target_link_libraries(test_c26_player_controllers PRIVATE
    Client2026
    EterBase
    EterLib
    GameLib
    PackLib
)
add_test(NAME PlayerControllersTest COMMAND test_c26_player_controllers)


add_executable(test_c26_terrain_subsystem
    test_c26_terrain_subsystem.cpp
)
target_include_directories(test_c26_terrain_subsystem PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/GameLib
    ${CMAKE_SOURCE_DIR}/src/GameLib/Terrain
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_terrain_subsystem PRIVATE
    GameLib
    EterBase
    lzo2
    mio
    ws2_32
)
add_test(NAME TerrainSubsystemTest COMMAND test_c26_terrain_subsystem)

add_executable(test_c26_process_scanner
    test_c26_process_scanner.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/ProcessScanner.cpp
)
target_include_directories(test_c26_process_scanner PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_compile_definitions(test_c26_process_scanner PRIVATE
    TEST_MODE_DISABLE_STDAFX
)
target_link_libraries(test_c26_process_scanner PRIVATE
    EterBase
    ws2_32
)
add_test(NAME ProcessScannerTest COMMAND test_c26_process_scanner)

add_executable(test_c26_multi_server_protocol
    test_c26_multi_server_protocol.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetCombatRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetItemRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetActorRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetShopRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetExchangeRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetQuestRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetPartyRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetGuildRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/PhaseGamePacketDispatcher.cpp
)
target_include_directories(test_c26_multi_server_protocol PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_compile_definitions(test_c26_multi_server_protocol PRIVATE
    TEST_MODE_DISABLE_STDAFX
)
target_link_libraries(test_c26_multi_server_protocol PRIVATE
    Client2026
    EterLib
    sodium
    EterBase
    ws2_32
)
add_test(NAME MultiServerProtocolTest COMMAND test_c26_multi_server_protocol)

add_executable(test_c26_test_harness_engine
    test_c26_test_harness_engine.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/TestHarness/DeterministicTickController.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/TestHarness/CrashSentinel.cpp
)
target_include_directories(test_c26_test_harness_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_SOURCE_DIR}/src/UserInterface/TestHarness
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_test_harness_engine PRIVATE
    EterBase
    Dbghelp
    ws2_32
)
add_test(NAME TestHarnessEngineTest COMMAND test_c26_test_harness_engine)


add_executable(test_c26_gltf_math test_c26_gltf_math.cpp ../src/EterModelLib/SkeletalMath.cpp)
add_test(NAME test_c26_gltf_math COMMAND test_c26_gltf_math)

add_executable(test_c26_gltf_model test_c26_gltf_model.cpp)
target_include_directories(test_c26_gltf_model PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_gltf_model PRIVATE
    EterModelLib
)
add_test(NAME test_c26_gltf_model COMMAND test_c26_gltf_model)

add_executable(test_c27_gltf_pipeline test_c27_gltf_pipeline.cpp)
target_include_directories(test_c27_gltf_pipeline PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c27_gltf_pipeline PRIVATE
    EterModelLib
)
add_test(NAME test_c27_gltf_pipeline COMMAND test_c27_gltf_pipeline)

add_executable(test_c28_real_simulation test_c28_real_simulation.cpp)
target_include_directories(test_c28_real_simulation PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c28_real_simulation PRIVATE
    EterModelLib
)
add_test(NAME test_c28_real_simulation COMMAND test_c28_real_simulation)

add_executable(test_c29_gltf_d3d_bridge test_c29_gltf_d3d_bridge.cpp)
target_include_directories(test_c29_gltf_d3d_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c29_gltf_d3d_bridge PRIVATE
    EterGrnLib
    EterModelLib
    EterLib
    EterImageLib
    PackLib
    EterBase
    DirectX
    sodium
    freetype
    libzstd_static
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c29_gltf_d3d_bridge COMMAND test_c29_gltf_d3d_bridge)

add_executable(test_c30_d3d9_state_guards test_c30_d3d9_state_guards.cpp)
target_include_directories(test_c30_d3d9_state_guards PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c30_d3d9_state_guards PRIVATE
    EterLib
    EterBase
    DirectX
    sodium
    freetype
    libzstd_static
    lzo2
    mio
    ws2_32
)
add_test(NAME D3D9StateGuardsTest COMMAND test_c30_d3d9_state_guards)

add_executable(test_c31_beavium_server_profile
    test_c31_beavium_server_profile.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/PhaseGamePacketDispatcher.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetCombatRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetItemRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetActorRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetShopRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetExchangeRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetQuestRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetPartyRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetGuildRouter.cpp
)
target_include_directories(test_c31_beavium_server_profile PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_compile_definitions(test_c31_beavium_server_profile PRIVATE
    TEST_MODE_DISABLE_STDAFX
)
target_link_libraries(test_c31_beavium_server_profile PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME BeaviumServerProfileTest COMMAND test_c31_beavium_server_profile)

add_executable(test_c32_agent_mcp_bridge test_c32_agent_mcp_bridge.cpp)
target_include_directories(test_c32_agent_mcp_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c32_agent_mcp_bridge PRIVATE
    Client2026
    EterBase
)
add_test(NAME AgentMCPBridgeTest COMMAND test_c32_agent_mcp_bridge)

add_executable(test_c33_bot_ffi_bridge test_c33_bot_ffi_bridge.cpp)
target_include_directories(test_c33_bot_ffi_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c33_bot_ffi_bridge PRIVATE
    Client2026
    EterBase
)
add_test(NAME BotFFIBridgeTest COMMAND test_c33_bot_ffi_bridge)

add_executable(test_c26_phase_game_quest_bridge test_c26_phase_game_quest_bridge.cpp)
target_include_directories(test_c26_phase_game_quest_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_phase_game_quest_bridge PRIVATE
    Client2026
    EterBase
)
add_test(NAME PhaseGameQuestBridgeTest COMMAND test_c26_phase_game_quest_bridge)

add_executable(test_c26_ipc_transport_pipe test_c26_ipc_transport_pipe.cpp)
target_include_directories(test_c26_ipc_transport_pipe PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ipc_transport_pipe PRIVATE
    Client2026
    EterBase
)
add_test(NAME IPCTransportPipeTest COMMAND test_c26_ipc_transport_pipe)

add_executable(test_c31_debug_renderer test_c31_debug_renderer.cpp)
target_include_directories(test_c31_debug_renderer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c31_debug_renderer PRIVATE
    Client2026
    DirectX
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME DebugRendererTest COMMAND test_c31_debug_renderer)

add_executable(test_c34_universal_protocol_drivers
    test_c34_universal_protocol_drivers.cpp
    ${CMAKE_SOURCE_DIR}/src/Client/Network/Protocol/Drivers/MartySamaProtocolDriver.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetCombatRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetItemRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetActorRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetShopRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetExchangeRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetQuestRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetPartyRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/NetGuildRouter.cpp
    ${CMAKE_SOURCE_DIR}/src/UserInterface/Network/Routers/PhaseGamePacketDispatcher.cpp
)
target_include_directories(test_c34_universal_protocol_drivers PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_compile_definitions(test_c34_universal_protocol_drivers PRIVATE
    TEST_MODE_DISABLE_STDAFX
)
target_link_libraries(test_c34_universal_protocol_drivers PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME UniversalProtocolDriversTest COMMAND test_c34_universal_protocol_drivers)

add_executable(test_c35_crypto_providers
    test_c35_crypto_providers.cpp
)
target_include_directories(test_c35_crypto_providers PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
)
target_link_libraries(test_c35_crypto_providers PRIVATE
    Client2026
    EterLib
    EterBase
    sodium
    ws2_32
    mio
)
add_test(NAME CryptoProvidersTest COMMAND test_c35_crypto_providers)

add_executable(test_ai_telemetry
    test_ai_telemetry.cpp
)
target_include_directories(test_ai_telemetry PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_ai_telemetry PRIVATE
    Client2026
    EterBase
)
add_test(NAME AITelemetryTest COMMAND test_ai_telemetry)

add_executable(test_c36_vfs_mapmanager_fixes
    test_c36_vfs_mapmanager_fixes.cpp
)
target_include_directories(test_c36_vfs_mapmanager_fixes PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c36_vfs_mapmanager_fixes PRIVATE
    GameLib
    PackLib
    EterLib
    EterImageLib
    EterBase
    EterGrnLib
    EterModelLib
    EffectLib
    AudioLib
    PRTerrainLib
    SphereLib
    SpeedTreeLib
    SpeedTree
    DirectX
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
    Dbghelp
)
add_test(NAME VfsMapManagerFixesTest COMMAND test_c36_vfs_mapmanager_fixes)

# ============================================================================

# TESTY INTEGRACYJNE MCP ORAZ HEADLESS CLIENT (FILAR 5 / ARCHITEKTURA 2026)
# ============================================================================
find_package(Python3 COMPONENTS Interpreter REQUIRED)
if (Python3_Interpreter_FOUND)
    add_test(NAME McpHarnessSelfTest COMMAND Python3::Interpreter ${CMAKE_SOURCE_DIR}/tools/mcp/m2_agent_harness_mcp.py --self-test)
    add_test(NAME HeadlessClientLiveTest COMMAND Python3::Interpreter ${CMAKE_SOURCE_DIR}/tools/mcp/test_live_headless_client.py)
    set_tests_properties(HeadlessClientLiveTest PROPERTIES TIMEOUT 30)
endif()

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_action_rate_limiter test_c26_action_rate_limiter.cpp)
target_include_directories(test_c26_action_rate_limiter PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_action_rate_limiter PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_action_rate_limiter COMMAND test_c26_action_rate_limiter)

add_executable(test_c26_actor_additional_info_handler test_c26_actor_additional_info_handler.cpp)
target_include_directories(test_c26_actor_additional_info_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_additional_info_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_additional_info_handler COMMAND test_c26_actor_additional_info_handler)

add_executable(test_c26_actor_alpha_blender test_c26_actor_alpha_blender.cpp)
target_include_directories(test_c26_actor_alpha_blender PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_alpha_blender PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_alpha_blender COMMAND test_c26_actor_alpha_blender)

add_executable(test_c26_actor_bounding_frustum test_c26_actor_bounding_frustum.cpp)
target_include_directories(test_c26_actor_bounding_frustum PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_bounding_frustum PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_bounding_frustum COMMAND test_c26_actor_bounding_frustum)

add_executable(test_c26_actor_delete_handler test_c26_actor_delete_handler.cpp)
target_include_directories(test_c26_actor_delete_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_delete_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_delete_handler COMMAND test_c26_actor_delete_handler)

add_executable(test_c26_actor_instanced_batcher test_c26_actor_instanced_batcher.cpp)
target_include_directories(test_c26_actor_instanced_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_instanced_batcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_instanced_batcher COMMAND test_c26_actor_instanced_batcher)

add_executable(test_c26_actor_lod_selector test_c26_actor_lod_selector.cpp)
target_include_directories(test_c26_actor_lod_selector PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_lod_selector PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_lod_selector COMMAND test_c26_actor_lod_selector)

add_executable(test_c26_actor_motion_machine test_c26_actor_motion_machine.cpp)
target_include_directories(test_c26_actor_motion_machine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_motion_machine PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_motion_machine COMMAND test_c26_actor_motion_machine)

add_executable(test_c26_actor_move_handler test_c26_actor_move_handler.cpp)
target_include_directories(test_c26_actor_move_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_move_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_move_handler COMMAND test_c26_actor_move_handler)

add_executable(test_c26_actor_packet_codec test_c26_actor_packet_codec.cpp)
target_include_directories(test_c26_actor_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_packet_codec COMMAND test_c26_actor_packet_codec)

add_executable(test_c26_actor_spawn_handler test_c26_actor_spawn_handler.cpp)
target_include_directories(test_c26_actor_spawn_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_spawn_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_spawn_handler COMMAND test_c26_actor_spawn_handler)

add_executable(test_c26_actor_update_handler test_c26_actor_update_handler.cpp)
target_include_directories(test_c26_actor_update_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_update_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_update_handler COMMAND test_c26_actor_update_handler)

add_executable(test_c26_additive_pass test_c26_additive_pass.cpp)
target_include_directories(test_c26_additive_pass PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_additive_pass PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_additive_pass COMMAND test_c26_additive_pass)

add_executable(test_c26_aes_crypto_provider test_c26_aes_crypto_provider.cpp)
target_include_directories(test_c26_aes_crypto_provider PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_aes_crypto_provider PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_aes_crypto_provider COMMAND test_c26_aes_crypto_provider)

add_executable(test_c26_alpha_test_scope test_c26_alpha_test_scope.cpp)
target_include_directories(test_c26_alpha_test_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_alpha_test_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_alpha_test_scope COMMAND test_c26_alpha_test_scope)

add_executable(test_c26_alphatest_pass test_c26_alphatest_pass.cpp)
target_include_directories(test_c26_alphatest_pass PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_alphatest_pass PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_alphatest_pass COMMAND test_c26_alphatest_pass)

add_executable(test_c26_blend_pass test_c26_blend_pass.cpp)
target_include_directories(test_c26_blend_pass PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_blend_pass PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_blend_pass COMMAND test_c26_blend_pass)

add_executable(test_c26_blend_scope test_c26_blend_scope.cpp)
target_include_directories(test_c26_blend_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_blend_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_blend_scope COMMAND test_c26_blend_scope)

add_executable(test_c26_buffer_cache test_c26_buffer_cache.cpp)
target_include_directories(test_c26_buffer_cache PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_buffer_cache PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_buffer_cache COMMAND test_c26_buffer_cache)

add_executable(test_c26_camera_frustum test_c26_camera_frustum.cpp)
target_include_directories(test_c26_camera_frustum PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_camera_frustum PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_camera_frustum COMMAND test_c26_camera_frustum)

add_executable(test_c26_chat_command_handler test_c26_chat_command_handler.cpp)
target_include_directories(test_c26_chat_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_chat_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_chat_command_handler COMMAND test_c26_chat_command_handler)

add_executable(test_c26_chat_filter_engine test_c26_chat_filter_engine.cpp)
target_include_directories(test_c26_chat_filter_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_chat_filter_engine PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_chat_filter_engine COMMAND test_c26_chat_filter_engine)

add_executable(test_c26_chat_handler test_c26_chat_handler.cpp)
target_include_directories(test_c26_chat_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_chat_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_chat_handler COMMAND test_c26_chat_handler)

add_executable(test_c26_cmd_indexed test_c26_cmd_indexed.cpp)
target_include_directories(test_c26_cmd_indexed PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_cmd_indexed PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_cmd_indexed COMMAND test_c26_cmd_indexed)

add_executable(test_c26_cmd_primitive test_c26_cmd_primitive.cpp)
target_include_directories(test_c26_cmd_primitive PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_cmd_primitive PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_cmd_primitive COMMAND test_c26_cmd_primitive)

add_executable(test_c26_cmd_user_prim test_c26_cmd_user_prim.cpp)
target_include_directories(test_c26_cmd_user_prim PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_cmd_user_prim PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_cmd_user_prim COMMAND test_c26_cmd_user_prim)

add_executable(test_c26_collision_detector test_c26_collision_detector.cpp)
target_include_directories(test_c26_collision_detector PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_collision_detector PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_collision_detector COMMAND test_c26_collision_detector)

add_executable(test_c26_color_write_scope test_c26_color_write_scope.cpp)
target_include_directories(test_c26_color_write_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_color_write_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_color_write_scope COMMAND test_c26_color_write_scope)

add_executable(test_c26_combat_command_encoder test_c26_combat_command_encoder.cpp)
target_include_directories(test_c26_combat_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_command_encoder COMMAND test_c26_combat_command_encoder)

add_executable(test_c26_combat_command_handler test_c26_combat_command_handler.cpp)
target_include_directories(test_c26_combat_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_command_handler COMMAND test_c26_combat_command_handler)

add_executable(test_c26_combat_damage_predictor test_c26_combat_damage_predictor.cpp)
target_include_directories(test_c26_combat_damage_predictor PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_damage_predictor PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_damage_predictor COMMAND test_c26_combat_damage_predictor)

add_executable(test_c26_combat_domain test_c26_combat_domain.cpp)
target_include_directories(test_c26_combat_domain PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_domain PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_domain COMMAND test_c26_combat_domain)

add_executable(test_c26_combat_packet_codec test_c26_combat_packet_codec.cpp)
target_include_directories(test_c26_combat_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_packet_codec COMMAND test_c26_combat_packet_codec)

add_executable(test_c26_combat_range_calculator test_c26_combat_range_calculator.cpp)
target_include_directories(test_c26_combat_range_calculator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_range_calculator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_range_calculator COMMAND test_c26_combat_range_calculator)

add_executable(test_c26_combat_sequence_guard test_c26_combat_sequence_guard.cpp)
target_include_directories(test_c26_combat_sequence_guard PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_sequence_guard PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_sequence_guard COMMAND test_c26_combat_sequence_guard)

add_executable(test_c26_combat_simulation_scenario test_c26_combat_simulation_scenario.cpp)
target_include_directories(test_c26_combat_simulation_scenario PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_simulation_scenario PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_simulation_scenario COMMAND test_c26_combat_simulation_scenario)

add_executable(test_c26_combat_target_validator test_c26_combat_target_validator.cpp)
target_include_directories(test_c26_combat_target_validator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_combat_target_validator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_combat_target_validator COMMAND test_c26_combat_target_validator)

add_executable(test_c26_crc_piece_calculator test_c26_crc_piece_calculator.cpp)
target_include_directories(test_c26_crc_piece_calculator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_crc_piece_calculator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_crc_piece_calculator COMMAND test_c26_crc_piece_calculator)

add_executable(test_c26_damage_info_handler test_c26_damage_info_handler.cpp)
target_include_directories(test_c26_damage_info_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_damage_info_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_damage_info_handler COMMAND test_c26_damage_info_handler)

add_executable(test_c26_dead_handler test_c26_dead_handler.cpp)
target_include_directories(test_c26_dead_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_dead_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_dead_handler COMMAND test_c26_dead_handler)

add_executable(test_c26_deduplicator test_c26_deduplicator.cpp)
target_include_directories(test_c26_deduplicator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_deduplicator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_deduplicator COMMAND test_c26_deduplicator)

add_executable(test_c26_depth_prepass test_c26_depth_prepass.cpp)
target_include_directories(test_c26_depth_prepass PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_depth_prepass PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_depth_prepass COMMAND test_c26_depth_prepass)

add_executable(test_c26_depth_scope test_c26_depth_scope.cpp)
target_include_directories(test_c26_depth_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_depth_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_depth_scope COMMAND test_c26_depth_scope)

add_executable(test_c26_dungeon_command_encoder test_c26_dungeon_command_encoder.cpp)
target_include_directories(test_c26_dungeon_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_dungeon_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_dungeon_command_encoder COMMAND test_c26_dungeon_command_encoder)

add_executable(test_c26_dynamic_packet_framer test_c26_dynamic_packet_framer.cpp)
target_include_directories(test_c26_dynamic_packet_framer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_dynamic_packet_framer PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_dynamic_packet_framer COMMAND test_c26_dynamic_packet_framer)

add_executable(test_c26_dynamic_packet_framer_v2 test_c26_dynamic_packet_framer_v2.cpp)
target_include_directories(test_c26_dynamic_packet_framer_v2 PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_dynamic_packet_framer_v2 PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_dynamic_packet_framer_v2 COMMAND test_c26_dynamic_packet_framer_v2)

add_executable(test_c26_dynamic_ring_ibo test_c26_dynamic_ring_ibo.cpp)
target_include_directories(test_c26_dynamic_ring_ibo PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_dynamic_ring_ibo PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_dynamic_ring_ibo COMMAND test_c26_dynamic_ring_ibo)

add_executable(test_c26_dynamic_ring_vbo test_c26_dynamic_ring_vbo.cpp)
target_include_directories(test_c26_dynamic_ring_vbo PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_dynamic_ring_vbo PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_dynamic_ring_vbo COMMAND test_c26_dynamic_ring_vbo)

add_executable(test_c26_e2e_classic_ymir_simulation test_c26_e2e_classic_ymir_simulation.cpp)
target_include_directories(test_c26_e2e_classic_ymir_simulation PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_e2e_classic_ymir_simulation PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_e2e_classic_ymir_simulation COMMAND test_c26_e2e_classic_ymir_simulation)

add_executable(test_c26_ecs_components test_c26_ecs_components.cpp)
target_include_directories(test_c26_ecs_components PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ecs_components PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ecs_components COMMAND test_c26_ecs_components)

add_executable(test_c26_exchange_command_encoder test_c26_exchange_command_encoder.cpp)
target_include_directories(test_c26_exchange_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_exchange_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_exchange_command_encoder COMMAND test_c26_exchange_command_encoder)

add_executable(test_c26_exchange_command_handler test_c26_exchange_command_handler.cpp)
target_include_directories(test_c26_exchange_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_exchange_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_exchange_command_handler COMMAND test_c26_exchange_command_handler)

add_executable(test_c26_exchange_handler test_c26_exchange_handler.cpp)
target_include_directories(test_c26_exchange_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_exchange_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_exchange_handler COMMAND test_c26_exchange_handler)

add_executable(test_c26_exchange_packet_codec test_c26_exchange_packet_codec.cpp)
target_include_directories(test_c26_exchange_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_exchange_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_exchange_packet_codec COMMAND test_c26_exchange_packet_codec)

add_executable(test_c26_exchange_state_guard test_c26_exchange_state_guard.cpp)
target_include_directories(test_c26_exchange_state_guard PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_exchange_state_guard PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_exchange_state_guard COMMAND test_c26_exchange_state_guard)

add_executable(test_c26_fixed_state_stack test_c26_fixed_state_stack.cpp)
target_include_directories(test_c26_fixed_state_stack PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_fixed_state_stack PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_fixed_state_stack COMMAND test_c26_fixed_state_stack)

add_executable(test_c26_fog_scope test_c26_fog_scope.cpp)
target_include_directories(test_c26_fog_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_fog_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_fog_scope COMMAND test_c26_fog_scope)

add_executable(test_c26_font_glyph_batcher test_c26_font_glyph_batcher.cpp)
target_include_directories(test_c26_font_glyph_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_font_glyph_batcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_font_glyph_batcher COMMAND test_c26_font_glyph_batcher)

add_executable(test_c26_frame_alloc test_c26_frame_alloc.cpp)
target_include_directories(test_c26_frame_alloc PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_frame_alloc PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_frame_alloc COMMAND test_c26_frame_alloc)

add_executable(test_c26_frame_coordinator test_c26_frame_coordinator.cpp)
target_include_directories(test_c26_frame_coordinator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_frame_coordinator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_frame_coordinator COMMAND test_c26_frame_coordinator)

add_executable(test_c26_frame_stats test_c26_frame_stats.cpp)
target_include_directories(test_c26_frame_stats PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_frame_stats PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_frame_stats COMMAND test_c26_frame_stats)

add_executable(test_c26_frame_timer test_c26_frame_timer.cpp)
target_include_directories(test_c26_frame_timer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_frame_timer PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_frame_timer COMMAND test_c26_frame_timer)

add_executable(test_c26_game_session test_c26_game_session.cpp)
target_include_directories(test_c26_game_session PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_game_session PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_game_session COMMAND test_c26_game_session)

add_executable(test_c26_gaussian_packet_jitter test_c26_gaussian_packet_jitter.cpp)
target_include_directories(test_c26_gaussian_packet_jitter PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_gaussian_packet_jitter PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_gaussian_packet_jitter COMMAND test_c26_gaussian_packet_jitter)

add_executable(test_c26_granny_bone_cache test_c26_granny_bone_cache.cpp)
target_include_directories(test_c26_granny_bone_cache PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_granny_bone_cache PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_granny_bone_cache COMMAND test_c26_granny_bone_cache)

add_executable(test_c26_granny_render_bridge test_c26_granny_render_bridge.cpp)
target_include_directories(test_c26_granny_render_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_granny_render_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_granny_render_bridge COMMAND test_c26_granny_render_bridge)

add_executable(test_c26_guild_handler test_c26_guild_handler.cpp)
target_include_directories(test_c26_guild_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_guild_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_guild_handler COMMAND test_c26_guild_handler)

add_executable(test_c26_guild_packet_codec test_c26_guild_packet_codec.cpp)
target_include_directories(test_c26_guild_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_guild_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_guild_packet_codec COMMAND test_c26_guild_packet_codec)

add_executable(test_c26_handshake_flow_adapter test_c26_handshake_flow_adapter.cpp)
target_include_directories(test_c26_handshake_flow_adapter PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_handshake_flow_adapter PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_handshake_flow_adapter COMMAND test_c26_handshake_flow_adapter)

add_executable(test_c26_handshake_fsm test_c26_handshake_fsm.cpp)
target_include_directories(test_c26_handshake_fsm PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_handshake_fsm PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_handshake_fsm COMMAND test_c26_handshake_fsm)

add_executable(test_c26_ib_scope test_c26_ib_scope.cpp)
target_include_directories(test_c26_ib_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ib_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ib_scope COMMAND test_c26_ib_scope)

add_executable(test_c26_inst_batcher test_c26_inst_batcher.cpp)
target_include_directories(test_c26_inst_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_inst_batcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_inst_batcher COMMAND test_c26_inst_batcher)

add_executable(test_c26_inst_collector test_c26_inst_collector.cpp)
target_include_directories(test_c26_inst_collector PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_inst_collector PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_inst_collector COMMAND test_c26_inst_collector)

add_executable(test_c26_instance_stream_buf test_c26_instance_stream_buf.cpp)
target_include_directories(test_c26_instance_stream_buf PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_instance_stream_buf PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_instance_stream_buf COMMAND test_c26_instance_stream_buf)

add_executable(test_c26_instanced_shadow_batcher test_c26_instanced_shadow_batcher.cpp)
target_include_directories(test_c26_instanced_shadow_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_instanced_shadow_batcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_instanced_shadow_batcher COMMAND test_c26_instanced_shadow_batcher)

add_executable(test_c26_inventory_command_encoder test_c26_inventory_command_encoder.cpp)
target_include_directories(test_c26_inventory_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_inventory_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_inventory_command_encoder COMMAND test_c26_inventory_command_encoder)

add_executable(test_c26_inventory_command_handler test_c26_inventory_command_handler.cpp)
target_include_directories(test_c26_inventory_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_inventory_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_inventory_command_handler COMMAND test_c26_inventory_command_handler)

add_executable(test_c26_inventory_drop_physics_bridge test_c26_inventory_drop_physics_bridge.cpp)
target_include_directories(test_c26_inventory_drop_physics_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_inventory_drop_physics_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_inventory_drop_physics_bridge COMMAND test_c26_inventory_drop_physics_bridge)

add_executable(test_c26_inventory_grid_manager test_c26_inventory_grid_manager.cpp)
target_include_directories(test_c26_inventory_grid_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_inventory_grid_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_inventory_grid_manager COMMAND test_c26_inventory_grid_manager)

add_executable(test_c26_inventory_item_validator test_c26_inventory_item_validator.cpp)
target_include_directories(test_c26_inventory_item_validator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_inventory_item_validator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_inventory_item_validator COMMAND test_c26_inventory_item_validator)

add_executable(test_c26_inventory_pickup_limiter test_c26_inventory_pickup_limiter.cpp)
target_include_directories(test_c26_inventory_pickup_limiter PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_inventory_pickup_limiter PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_inventory_pickup_limiter COMMAND test_c26_inventory_pickup_limiter)

add_executable(test_c26_ipc_command_dispatcher test_c26_ipc_command_dispatcher.cpp)
target_include_directories(test_c26_ipc_command_dispatcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ipc_command_dispatcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ipc_command_dispatcher COMMAND test_c26_ipc_command_dispatcher)

add_executable(test_c26_ipc_protocol_codec test_c26_ipc_protocol_codec.cpp)
target_include_directories(test_c26_ipc_protocol_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ipc_protocol_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ipc_protocol_codec COMMAND test_c26_ipc_protocol_codec)

add_executable(test_c26_ipc_query_handler test_c26_ipc_query_handler.cpp)
target_include_directories(test_c26_ipc_query_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ipc_query_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ipc_query_handler COMMAND test_c26_ipc_query_handler)

add_executable(test_c26_item_del_handler test_c26_item_del_handler.cpp)
target_include_directories(test_c26_item_del_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_item_del_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_item_del_handler COMMAND test_c26_item_del_handler)

add_executable(test_c26_item_ground_add_handler test_c26_item_ground_add_handler.cpp)
target_include_directories(test_c26_item_ground_add_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_item_ground_add_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_item_ground_add_handler COMMAND test_c26_item_ground_add_handler)

add_executable(test_c26_item_ground_del_handler test_c26_item_ground_del_handler.cpp)
target_include_directories(test_c26_item_ground_del_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_item_ground_del_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_item_ground_del_handler COMMAND test_c26_item_ground_del_handler)

add_executable(test_c26_item_manager_modern_loader test_c26_item_manager_modern_loader.cpp)
target_include_directories(test_c26_item_manager_modern_loader PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_item_manager_modern_loader PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_item_manager_modern_loader COMMAND test_c26_item_manager_modern_loader)

add_executable(test_c26_item_packet_codec test_c26_item_packet_codec.cpp)
target_include_directories(test_c26_item_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_item_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_item_packet_codec COMMAND test_c26_item_packet_codec)

add_executable(test_c26_item_set_handler test_c26_item_set_handler.cpp)
target_include_directories(test_c26_item_set_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_item_set_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_item_set_handler COMMAND test_c26_item_set_handler)

add_executable(test_c26_key_builder test_c26_key_builder.cpp)
target_include_directories(test_c26_key_builder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_key_builder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_key_builder COMMAND test_c26_key_builder)

add_executable(test_c26_lights_scope test_c26_lights_scope.cpp)
target_include_directories(test_c26_lights_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_lights_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_lights_scope COMMAND test_c26_lights_scope)

add_executable(test_c26_loading_packet_handler test_c26_loading_packet_handler.cpp)
target_include_directories(test_c26_loading_packet_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_loading_packet_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_loading_packet_handler COMMAND test_c26_loading_packet_handler)

add_executable(test_c26_login_packet_handler test_c26_login_packet_handler.cpp)
target_include_directories(test_c26_login_packet_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_login_packet_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_login_packet_handler COMMAND test_c26_login_packet_handler)

add_executable(test_c26_map_outdoor_facade test_c26_map_outdoor_facade.cpp)
target_include_directories(test_c26_map_outdoor_facade PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_map_outdoor_facade PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_map_outdoor_facade COMMAND test_c26_map_outdoor_facade)

add_executable(test_c26_mat_cache test_c26_mat_cache.cpp)
target_include_directories(test_c26_mat_cache PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_mat_cache PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_mat_cache COMMAND test_c26_mat_cache)

add_executable(test_c26_material_scope test_c26_material_scope.cpp)
target_include_directories(test_c26_material_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_material_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_material_scope COMMAND test_c26_material_scope)

add_executable(test_c26_math_simd test_c26_math_simd.cpp)
target_include_directories(test_c26_math_simd PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_math_simd PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_math_simd COMMAND test_c26_math_simd)

add_executable(test_c26_mcp_bridge_server test_c26_mcp_bridge_server.cpp)
target_include_directories(test_c26_mcp_bridge_server PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_mcp_bridge_server PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_mcp_bridge_server COMMAND test_c26_mcp_bridge_server)

add_executable(test_c26_minimap_radar test_c26_minimap_radar.cpp)
target_include_directories(test_c26_minimap_radar PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_minimap_radar PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_minimap_radar COMMAND test_c26_minimap_radar)

add_executable(test_c26_mock_network_port_advanced test_c26_mock_network_port_advanced.cpp)
target_include_directories(test_c26_mock_network_port_advanced PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_mock_network_port_advanced PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_mock_network_port_advanced COMMAND test_c26_mock_network_port_advanced)

add_executable(test_c26_model_shader_pipeline test_c26_model_shader_pipeline.cpp)
target_include_directories(test_c26_model_shader_pipeline PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_model_shader_pipeline PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_model_shader_pipeline COMMAND test_c26_model_shader_pipeline)

add_executable(test_c26_modern_bridge test_c26_modern_bridge.cpp)
target_include_directories(test_c26_modern_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_modern_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_modern_bridge COMMAND test_c26_modern_bridge)

add_executable(test_c26_modern_mutex test_c26_modern_mutex.cpp)
target_include_directories(test_c26_modern_mutex PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_modern_mutex PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_modern_mutex COMMAND test_c26_modern_mutex)

add_executable(test_c26_mount_domain test_c26_mount_domain.cpp)
target_include_directories(test_c26_mount_domain PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_mount_domain PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_mount_domain COMMAND test_c26_mount_domain)

add_executable(test_c26_movement_command_encoder test_c26_movement_command_encoder.cpp)
target_include_directories(test_c26_movement_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_movement_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_movement_command_encoder COMMAND test_c26_movement_command_encoder)

add_executable(test_c26_movement_command_handler test_c26_movement_command_handler.cpp)
target_include_directories(test_c26_movement_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_movement_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_movement_command_handler COMMAND test_c26_movement_command_handler)

add_executable(test_c26_movement_interpolation_engine test_c26_movement_interpolation_engine.cpp)
target_include_directories(test_c26_movement_interpolation_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_movement_interpolation_engine PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_movement_interpolation_engine COMMAND test_c26_movement_interpolation_engine)

add_executable(test_c26_movement_rate_limiter test_c26_movement_rate_limiter.cpp)
target_include_directories(test_c26_movement_rate_limiter PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_movement_rate_limiter PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_movement_rate_limiter COMMAND test_c26_movement_rate_limiter)

add_executable(test_c26_movement_step_validator test_c26_movement_step_validator.cpp)
target_include_directories(test_c26_movement_step_validator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_movement_step_validator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_movement_step_validator COMMAND test_c26_movement_step_validator)

add_executable(test_c26_movement_terrain_collider test_c26_movement_terrain_collider.cpp)
target_include_directories(test_c26_movement_terrain_collider PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_movement_terrain_collider PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_movement_terrain_collider COMMAND test_c26_movement_terrain_collider)

add_executable(test_c26_multi_crypto_manager test_c26_multi_crypto_manager.cpp)
target_include_directories(test_c26_multi_crypto_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_multi_crypto_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_multi_crypto_manager COMMAND test_c26_multi_crypto_manager)

add_executable(test_c26_opaque_pass test_c26_opaque_pass.cpp)
target_include_directories(test_c26_opaque_pass PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_opaque_pass PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_opaque_pass COMMAND test_c26_opaque_pass)

add_executable(test_c26_packet_ring_buffer test_c26_packet_ring_buffer.cpp)
target_include_directories(test_c26_packet_ring_buffer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_packet_ring_buffer PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_packet_ring_buffer COMMAND test_c26_packet_ring_buffer)

add_executable(test_c26_packet_schema_engine test_c26_packet_schema_engine.cpp)
target_include_directories(test_c26_packet_schema_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_packet_schema_engine PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_packet_schema_engine COMMAND test_c26_packet_schema_engine)

add_executable(test_c26_party_command_handler test_c26_party_command_handler.cpp)
target_include_directories(test_c26_party_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_party_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_party_command_handler COMMAND test_c26_party_command_handler)

add_executable(test_c26_party_handler test_c26_party_handler.cpp)
target_include_directories(test_c26_party_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_party_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_party_handler COMMAND test_c26_party_handler)

add_executable(test_c26_party_packet_codec test_c26_party_packet_codec.cpp)
target_include_directories(test_c26_party_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_party_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_party_packet_codec COMMAND test_c26_party_packet_codec)

add_executable(test_c26_pending_spawn_registry test_c26_pending_spawn_registry.cpp)
target_include_directories(test_c26_pending_spawn_registry PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_pending_spawn_registry PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_pending_spawn_registry COMMAND test_c26_pending_spawn_registry)

add_executable(test_c26_phase_game_guild_bridge test_c26_phase_game_guild_bridge.cpp)
target_include_directories(test_c26_phase_game_guild_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_guild_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_guild_bridge COMMAND test_c26_phase_game_guild_bridge)

add_executable(test_c26_phase_game_skills_bridge test_c26_phase_game_skills_bridge.cpp)
target_include_directories(test_c26_phase_game_skills_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_skills_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_skills_bridge COMMAND test_c26_phase_game_skills_bridge)

add_executable(test_c26_phase_game_target_bridge test_c26_phase_game_target_bridge.cpp)
target_include_directories(test_c26_phase_game_target_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_target_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_target_bridge COMMAND test_c26_phase_game_target_bridge)

add_executable(test_c26_phase_state_machine test_c26_phase_state_machine.cpp)
target_include_directories(test_c26_phase_state_machine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_state_machine PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_state_machine COMMAND test_c26_phase_state_machine)

add_executable(test_c26_ping_pong_engine test_c26_ping_pong_engine.cpp)
target_include_directories(test_c26_ping_pong_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ping_pong_engine PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ping_pong_engine COMMAND test_c26_ping_pong_engine)

add_executable(test_c26_pipeline_exec test_c26_pipeline_exec.cpp)
target_include_directories(test_c26_pipeline_exec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_pipeline_exec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_pipeline_exec COMMAND test_c26_pipeline_exec)

add_executable(test_c26_player_points_handler test_c26_player_points_handler.cpp)
target_include_directories(test_c26_player_points_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_points_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_points_handler COMMAND test_c26_player_points_handler)

add_executable(test_c26_player_stats_domain test_c26_player_stats_domain.cpp)
target_include_directories(test_c26_player_stats_domain PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_stats_domain PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_stats_domain COMMAND test_c26_player_stats_domain)

add_executable(test_c26_pluggable_crypto_provider test_c26_pluggable_crypto_provider.cpp)
target_include_directories(test_c26_pluggable_crypto_provider PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_pluggable_crypto_provider PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_pluggable_crypto_provider COMMAND test_c26_pluggable_crypto_provider)

add_executable(test_c26_post_process_pass test_c26_post_process_pass.cpp)
target_include_directories(test_c26_post_process_pass PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_post_process_pass PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_post_process_pass COMMAND test_c26_post_process_pass)

add_executable(test_c26_proto_binary_serializer_zstd test_c26_proto_binary_serializer_zstd.cpp)
target_include_directories(test_c26_proto_binary_serializer_zstd PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_proto_binary_serializer_zstd PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_proto_binary_serializer_zstd COMMAND test_c26_proto_binary_serializer_zstd)

add_executable(test_c26_proto_csv_parser_modern test_c26_proto_csv_parser_modern.cpp)
target_include_directories(test_c26_proto_csv_parser_modern PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_proto_csv_parser_modern PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_proto_csv_parser_modern COMMAND test_c26_proto_csv_parser_modern)

add_executable(test_c26_proto_json_parser_modern test_c26_proto_json_parser_modern.cpp)
target_include_directories(test_c26_proto_json_parser_modern PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_proto_json_parser_modern PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_proto_json_parser_modern COMMAND test_c26_proto_json_parser_modern)

add_executable(test_c26_proto_legacy_adapter test_c26_proto_legacy_adapter.cpp)
target_include_directories(test_c26_proto_legacy_adapter PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_proto_legacy_adapter PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_proto_legacy_adapter COMMAND test_c26_proto_legacy_adapter)

add_executable(test_c26_proto_schema_definition test_c26_proto_schema_definition.cpp)
target_include_directories(test_c26_proto_schema_definition PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_proto_schema_definition PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_proto_schema_definition COMMAND test_c26_proto_schema_definition)

add_executable(test_c26_protocol_adapter test_c26_protocol_adapter.cpp)
target_include_directories(test_c26_protocol_adapter PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_protocol_adapter PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_protocol_adapter COMMAND test_c26_protocol_adapter)

add_executable(test_c26_ps_scope test_c26_ps_scope.cpp)
target_include_directories(test_c26_ps_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ps_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ps_scope COMMAND test_c26_ps_scope)

add_executable(test_c26_py_graphic_bridge test_c26_py_graphic_bridge.cpp)
target_include_directories(test_c26_py_graphic_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_py_graphic_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_py_graphic_bridge COMMAND test_c26_py_graphic_bridge)

add_executable(test_c26_quest_command_encoder test_c26_quest_command_encoder.cpp)
target_include_directories(test_c26_quest_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_quest_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_quest_command_encoder COMMAND test_c26_quest_command_encoder)

add_executable(test_c26_quest_command_handler test_c26_quest_command_handler.cpp)
target_include_directories(test_c26_quest_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_quest_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_quest_command_handler COMMAND test_c26_quest_command_handler)

add_executable(test_c26_quest_handler test_c26_quest_handler.cpp)
target_include_directories(test_c26_quest_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_quest_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_quest_handler COMMAND test_c26_quest_handler)

add_executable(test_c26_quest_packet_codec test_c26_quest_packet_codec.cpp)
target_include_directories(test_c26_quest_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_quest_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_quest_packet_codec COMMAND test_c26_quest_packet_codec)

add_executable(test_c26_raster_scope test_c26_raster_scope.cpp)
target_include_directories(test_c26_raster_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_raster_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_raster_scope COMMAND test_c26_raster_scope)

add_executable(test_c26_refine_command_encoder test_c26_refine_command_encoder.cpp)
target_include_directories(test_c26_refine_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_refine_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_refine_command_encoder COMMAND test_c26_refine_command_encoder)

add_executable(test_c26_refine_packet_codec test_c26_refine_packet_codec.cpp)
target_include_directories(test_c26_refine_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_refine_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_refine_packet_codec COMMAND test_c26_refine_packet_codec)

add_executable(test_c26_render_context test_c26_render_context.cpp)
target_include_directories(test_c26_render_context PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_render_context PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_render_context COMMAND test_c26_render_context)

add_executable(test_c26_render_queue test_c26_render_queue.cpp)
target_include_directories(test_c26_render_queue PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_render_queue PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_render_queue COMMAND test_c26_render_queue)

add_executable(test_c26_render_state_guard test_c26_render_state_guard.cpp)
target_include_directories(test_c26_render_state_guard PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_render_state_guard PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_render_state_guard COMMAND test_c26_render_state_guard)

add_executable(test_c26_result_system test_c26_result_system.cpp)
target_include_directories(test_c26_result_system PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_result_system PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_result_system COMMAND test_c26_result_system)

add_executable(test_c26_rhi_interface test_c26_rhi_interface.cpp)
target_include_directories(test_c26_rhi_interface PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_rhi_interface PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_rhi_interface COMMAND test_c26_rhi_interface)

add_executable(test_c26_safebox_command_handler test_c26_safebox_command_handler.cpp)
target_include_directories(test_c26_safebox_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_safebox_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_safebox_command_handler COMMAND test_c26_safebox_command_handler)

add_executable(test_c26_safebox_handler test_c26_safebox_handler.cpp)
target_include_directories(test_c26_safebox_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_safebox_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_safebox_handler COMMAND test_c26_safebox_handler)

add_executable(test_c26_sampler_addr_scope test_c26_sampler_addr_scope.cpp)
target_include_directories(test_c26_sampler_addr_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_sampler_addr_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_sampler_addr_scope COMMAND test_c26_sampler_addr_scope)

add_executable(test_c26_sampler_filter_scope test_c26_sampler_filter_scope.cpp)
target_include_directories(test_c26_sampler_filter_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_sampler_filter_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_sampler_filter_scope COMMAND test_c26_sampler_filter_scope)

add_executable(test_c26_select_packet_handler test_c26_select_packet_handler.cpp)
target_include_directories(test_c26_select_packet_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_select_packet_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_select_packet_handler COMMAND test_c26_select_packet_handler)

add_executable(test_c26_server_profile test_c26_server_profile.cpp)
target_include_directories(test_c26_server_profile PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_server_profile PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_server_profile COMMAND test_c26_server_profile)

add_executable(test_c26_server_profile_manager test_c26_server_profile_manager.cpp)
target_include_directories(test_c26_server_profile_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_server_profile_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_server_profile_manager COMMAND test_c26_server_profile_manager)

add_executable(test_c26_server_profile_registry test_c26_server_profile_registry.cpp)
target_include_directories(test_c26_server_profile_registry PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_server_profile_registry PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_server_profile_registry COMMAND test_c26_server_profile_registry)

add_executable(test_c26_session_simulation_harness test_c26_session_simulation_harness.cpp)
target_include_directories(test_c26_session_simulation_harness PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_session_simulation_harness PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_session_simulation_harness COMMAND test_c26_session_simulation_harness)

add_executable(test_c26_shader_cache test_c26_shader_cache.cpp)
target_include_directories(test_c26_shader_cache PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_shader_cache PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_shader_cache COMMAND test_c26_shader_cache)

add_executable(test_c26_shader_const_scope test_c26_shader_const_scope.cpp)
target_include_directories(test_c26_shader_const_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_shader_const_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_shader_const_scope COMMAND test_c26_shader_const_scope)

add_executable(test_c26_shader_constant_buffer test_c26_shader_constant_buffer.cpp)
target_include_directories(test_c26_shader_constant_buffer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_shader_constant_buffer PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_shader_constant_buffer COMMAND test_c26_shader_constant_buffer)

add_executable(test_c26_shop_command_encoder test_c26_shop_command_encoder.cpp)
target_include_directories(test_c26_shop_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_shop_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_shop_command_encoder COMMAND test_c26_shop_command_encoder)

add_executable(test_c26_shop_command_handler test_c26_shop_command_handler.cpp)
target_include_directories(test_c26_shop_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_shop_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_shop_command_handler COMMAND test_c26_shop_command_handler)

add_executable(test_c26_shop_handler test_c26_shop_handler.cpp)
target_include_directories(test_c26_shop_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_shop_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_shop_handler COMMAND test_c26_shop_handler)

add_executable(test_c26_shop_packet_codec test_c26_shop_packet_codec.cpp)
target_include_directories(test_c26_shop_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_shop_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_shop_packet_codec COMMAND test_c26_shop_packet_codec)

add_executable(test_c26_shop_price_validator test_c26_shop_price_validator.cpp)
target_include_directories(test_c26_shop_price_validator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_shop_price_validator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_shop_price_validator COMMAND test_c26_shop_price_validator)

add_executable(test_c26_skill_cooltime_handler test_c26_skill_cooltime_handler.cpp)
target_include_directories(test_c26_skill_cooltime_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_cooltime_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_cooltime_handler COMMAND test_c26_skill_cooltime_handler)

add_executable(test_c26_skill_level_handler test_c26_skill_level_handler.cpp)
target_include_directories(test_c26_skill_level_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_level_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_level_handler COMMAND test_c26_skill_level_handler)

add_executable(test_c26_skill_packet_codec test_c26_skill_packet_codec.cpp)
target_include_directories(test_c26_skill_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_packet_codec COMMAND test_c26_skill_packet_codec)

add_executable(test_c26_social_command_encoder test_c26_social_command_encoder.cpp)
target_include_directories(test_c26_social_command_encoder PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_social_command_encoder PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_social_command_encoder COMMAND test_c26_social_command_encoder)

add_executable(test_c26_social_domain test_c26_social_domain.cpp)
target_include_directories(test_c26_social_domain PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_social_domain PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_social_domain COMMAND test_c26_social_domain)

add_executable(test_c26_sort_key test_c26_sort_key.cpp)
target_include_directories(test_c26_sort_key PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_sort_key PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_sort_key COMMAND test_c26_sort_key)

add_executable(test_c26_speedtree_batcher test_c26_speedtree_batcher.cpp)
target_include_directories(test_c26_speedtree_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_speedtree_batcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_speedtree_batcher COMMAND test_c26_speedtree_batcher)

add_executable(test_c26_speedtree_branch test_c26_speedtree_branch.cpp)
target_include_directories(test_c26_speedtree_branch PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_speedtree_branch PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_speedtree_branch COMMAND test_c26_speedtree_branch)

add_executable(test_c26_spline_motion_interpolator test_c26_spline_motion_interpolator.cpp)
target_include_directories(test_c26_spline_motion_interpolator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_spline_motion_interpolator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_spline_motion_interpolator COMMAND test_c26_spline_motion_interpolator)

add_executable(test_c26_stage_alpha_scope test_c26_stage_alpha_scope.cpp)
target_include_directories(test_c26_stage_alpha_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_stage_alpha_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_stage_alpha_scope COMMAND test_c26_stage_alpha_scope)

add_executable(test_c26_stage_color_scope test_c26_stage_color_scope.cpp)
target_include_directories(test_c26_stage_color_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_stage_color_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_stage_color_scope COMMAND test_c26_stage_color_scope)

add_executable(test_c26_state_bitmask test_c26_state_bitmask.cpp)
target_include_directories(test_c26_state_bitmask PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_state_bitmask PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_state_bitmask COMMAND test_c26_state_bitmask)

add_executable(test_c26_state_snapshot test_c26_state_snapshot.cpp)
target_include_directories(test_c26_state_snapshot PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_state_snapshot PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_state_snapshot COMMAND test_c26_state_snapshot)

add_executable(test_c26_stress_loot_scenario test_c26_stress_loot_scenario.cpp)
target_include_directories(test_c26_stress_loot_scenario PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_stress_loot_scenario PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_stress_loot_scenario COMMAND test_c26_stress_loot_scenario)

add_executable(test_c26_strong_types test_c26_strong_types.cpp)
target_include_directories(test_c26_strong_types PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_strong_types PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_strong_types COMMAND test_c26_strong_types)

add_executable(test_c26_stun_handler test_c26_stun_handler.cpp)
target_include_directories(test_c26_stun_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_stun_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_stun_handler COMMAND test_c26_stun_handler)

add_executable(test_c26_target_handler test_c26_target_handler.cpp)
target_include_directories(test_c26_target_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_target_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_target_handler COMMAND test_c26_target_handler)

add_executable(test_c26_target_packet_codec test_c26_target_packet_codec.cpp)
target_include_directories(test_c26_target_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_target_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_target_packet_codec COMMAND test_c26_target_packet_codec)

add_executable(test_c26_tea_crypto_provider test_c26_tea_crypto_provider.cpp)
target_include_directories(test_c26_tea_crypto_provider PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_tea_crypto_provider PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_tea_crypto_provider COMMAND test_c26_tea_crypto_provider)

add_executable(test_c26_terrain_frustum_culler test_c26_terrain_frustum_culler.cpp)
target_include_directories(test_c26_terrain_frustum_culler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_frustum_culler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_frustum_culler COMMAND test_c26_terrain_frustum_culler)

add_executable(test_c26_terrain_height_sampler test_c26_terrain_height_sampler.cpp)
target_include_directories(test_c26_terrain_height_sampler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_height_sampler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_height_sampler COMMAND test_c26_terrain_height_sampler)

add_executable(test_c26_terrain_lod_manager test_c26_terrain_lod_manager.cpp)
target_include_directories(test_c26_terrain_lod_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_lod_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_lod_manager COMMAND test_c26_terrain_lod_manager)

add_executable(test_c26_terrain_patch_batcher test_c26_terrain_patch_batcher.cpp)
target_include_directories(test_c26_terrain_patch_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_patch_batcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_patch_batcher COMMAND test_c26_terrain_patch_batcher)

add_executable(test_c26_terrain_sampler test_c26_terrain_sampler.cpp)
target_include_directories(test_c26_terrain_sampler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_sampler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_sampler COMMAND test_c26_terrain_sampler)

add_executable(test_c26_terrain_shader_pipeline test_c26_terrain_shader_pipeline.cpp)
target_include_directories(test_c26_terrain_shader_pipeline PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_shader_pipeline PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_shader_pipeline COMMAND test_c26_terrain_shader_pipeline)

add_executable(test_c26_terrain_shadow_proj test_c26_terrain_shadow_proj.cpp)
target_include_directories(test_c26_terrain_shadow_proj PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_shadow_proj PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_shadow_proj COMMAND test_c26_terrain_shadow_proj)

add_executable(test_c26_terrain_splat_weights test_c26_terrain_splat_weights.cpp)
target_include_directories(test_c26_terrain_splat_weights PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_splat_weights PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_splat_weights COMMAND test_c26_terrain_splat_weights)

add_executable(test_c26_tex_cache test_c26_tex_cache.cpp)
target_include_directories(test_c26_tex_cache PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_tex_cache PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_tex_cache COMMAND test_c26_tex_cache)

add_executable(test_c26_tex_transform_scope test_c26_tex_transform_scope.cpp)
target_include_directories(test_c26_tex_transform_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_tex_transform_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_tex_transform_scope COMMAND test_c26_tex_transform_scope)

add_executable(test_c26_texttail_engine test_c26_texttail_engine.cpp)
target_include_directories(test_c26_texttail_engine PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_texttail_engine PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_texttail_engine COMMAND test_c26_texttail_engine)

add_executable(test_c26_trade_domain test_c26_trade_domain.cpp)
target_include_directories(test_c26_trade_domain PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_trade_domain PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_trade_domain COMMAND test_c26_trade_domain)

add_executable(test_c26_ui_pass test_c26_ui_pass.cpp)
target_include_directories(test_c26_ui_pass PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_pass PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_pass COMMAND test_c26_ui_pass)

add_executable(test_c26_ui_prim_batcher test_c26_ui_prim_batcher.cpp)
target_include_directories(test_c26_ui_prim_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_prim_batcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_prim_batcher COMMAND test_c26_ui_prim_batcher)

add_executable(test_c26_ui_shader_pipeline test_c26_ui_shader_pipeline.cpp)
target_include_directories(test_c26_ui_shader_pipeline PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_shader_pipeline PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_shader_pipeline COMMAND test_c26_ui_shader_pipeline)

add_executable(test_c26_vb_stream_scope test_c26_vb_stream_scope.cpp)
target_include_directories(test_c26_vb_stream_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_vb_stream_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_vb_stream_scope COMMAND test_c26_vb_stream_scope)

add_executable(test_c26_vertex_decl_cache test_c26_vertex_decl_cache.cpp)
target_include_directories(test_c26_vertex_decl_cache PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_vertex_decl_cache PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_vertex_decl_cache COMMAND test_c26_vertex_decl_cache)

add_executable(test_c26_vfs_pack_manager test_c26_vfs_pack_manager.cpp)
target_include_directories(test_c26_vfs_pack_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_vfs_pack_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_vfs_pack_manager COMMAND test_c26_vfs_pack_manager)

add_executable(test_c26_view_proj_scope test_c26_view_proj_scope.cpp)
target_include_directories(test_c26_view_proj_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_view_proj_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_view_proj_scope COMMAND test_c26_view_proj_scope)

add_executable(test_c26_virtual_packet_generator test_c26_virtual_packet_generator.cpp)
target_include_directories(test_c26_virtual_packet_generator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_virtual_packet_generator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_virtual_packet_generator COMMAND test_c26_virtual_packet_generator)

add_executable(test_c26_vs_scope test_c26_vs_scope.cpp)
target_include_directories(test_c26_vs_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_vs_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_vs_scope COMMAND test_c26_vs_scope)

add_executable(test_c26_warp_packet_handler test_c26_warp_packet_handler.cpp)
target_include_directories(test_c26_warp_packet_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_warp_packet_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_warp_packet_handler COMMAND test_c26_warp_packet_handler)

add_executable(test_c26_water_surface_gen test_c26_water_surface_gen.cpp)
target_include_directories(test_c26_water_surface_gen PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_water_surface_gen PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_water_surface_gen COMMAND test_c26_water_surface_gen)

add_executable(test_c26_weapon_attach_res test_c26_weapon_attach_res.cpp)
target_include_directories(test_c26_weapon_attach_res PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_weapon_attach_res PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_weapon_attach_res COMMAND test_c26_weapon_attach_res)

add_executable(test_c26_whisper_command_handler test_c26_whisper_command_handler.cpp)
target_include_directories(test_c26_whisper_command_handler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_whisper_command_handler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_whisper_command_handler COMMAND test_c26_whisper_command_handler)

add_executable(test_c26_window_hierarchy test_c26_window_hierarchy.cpp)
target_include_directories(test_c26_window_hierarchy PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_window_hierarchy PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_window_hierarchy COMMAND test_c26_window_hierarchy)

add_executable(test_c26_world_matrix_scope test_c26_world_matrix_scope.cpp)
target_include_directories(test_c26_world_matrix_scope PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_world_matrix_scope PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_world_matrix_scope COMMAND test_c26_world_matrix_scope)

add_executable(test_c26_world_packet_codec test_c26_world_packet_codec.cpp)
target_include_directories(test_c26_world_packet_codec PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_world_packet_codec PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_world_packet_codec COMMAND test_c26_world_packet_codec)



add_executable(test_c26_player_stats test_c26_player_stats.cpp)
target_include_directories(test_c26_player_stats PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_stats PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_stats COMMAND test_c26_player_stats)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_player_equipment test_c26_player_equipment.cpp)
target_include_directories(test_c26_player_equipment PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_equipment PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_equipment COMMAND test_c26_player_equipment)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_validator test_c26_skill_validator.cpp)
target_include_directories(test_c26_skill_validator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_validator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_validator COMMAND test_c26_skill_validator)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_visuals test_c26_skill_visuals.cpp)
target_include_directories(test_c26_skill_visuals PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_visuals PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_visuals COMMAND test_c26_skill_visuals)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_equipment test_c26_actor_equipment.cpp)
target_include_directories(test_c26_actor_equipment PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_equipment PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_equipment COMMAND test_c26_actor_equipment)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_combat_state test_c26_actor_combat_state.cpp)
target_include_directories(test_c26_actor_combat_state PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_combat_state PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_combat_state COMMAND test_c26_actor_combat_state)

