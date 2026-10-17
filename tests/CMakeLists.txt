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

enable_testing()
add_test(NAME StranglerFacadeTest COMMAND test_c26_strangler_facade)
add_test(NAME PandoraSimulationTest COMMAND test_c26_e2e_pandora_simulation)
add_test(NAME StateUnificationTest COMMAND test_c26_state_unification)
add_test(NAME SkillCooldownTest COMMAND test_c26_skill_cooldown)
add_test(NAME SkillDomainTest COMMAND test_c26_skill_domain)
add_test(NAME ItemBonusTest COMMAND test_c26_item_bonus)
add_test(NAME InventoryDomainTest COMMAND test_c26_inventory_domain)
add_test(NAME QuickslotDomainTest COMMAND test_c26_quickslot_domain)
add_test(NAME ActorRegistryTest COMMAND test_c26_actor_registry)
add_test(NAME SpatialHashGridTest COMMAND test_c26_spatial_hash_grid)
add_test(NAME ZeroCopyRingBufferTest COMMAND test_c26_zero_copy_ring_buffer)
add_test(NAME WorldContextTest COMMAND test_c26_world_context)
add_test(NAME DomainErrorsEventsTest COMMAND test_c26_domain_errors_events)
add_test(NAME InstanceComponentsTest COMMAND test_c26_instance_components)
add_test(NAME InstancePhysicsEngineTest COMMAND test_c26_instance_physics_engine)
add_test(NAME InstanceCombatEngineTest COMMAND test_c26_instance_combat_engine)
add_test(NAME TypeSanitizationTest COMMAND test_c26_type_sanitization)


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

add_executable(test_c26_gltf_skinning test_c26_gltf_skinning.cpp)
target_include_directories(test_c26_gltf_skinning PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_gltf_skinning PRIVATE
    EterModelLib
)
add_test(NAME GltfSkinningTest COMMAND test_c26_gltf_skinning)

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

add_executable(test_c26_phase_game_exchange_bridge test_c26_phase_game_exchange_bridge.cpp ${CMAKE_SOURCE_DIR}/src/Client/Bridge/PhaseGameExchangeBridge.cpp)

target_include_directories(test_c26_phase_game_exchange_bridge PRIVATE

    ${CMAKE_SOURCE_DIR}/src

    ${CMAKE_SOURCE_DIR}/src/Client

    ${CMAKE_CURRENT_SOURCE_DIR}

    ${CMAKE_SOURCE_DIR}/extern/include

)

target_link_libraries(test_c26_phase_game_exchange_bridge PRIVATE

    Client2026

    EterBase

)

add_test(NAME PhaseGameExchangeBridgeTest COMMAND test_c26_phase_game_exchange_bridge)

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

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_movement test_c26_actor_movement.cpp)
target_include_directories(test_c26_actor_movement PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_movement PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_movement COMMAND test_c26_actor_movement)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_expected test_c26_terrain_expected.cpp)
target_include_directories(test_c26_terrain_expected PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_expected PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_expected COMMAND test_c26_terrain_expected)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_vfs_mmap test_c26_vfs_mmap.cpp)
target_include_directories(test_c26_vfs_mmap PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_vfs_mmap PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_vfs_mmap COMMAND test_c26_vfs_mmap)


# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_area_collision test_c26_area_collision.cpp)
target_include_directories(test_c26_area_collision PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_area_collision PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_area_collision COMMAND test_c26_area_collision)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_area_streamer test_c26_area_streamer.cpp)
target_include_directories(test_c26_area_streamer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_area_streamer PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_area_streamer COMMAND test_c26_area_streamer)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_minimap_scaler test_c26_minimap_scaler.cpp)
target_include_directories(test_c26_minimap_scaler PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_minimap_scaler PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_minimap_scaler COMMAND test_c26_minimap_scaler)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_minimap_waypoints test_c26_minimap_waypoints.cpp)
target_include_directories(test_c26_minimap_waypoints PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_minimap_waypoints PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_minimap_waypoints COMMAND test_c26_minimap_waypoints)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_minimap_texture test_c26_minimap_texture.cpp)
target_include_directories(test_c26_minimap_texture PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_minimap_texture PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_minimap_texture COMMAND test_c26_minimap_texture)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_text_style test_c26_text_style.cpp)
target_include_directories(test_c26_text_style PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_text_style PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_text_style COMMAND test_c26_text_style)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_glyph_atlas test_c26_glyph_atlas.cpp)
target_include_directories(test_c26_glyph_atlas PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_glyph_atlas PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_glyph_atlas COMMAND test_c26_glyph_atlas)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_text_layout test_c26_text_layout.cpp)
target_include_directories(test_c26_text_layout PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_text_layout PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_text_layout COMMAND test_c26_text_layout)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_composition_string test_c26_composition_string.cpp)
target_include_directories(test_c26_composition_string PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_composition_string PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_composition_string COMMAND test_c26_composition_string)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_keyboard_input test_c26_keyboard_input.cpp)
target_include_directories(test_c26_keyboard_input PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_keyboard_input PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_keyboard_input COMMAND test_c26_keyboard_input)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_ui_render_batcher test_c26_ui_render_batcher.cpp)
target_include_directories(test_c26_ui_render_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_render_batcher PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_render_batcher COMMAND test_c26_ui_render_batcher)

