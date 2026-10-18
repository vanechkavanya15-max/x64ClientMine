#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderPacketTypes.h"
#include "EterLib/WgpuModule.h"
#include <thread>
#include <vector>
#include <atomic>

TEST_CASE("WgpuModule - Nonexistent DLL lifecycle & no crash")
{
	CWgpuModule& module = CWgpuModule::Instance();
	module.FreeLibrary();
	module.ResetCounters();

	// 1. Sprawdzenie stanu poczatkowego przed zaladowaniem
	CHECK_FALSE(module.IsLoaded());
	CHECK(module.GetHandle() == nullptr);
	CHECK(module.GetFunctions().initialize == nullptr);
	CHECK(module.GetFunctions().render_frame == nullptr);
	CHECK(module.GetFunctions().register_texture == nullptr);
	CHECK(module.GetFunctions().unregister_texture == nullptr);
	CHECK(module.GetFunctions().resize == nullptr);
	CHECK(module.GetFunctions().destroy == nullptr);
	CHECK(module.GetFunctions().read_rgba_pixels == nullptr);

	// 2. Proba zaladowania nieistniejacej biblioteki DLL
	bool loadResult = module.Load("definitely_nonexistent_wgpu_module_xyz123.dll");
	CHECK_FALSE(loadResult);
	CHECK_FALSE(module.IsLoaded());
	CHECK(module.GetHandle() == nullptr);

	// 3. Bezpieczne wywolania metod przy braku modulu (brak crasha, brak wywolan PFN)
	CHECK_FALSE(module.Initialize(nullptr, 1280, 720));
	module.RenderFrame(nullptr);

	TRenderFramePacket dummyPacket{};
	module.RenderFrame(&dummyPacket);

	const uint8_t dummyTexData[64] = { 0 };
	CHECK_FALSE(module.RegisterTexture(1001, 8, 8, 1, dummyTexData, sizeof(dummyTexData)));
	CHECK_FALSE(module.RegisterTexture(0, 8, 8, 1, dummyTexData, sizeof(dummyTexData)));
	CHECK_FALSE(module.RegisterTexture(1002, 8, 8, 1, nullptr, 0));
	CHECK_FALSE(module.ReadbackFramebuffer(nullptr, 0, nullptr, nullptr));

	// 4. UnregisterTexture staje sie bezpiecznym no-op
	module.UnregisterTexture(1001);
	module.UnregisterTexture(0);

	module.Resize(1920, 1080);
	module.Destroy();
	module.FreeLibrary();

	CHECK_FALSE(module.IsLoaded());
}

TEST_CASE("WgpuModule - Safe UnregisterTexture after FreeLibrary")
{
	CWgpuModule& module = CWgpuModule::Instance();
	module.FreeLibrary();

	// UnregisterTexture musi byc bezpiecznym no-op po zamknieciu modulu
	for (uintptr_t id = 1; id <= 100; ++id)
	{
		module.UnregisterTexture(id);
	}
	CHECK_FALSE(module.IsLoaded());
}

TEST_CASE("WgpuModule - Reference and Transaction counters")
{
	CWgpuModule& module = CWgpuModule::Instance();
	module.ResetCounters();

	CHECK(module.GetRefCount() == 0);
	module.AddRef();
	CHECK(module.GetRefCount() == 1);
	module.AddRef();
	CHECK(module.GetRefCount() == 2);
	module.ReleaseRef();
	CHECK(module.GetRefCount() == 1);
	module.ReleaseRef();
	CHECK(module.GetRefCount() == 0);
	module.ReleaseRef(); // Ochrona przed underflow
	CHECK(module.GetRefCount() == 0);

	CHECK(module.GetTransactionCount() == 0);
}

TEST_CASE("WgpuModule - Thread safety of access methods")
{
	CWgpuModule& module = CWgpuModule::Instance();
	module.FreeLibrary();
	module.ResetCounters();

	constexpr int NUM_THREADS = 16;
	constexpr int ITERATIONS_PER_THREAD = 200;

	std::vector<std::thread> threads;
	threads.reserve(NUM_THREADS);

	std::atomic<bool> startFlag{ false };
	std::atomic<int> completedThreads{ 0 };

	const uint8_t dummyData[16] = { 0 };

	for (int t = 0; t < NUM_THREADS; ++t)
	{
		threads.emplace_back([&module, &startFlag, &completedThreads, t, &dummyData]() {
			while (!startFlag.load(std::memory_order_relaxed))
			{
				std::this_thread::yield();
			}

			TRenderFramePacket packet{};
			for (int i = 0; i < ITERATIONS_PER_THREAD; ++i)
			{
				// Rownolegly odczyt stanu
				bool loaded = module.IsLoaded();
				(void)loaded;

				HMODULE hMod = module.GetHandle();
				(void)hMod;

				// Rownolegle no-op wyrejestrowywanie i rejestrowanie
				uintptr_t dummyId = static_cast<uintptr_t>((t + 1) * 1000 + i);
				module.UnregisterTexture(dummyId);
				module.RegisterTexture(dummyId, 4, 4, 1, dummyData, sizeof(dummyData));

				// Rownolegle transakcje licznikowe
				module.AddRef();
				module.GetRefCount();
				module.ReleaseRef();

				// Rownolegly render & resize & destroy no-op
				module.Resize(800 + i, 600 + i);
				module.RenderFrame(&packet);

				if (i % 50 == 0)
				{
					module.FreeLibrary();
					module.Load("nonexistent_concurrent_test.dll");
				}
			}

			completedThreads.fetch_add(1, std::memory_order_relaxed);
		});
	}

	startFlag.store(true, std::memory_order_release);

	for (auto& th : threads)
	{
		if (th.joinable())
		{
			th.join();
		}
	}

	CHECK(completedThreads.load() == NUM_THREADS);
	CHECK_FALSE(module.IsLoaded());
}

TEST_CASE("WgpuModule - Real DLL load & lifecycle verification")
{
	CWgpuModule& module = CWgpuModule::Instance();
	module.FreeLibrary();
	module.ResetCounters();

	bool loaded = module.Load("m2_render_wgpu.dll");
	if (loaded)
	{
		CHECK(module.IsLoaded());
		CHECK(module.GetHandle() != nullptr);
		CHECK(module.GetFunctions().initialize != nullptr);
		CHECK(module.GetFunctions().render_frame != nullptr);
		CHECK(module.GetFunctions().register_texture != nullptr);
		CHECK(module.GetFunctions().unregister_texture != nullptr);
		CHECK(module.GetFunctions().resize != nullptr);
		CHECK(module.GetFunctions().destroy != nullptr);

		// Wywolanie initialize w trybie headless (HWND = nullptr)
		bool initOk = module.Initialize(nullptr, 64, 64);
		CHECK(initOk);

		module.Resize(128, 128);

		TRenderFramePacket dummyPacket{};
		module.RenderFrame(&dummyPacket);

		const uint8_t dummyTex[64] = { 0 };
		bool regOk = module.RegisterTexture(2001, 4, 4, 1, dummyTex, sizeof(dummyTex));
		CHECK(regOk);

		module.UnregisterTexture(2001);

		module.Destroy();
		// Czyszczenie zasobow graficznych przez Destroy()
		CHECK(module.IsLoaded());

		module.FreeLibrary();
		CHECK_FALSE(module.IsLoaded());
	}
}