add_executable(test_c26_dynamic_quad_batcher test_c26_dynamic_quad_batcher.cpp)
target_include_directories(test_c26_dynamic_quad_batcher PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_dynamic_quad_batcher PRIVATE
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
    DirectX
)
add_test(NAME test_c26_dynamic_quad_batcher COMMAND test_c26_dynamic_quad_batcher)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_ui_hierarchy test_c26_ui_hierarchy.cpp)
target_include_directories(test_c26_ui_hierarchy PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_hierarchy PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_hierarchy COMMAND test_c26_ui_hierarchy)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_ui_slot_grid test_c26_ui_slot_grid.cpp)
target_include_directories(test_c26_ui_slot_grid PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_slot_grid PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_slot_grid COMMAND test_c26_ui_slot_grid)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_ui_event_listener test_c26_ui_event_listener.cpp)
target_include_directories(test_c26_ui_event_listener PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_event_listener PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_event_listener COMMAND test_c26_ui_event_listener)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_ui_layout_manager test_c26_ui_layout_manager.cpp)
target_include_directories(test_c26_ui_layout_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_layout_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_layout_manager COMMAND test_c26_ui_layout_manager)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_vfs_result test_c26_vfs_result.cpp)
target_include_directories(test_c26_vfs_result PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_vfs_result PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_vfs_result COMMAND test_c26_vfs_result)



add_executable(test_c26_terrain_splat test_c26_terrain_splat.cpp)
target_include_directories(test_c26_terrain_splat PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_splat PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_splat COMMAND test_c26_terrain_splat)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_water test_c26_terrain_water.cpp)
target_include_directories(test_c26_terrain_water PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_water PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_water COMMAND test_c26_terrain_water)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_culling test_c26_terrain_culling.cpp)
target_include_directories(test_c26_terrain_culling PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_culling PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_culling COMMAND test_c26_terrain_culling)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_player_skills test_c26_player_skills.cpp)
target_include_directories(test_c26_player_skills PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_skills PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_skills COMMAND test_c26_player_skills)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_player_quickslot test_c26_player_quickslot.cpp)
target_include_directories(test_c26_player_quickslot PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_quickslot PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_quickslot COMMAND test_c26_player_quickslot)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_player_inventory test_c26_player_inventory.cpp)
target_include_directories(test_c26_player_inventory PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_inventory PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_inventory COMMAND test_c26_player_inventory)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_formulas test_c26_skill_formulas.cpp)
target_include_directories(test_c26_skill_formulas PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_formulas PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_formulas COMMAND test_c26_skill_formulas)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_loader test_c26_skill_loader.cpp)
target_include_directories(test_c26_skill_loader PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_loader PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_loader COMMAND test_c26_skill_loader)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_effects test_c26_actor_effects.cpp)
target_include_directories(test_c26_actor_effects PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_effects PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_effects COMMAND test_c26_actor_effects)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_sound test_c26_actor_sound.cpp)
target_include_directories(test_c26_actor_sound PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_sound PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_sound COMMAND test_c26_actor_sound)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_texttail test_c26_actor_texttail.cpp)
target_include_directories(test_c26_actor_texttail PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_texttail PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_texttail COMMAND test_c26_actor_texttail)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_warp_bridge test_c26_phase_game_warp_bridge.cpp)
target_include_directories(test_c26_phase_game_warp_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_warp_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_warp_bridge COMMAND test_c26_phase_game_warp_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_messenger_bridge test_c26_phase_game_messenger_bridge.cpp)
target_include_directories(test_c26_phase_game_messenger_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_messenger_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_messenger_bridge COMMAND test_c26_phase_game_messenger_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_safebox_bridge test_c26_phase_game_safebox_bridge.cpp)
target_include_directories(test_c26_phase_game_safebox_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_safebox_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_safebox_bridge COMMAND test_c26_phase_game_safebox_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_shop_bridge test_c26_phase_game_shop_bridge.cpp)
target_include_directories(test_c26_phase_game_shop_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_shop_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_shop_bridge COMMAND test_c26_phase_game_shop_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_party_bridge test_c26_phase_game_party_bridge.cpp)
target_include_directories(test_c26_phase_game_party_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_party_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_party_bridge COMMAND test_c26_phase_game_party_bridge)

add_executable(test_c26_phase_game_combat_bridge test_c26_phase_game_combat_bridge.cpp)
target_include_directories(test_c26_phase_game_combat_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
)
target_compile_definitions(test_c26_phase_game_combat_bridge PRIVATE TEST_MODE_DISABLE_STDAFX)
target_link_libraries(test_c26_phase_game_combat_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
)
add_test(NAME test_c26_phase_game_combat_bridge COMMAND test_c26_phase_game_combat_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_collision test_c26_actor_collision.cpp)
target_include_directories(test_c26_actor_collision PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_collision PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_collision COMMAND test_c26_actor_collision)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_title test_c26_actor_title.cpp)
target_include_directories(test_c26_actor_title PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_title PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_title COMMAND test_c26_actor_title)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_event_dialog_box test_c26_event_dialog_box.cpp ${CMAKE_SOURCE_DIR}/src/Client/Events/EventDialogBoxController.cpp)
target_include_directories(test_c26_event_dialog_box PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_compile_definitions(test_c26_event_dialog_box PRIVATE TEST_MODE_DISABLE_STDAFX)
target_link_libraries(test_c26_event_dialog_box PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
)
add_test(NAME test_c26_event_dialog_box COMMAND test_c26_event_dialog_box)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_dummy test_dummy.cpp)
target_include_directories(test_dummy PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_dummy PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_dummy COMMAND test_dummy)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_event_script_parser test_c26_event_script_parser.cpp)
target_include_directories(test_c26_event_script_parser PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_event_script_parser PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_event_script_parser COMMAND test_c26_event_script_parser)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_ui_tooltip_renderer test_c26_ui_tooltip_renderer.cpp)
target_include_directories(test_c26_ui_tooltip_renderer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_tooltip_renderer PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_tooltip_renderer COMMAND test_c26_ui_tooltip_renderer)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_ui_drag_drop test_c26_ui_drag_drop.cpp)
target_include_directories(test_c26_ui_drag_drop PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_drag_drop PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_drag_drop COMMAND test_c26_ui_drag_drop)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_ui_modal_dialog test_c26_ui_modal_dialog.cpp)
target_include_directories(test_c26_ui_modal_dialog PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_ui_modal_dialog PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_ui_modal_dialog COMMAND test_c26_ui_modal_dialog)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_area_ambient_sound test_c26_area_ambient_sound.cpp)
target_include_directories(test_c26_area_ambient_sound PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_area_ambient_sound PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_area_ambient_sound COMMAND test_c26_area_ambient_sound)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_area_portal_manager test_c26_area_portal_manager.cpp)
target_include_directories(test_c26_area_portal_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_area_portal_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_area_portal_manager COMMAND test_c26_area_portal_manager)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_foliage test_c26_terrain_foliage.cpp)
target_include_directories(test_c26_terrain_foliage PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_foliage PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_foliage COMMAND test_c26_terrain_foliage)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_shadow_map test_c26_terrain_shadow_map.cpp)
target_include_directories(test_c26_terrain_shadow_map PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_shadow_map PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_shadow_map COMMAND test_c26_terrain_shadow_map)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_player_auto_potion test_c26_player_auto_potion.cpp)
target_include_directories(test_c26_player_auto_potion PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_auto_potion PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_auto_potion COMMAND test_c26_player_auto_potion)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_player_target_manager test_c26_player_target_manager.cpp)
target_include_directories(test_c26_player_target_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_target_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_target_manager COMMAND test_c26_player_target_manager)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_player_party_manager test_c26_player_party_manager.cpp)
target_include_directories(test_c26_player_party_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_party_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_party_manager COMMAND test_c26_player_party_manager)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_player_quest_manager test_c26_player_quest_manager.cpp)
target_include_directories(test_c26_player_quest_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_player_quest_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_player_quest_manager COMMAND test_c26_player_quest_manager)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_tree_navigator test_c26_skill_tree_navigator.cpp)
target_include_directories(test_c26_skill_tree_navigator PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_tree_navigator PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_tree_navigator COMMAND test_c26_skill_tree_navigator)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_affect_bridge test_c26_skill_affect_bridge.cpp)
target_include_directories(test_c26_skill_affect_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_affect_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_affect_bridge COMMAND test_c26_skill_affect_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_sound_mapper test_c26_skill_sound_mapper.cpp)
target_include_directories(test_c26_skill_sound_mapper PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_sound_mapper PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_sound_mapper COMMAND test_c26_skill_sound_mapper)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_buff_manager test_c26_skill_buff_manager.cpp)
target_include_directories(test_c26_skill_buff_manager PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_buff_manager PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_buff_manager COMMAND test_c26_skill_buff_manager)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_mining_bridge test_c26_phase_game_mining_bridge.cpp)
target_include_directories(test_c26_phase_game_mining_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_mining_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_mining_bridge COMMAND test_c26_phase_game_mining_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_fishing_bridge test_c26_phase_game_fishing_bridge.cpp)
target_include_directories(test_c26_phase_game_fishing_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_fishing_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_fishing_bridge COMMAND test_c26_phase_game_fishing_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_aura test_c26_actor_aura.cpp)
target_include_directories(test_c26_actor_aura PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_aura PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_aura COMMAND test_c26_actor_aura)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_guild_emblem test_c26_actor_guild_emblem.cpp)
target_include_directories(test_c26_actor_guild_emblem PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_guild_emblem PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_guild_emblem COMMAND test_c26_actor_guild_emblem)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_mount test_c26_actor_mount.cpp)
target_include_directories(test_c26_actor_mount PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_mount PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_mount COMMAND test_c26_actor_mount)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_item_bridge test_c26_phase_game_item_bridge.cpp)
target_include_directories(test_c26_phase_game_item_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_item_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_item_bridge COMMAND test_c26_phase_game_item_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_luria_dh2 test_c26_luria_dh2.cpp)
target_include_directories(test_c26_luria_dh2 PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_luria_dh2 PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_luria_dh2 COMMAND test_c26_luria_dh2)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_whisper_bridge test_c26_phase_game_whisper_bridge.cpp)
target_include_directories(test_c26_phase_game_whisper_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_whisper_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_whisper_bridge COMMAND test_c26_phase_game_whisper_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_phase_game_dungeon_bridge test_c26_phase_game_dungeon_bridge.cpp)
target_include_directories(test_c26_phase_game_dungeon_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_phase_game_dungeon_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_phase_game_dungeon_bridge COMMAND test_c26_phase_game_dungeon_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_event_answer_choice test_c26_event_answer_choice.cpp)
target_include_directories(test_c26_event_answer_choice PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_event_answer_choice PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_event_answer_choice COMMAND test_c26_event_answer_choice)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_event_state_flags test_c26_event_state_flags.cpp)
target_include_directories(test_c26_event_state_flags PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_event_state_flags PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_event_state_flags COMMAND test_c26_event_state_flags)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_slot_highlight test_c26_slot_highlight.cpp)
target_include_directories(test_c26_slot_highlight PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_slot_highlight PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_slot_highlight COMMAND test_c26_slot_highlight)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_slot_drag_drop test_c26_slot_drag_drop.cpp)
target_include_directories(test_c26_slot_drag_drop PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_slot_drag_drop PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_slot_drag_drop COMMAND test_c26_slot_drag_drop)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_slot_cooldown test_c26_slot_cooldown.cpp)
target_include_directories(test_c26_slot_cooldown PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_slot_cooldown PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_slot_cooldown COMMAND test_c26_slot_cooldown)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_slot_count_text test_c26_slot_count_text.cpp)
target_include_directories(test_c26_slot_count_text PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_slot_count_text PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_slot_count_text COMMAND test_c26_slot_count_text)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_heightmap test_c26_terrain_heightmap.cpp)
target_include_directories(test_c26_terrain_heightmap PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_heightmap PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_heightmap COMMAND test_c26_terrain_heightmap)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_sector_shadow test_c26_terrain_sector_shadow.cpp)
target_include_directories(test_c26_terrain_sector_shadow PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_sector_shadow PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_sector_shadow COMMAND test_c26_terrain_sector_shadow)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_layer_blending test_c26_terrain_layer_blending.cpp)
target_include_directories(test_c26_terrain_layer_blending PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_layer_blending PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_layer_blending COMMAND test_c26_terrain_layer_blending)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_terrain_normal_map test_c26_terrain_normal_map.cpp)
target_include_directories(test_c26_terrain_normal_map PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_terrain_normal_map PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_terrain_normal_map COMMAND test_c26_terrain_normal_map)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_speedtree_lod test_c26_speedtree_lod.cpp)
target_include_directories(test_c26_speedtree_lod PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_speedtree_lod PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_speedtree_lod COMMAND test_c26_speedtree_lod)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_speedtree_instancing test_c26_speedtree_instancing.cpp)
target_include_directories(test_c26_speedtree_instancing PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_speedtree_instancing PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_speedtree_instancing COMMAND test_c26_speedtree_instancing)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_speedtree_wind test_c26_speedtree_wind.cpp)
target_include_directories(test_c26_speedtree_wind PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_speedtree_wind PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_speedtree_wind COMMAND test_c26_speedtree_wind)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_speedtree_billboard test_c26_speedtree_billboard.cpp)
target_include_directories(test_c26_speedtree_billboard PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_speedtree_billboard PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_speedtree_billboard COMMAND test_c26_speedtree_billboard)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_text_tag_color test_c26_text_tag_color.cpp)
target_include_directories(test_c26_text_tag_color PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_text_tag_color PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_text_tag_color COMMAND test_c26_text_tag_color)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_text_hyperlink test_c26_text_hyperlink.cpp)
target_include_directories(test_c26_text_hyperlink PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_text_hyperlink PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_text_hyperlink COMMAND test_c26_text_hyperlink)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_text_glyph_metrics test_c26_text_glyph_metrics.cpp)
target_include_directories(test_c26_text_glyph_metrics PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_text_glyph_metrics PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_text_glyph_metrics COMMAND test_c26_text_glyph_metrics)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_text_linebreak test_c26_text_linebreak.cpp)
target_include_directories(test_c26_text_linebreak PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_text_linebreak PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_text_linebreak COMMAND test_c26_text_linebreak)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_pet test_c26_actor_pet.cpp)
target_include_directories(test_c26_actor_pet PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_pet PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_pet COMMAND test_c26_actor_pet)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_acce test_c26_actor_acce.cpp)
target_include_directories(test_c26_actor_acce PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_acce PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_acce COMMAND test_c26_actor_acce)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_weapon_trace test_c26_actor_weapon_trace.cpp)
target_include_directories(test_c26_actor_weapon_trace PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_weapon_trace PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_weapon_trace COMMAND test_c26_actor_weapon_trace)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_affect_bridge test_c26_actor_affect_bridge.cpp)
target_include_directories(test_c26_actor_affect_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_affect_bridge PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_affect_bridge COMMAND test_c26_actor_affect_bridge)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_actor_emotion test_c26_actor_emotion.cpp)
target_include_directories(test_c26_actor_emotion PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_actor_emotion PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_actor_emotion COMMAND test_c26_actor_emotion)

# --- Autonomicznie zarejestrowane testy roju Jules (Architektura 2026) ---
add_executable(test_c26_skill_motion test_c26_skill_motion.cpp)
target_include_directories(test_c26_skill_motion PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
    ${CMAKE_SOURCE_DIR}/vendor/libsodium/src/libsodium/include
    ${CMAKE_SOURCE_DIR}/vendor/zstd-1.5.7/lib
)
target_link_libraries(test_c26_skill_motion PRIVATE
    Client2026
    EterLib
    EterBase
    libzstd_static
    sodium
    lzo2
    mio
    ws2_32
)
add_test(NAME test_c26_skill_motion COMMAND test_c26_skill_motion)


# --- FAZY 64-72: 36 MODULOW SRP (T501-T536) ---

add_executable(test_c26_wm_slot_api test_c26_wm_slot_api.cpp ${CMAKE_SOURCE_DIR}/src/Client/UI/WindowManagerSlotApiBridge.cpp)
target_include_directories(test_c26_wm_slot_api PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_wm_slot_api PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_wm_slot_api COMMAND test_c26_wm_slot_api)


add_executable(test_c26_wm_button_api test_c26_wm_button_api.cpp ${CMAKE_SOURCE_DIR}/src/Client/UI/WindowManagerButtonApiBridge.cpp)
target_include_directories(test_c26_wm_button_api PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_wm_button_api PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_wm_button_api COMMAND test_c26_wm_button_api)


add_executable(test_c26_wm_layout_api test_c26_wm_layout_api.cpp ${CMAKE_SOURCE_DIR}/src/Client/UI/WindowManagerLayoutApiBridge.cpp)
target_include_directories(test_c26_wm_layout_api PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_wm_layout_api PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_wm_layout_api COMMAND test_c26_wm_layout_api)


add_executable(test_c26_wm_input_api test_c26_wm_input_api.cpp ${CMAKE_SOURCE_DIR}/src/Client/UI/WindowManagerInputApiBridge.cpp)
target_include_directories(test_c26_wm_input_api PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_wm_input_api PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_wm_input_api COMMAND test_c26_wm_input_api)


add_executable(test_c26_actor_pixel_movement test_c26_actor_pixel_movement.cpp ${CMAKE_SOURCE_DIR}/src/Client/Actors/ActorPixelMovementIntegrator.cpp)
target_include_directories(test_c26_actor_pixel_movement PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_actor_pixel_movement PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_actor_pixel_movement COMMAND test_c26_actor_pixel_movement)


add_executable(test_c26_actor_target_rule test_c26_actor_target_rule.cpp ${CMAKE_SOURCE_DIR}/src/Client/Actors/ActorTargetRuleValidator.cpp)
target_include_directories(test_c26_actor_target_rule PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_actor_target_rule PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_actor_target_rule COMMAND test_c26_actor_target_rule)


add_executable(test_c26_actor_type_classifier test_c26_actor_type_classifier.cpp ${CMAKE_SOURCE_DIR}/src/Client/Actors/ActorTypeClassifier.cpp)
target_include_directories(test_c26_actor_type_classifier PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_actor_type_classifier PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_actor_type_classifier COMMAND test_c26_actor_type_classifier)


add_executable(test_c26_actor_tcp_state test_c26_actor_tcp_state.cpp ${CMAKE_SOURCE_DIR}/src/Client/Actors/ActorTcpStateManager.cpp)
target_include_directories(test_c26_actor_tcp_state PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_actor_tcp_state PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_actor_tcp_state COMMAND test_c26_actor_tcp_state)


add_executable(test_c26_harness_player_rpc test_c26_harness_player_rpc.cpp ${CMAKE_SOURCE_DIR}/src/Client/IPC/HarnessPlayerRpcRouter.cpp)
target_include_directories(test_c26_harness_player_rpc PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_harness_player_rpc PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_harness_player_rpc COMMAND test_c26_harness_player_rpc)


add_executable(test_c26_harness_world_query test_c26_harness_world_query.cpp ${CMAKE_SOURCE_DIR}/src/Client/IPC/HarnessWorldQueryRouter.cpp)
target_include_directories(test_c26_harness_world_query PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_harness_world_query PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_harness_world_query COMMAND test_c26_harness_world_query)


add_executable(test_c26_harness_auth_protocol test_c26_harness_auth_protocol.cpp ${CMAKE_SOURCE_DIR}/src/Client/IPC/HarnessAuthProtocolRouter.cpp)
target_include_directories(test_c26_harness_auth_protocol PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_harness_auth_protocol PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_harness_auth_protocol COMMAND test_c26_harness_auth_protocol)


add_executable(test_c26_harness_engine_control test_c26_harness_engine_control.cpp ${CMAKE_SOURCE_DIR}/src/Client/IPC/HarnessEngineControlRouter.cpp)
target_include_directories(test_c26_harness_engine_control PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_harness_engine_control PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_harness_engine_control COMMAND test_c26_harness_engine_control)


add_executable(test_c26_ime_composition test_c26_ime_composition.cpp ${CMAKE_SOURCE_DIR}/src/Client/Input/IMECompositionManager.cpp)
target_include_directories(test_c26_ime_composition PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ime_composition PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_ime_composition COMMAND test_c26_ime_composition)


add_executable(test_c26_ime_candidate_list test_c26_ime_candidate_list.cpp ${CMAKE_SOURCE_DIR}/src/Client/Input/IMECandidateListController.cpp)
target_include_directories(test_c26_ime_candidate_list PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ime_candidate_list PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_ime_candidate_list COMMAND test_c26_ime_candidate_list)


add_executable(test_c26_ime_clipboard test_c26_ime_clipboard.cpp ${CMAKE_SOURCE_DIR}/src/Client/Input/IMEClipboardBridge.cpp)
target_include_directories(test_c26_ime_clipboard PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ime_clipboard PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_ime_clipboard COMMAND test_c26_ime_clipboard)


add_executable(test_c26_ime_os_hook test_c26_ime_os_hook.cpp ${CMAKE_SOURCE_DIR}/src/Client/Input/IMEOsHookController.cpp)
target_include_directories(test_c26_ime_os_hook PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ime_os_hook PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_ime_os_hook COMMAND test_c26_ime_os_hook)


add_executable(test_c26_skill_table_registry test_c26_skill_table_registry.cpp ${CMAKE_SOURCE_DIR}/src/Client/Skills/SkillTableRegistry.cpp)
target_include_directories(test_c26_skill_table_registry PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_skill_table_registry PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_skill_table_registry COMMAND test_c26_skill_table_registry)


add_executable(test_c26_skill_desc_parser test_c26_skill_desc_parser.cpp ${CMAKE_SOURCE_DIR}/src/Client/Skills/SkillDescriptionParser.cpp)
target_include_directories(test_c26_skill_desc_parser PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_skill_desc_parser PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_skill_desc_parser COMMAND test_c26_skill_desc_parser)


add_executable(test_c26_skill_grade_icon test_c26_skill_grade_icon.cpp ${CMAKE_SOURCE_DIR}/src/Client/Skills/SkillGradeIconManager.cpp)
target_include_directories(test_c26_skill_grade_icon PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_skill_grade_icon PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_skill_grade_icon COMMAND test_c26_skill_grade_icon)


add_executable(test_c26_skill_requirement test_c26_skill_requirement.cpp ${CMAKE_SOURCE_DIR}/src/Client/Skills/SkillRequirementEvaluator.cpp)
target_include_directories(test_c26_skill_requirement PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_skill_requirement PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_skill_requirement COMMAND test_c26_skill_requirement)


add_executable(test_c26_ui_window_hierarchy test_c26_ui_window_hierarchy.cpp ${CMAKE_SOURCE_DIR}/src/Client/UI/UIWindowHierarchyController.cpp)
target_include_directories(test_c26_ui_window_hierarchy PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ui_window_hierarchy PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_ui_window_hierarchy COMMAND test_c26_ui_window_hierarchy)


add_executable(test_c26_ui_window_coord test_c26_ui_window_coord.cpp ${CMAKE_SOURCE_DIR}/src/Client/UI/UIWindowCoordinateTransform.cpp)
target_include_directories(test_c26_ui_window_coord PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ui_window_coord PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_ui_window_coord COMMAND test_c26_ui_window_coord)


add_executable(test_c26_ui_window_flags test_c26_ui_window_flags.cpp ${CMAKE_SOURCE_DIR}/src/Client/UI/UIWindowStateFlags.cpp)
target_include_directories(test_c26_ui_window_flags PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ui_window_flags PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_ui_window_flags COMMAND test_c26_ui_window_flags)


add_executable(test_c26_ui_window_events test_c26_ui_window_events.cpp ${CMAKE_SOURCE_DIR}/src/Client/UI/UIWindowEventListenerDispatch.cpp)
target_include_directories(test_c26_ui_window_events PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_ui_window_events PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_ui_window_events COMMAND test_c26_ui_window_events)


add_executable(test_c26_phase_game_char_state test_c26_phase_game_char_state.cpp ${CMAKE_SOURCE_DIR}/src/Client/Bridge/PhaseGameCharacterStateBridge.cpp)
target_include_directories(test_c26_phase_game_char_state PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_phase_game_char_state PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_phase_game_char_state COMMAND test_c26_phase_game_char_state)


add_executable(test_c26_phase_game_chat_whisper test_c26_phase_game_chat_whisper.cpp ${CMAKE_SOURCE_DIR}/src/Client/Bridge/PhaseGameChatWhisperBridge.cpp)
target_include_directories(test_c26_phase_game_chat_whisper PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_phase_game_chat_whisper PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_phase_game_chat_whisper COMMAND test_c26_phase_game_chat_whisper)


add_executable(test_c26_phase_game_duel_pvp test_c26_phase_game_duel_pvp.cpp ${CMAKE_SOURCE_DIR}/src/Client/Bridge/PhaseGameDuelPvpBridge.cpp)
target_include_directories(test_c26_phase_game_duel_pvp PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_phase_game_duel_pvp PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_phase_game_duel_pvp COMMAND test_c26_phase_game_duel_pvp)


add_executable(test_c26_phase_game_observer test_c26_phase_game_observer.cpp ${CMAKE_SOURCE_DIR}/src/Client/Bridge/PhaseGameObserverBridge.cpp)
target_include_directories(test_c26_phase_game_observer PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_phase_game_observer PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_phase_game_observer COMMAND test_c26_phase_game_observer)


add_executable(test_c26_player_pos_tracker test_c26_player_pos_tracker.cpp ${CMAKE_SOURCE_DIR}/src/Client/Player/PlayerMainActorPositionTracker.cpp)
target_include_directories(test_c26_player_pos_tracker PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_player_pos_tracker PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_player_pos_tracker COMMAND test_c26_player_pos_tracker)


add_executable(test_c26_player_entity_picker test_c26_player_entity_picker.cpp ${CMAKE_SOURCE_DIR}/src/Client/Player/PlayerEntityPicker.cpp)
target_include_directories(test_c26_player_entity_picker PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_player_entity_picker PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_player_entity_picker COMMAND test_c26_player_entity_picker)


add_executable(test_c26_player_effect_bridge test_c26_player_effect_bridge.cpp ${CMAKE_SOURCE_DIR}/src/Client/Player/PlayerEffectRegistryBridge.cpp)
target_include_directories(test_c26_player_effect_bridge PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_player_effect_bridge PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_player_effect_bridge COMMAND test_c26_player_effect_bridge)


add_executable(test_c26_player_target_vid test_c26_player_target_vid.cpp ${CMAKE_SOURCE_DIR}/src/Client/Player/PlayerTargetVidController.cpp)
target_include_directories(test_c26_player_target_vid PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_player_target_vid PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_player_target_vid COMMAND test_c26_player_target_vid)


add_executable(test_c26_terrain_coord_mapper test_c26_terrain_coord_mapper.cpp ${CMAKE_SOURCE_DIR}/src/Client/Terrain/TerrainCoordinateMapper.cpp)
target_include_directories(test_c26_terrain_coord_mapper PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_terrain_coord_mapper PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_terrain_coord_mapper COMMAND test_c26_terrain_coord_mapper)


add_executable(test_c26_terrain_sector_loader test_c26_terrain_sector_loader.cpp ${CMAKE_SOURCE_DIR}/src/Client/Terrain/TerrainSectorLoader.cpp)
target_include_directories(test_c26_terrain_sector_loader PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_terrain_sector_loader PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_terrain_sector_loader COMMAND test_c26_terrain_sector_loader)


add_executable(test_c26_terrain_height_querier test_c26_terrain_height_querier.cpp ${CMAKE_SOURCE_DIR}/src/Client/Terrain/TerrainHeightCollisionQuerier.cpp)
target_include_directories(test_c26_terrain_height_querier PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_terrain_height_querier PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_terrain_height_querier COMMAND test_c26_terrain_height_querier)


add_executable(test_c26_terrain_env_config test_c26_terrain_env_config.cpp ${CMAKE_SOURCE_DIR}/src/Client/Terrain/TerrainEnvironmentConfigManager.cpp)
target_include_directories(test_c26_terrain_env_config PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/Client
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_terrain_env_config PRIVATE Client2026 EterLib EterBase ws2_32)
add_test(NAME test_c26_terrain_env_config COMMAND test_c26_terrain_env_config)

add_executable(test_c26_minimap_coord_projector test_c26_minimap_coord_projector.cpp ${CMAKE_SOURCE_DIR}/src/UserInterface/MiniMap/MiniMapCoordinateProjector.cpp)
target_include_directories(test_c26_minimap_coord_projector PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/UserInterface
    ${CMAKE_SOURCE_DIR}/src/UserInterface/MiniMap
    ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(test_c26_minimap_coord_projector PRIVATE EterBase ws2_32)
add_test(NAME test_c26_minimap_coord_projector COMMAND test_c26_minimap_coord_projector)

add_executable(test_c26_texture_swizzle test_c26_texture_swizzle.cpp)
target_include_directories(test_c26_texture_swizzle PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_texture_swizzle PRIVATE EterLib EterBase DirectX ws2_32)
add_test(NAME TextureSwizzleTest COMMAND test_c26_texture_swizzle)

add_executable(test_c26_world_small_fixes test_c26_world_small_fixes.cpp)
target_include_directories(test_c26_world_small_fixes PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/extern/include
)
target_link_libraries(test_c26_world_small_fixes PRIVATE DirectX)
add_test(NAME WorldSmallFixesTest COMMAND test_c26_world_small_fixes)


