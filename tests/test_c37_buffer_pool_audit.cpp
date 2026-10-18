#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>

#include "EterLib/BufferPool.h"

#include <thread>
#include <vector>
#include <atomic>
#include <random>
#include <iostream>

TEST_CASE("BufferPool - Basic Acquire and Release")
{
	CBufferPool pool;

	SUBCASE("Acquire minimum size creates a correct buffer")
	{
		auto buf = pool.Acquire(1024);
		CHECK(buf.capacity() >= 1024);
		CHECK(buf.empty());
	}

	SUBCASE("Release stores the buffer")
	{
		auto buf = pool.Acquire(1024);
		size_t cap = buf.capacity();
		pool.Release(std::move(buf));

		CHECK(pool.GetPoolSize() == 1);
		CHECK(pool.GetTotalMemoryPooled() == cap);
	}

	SUBCASE("Acquire reuses an existing buffer")
	{
		auto buf1 = pool.Acquire(1024);
		pool.Release(std::move(buf1));

		auto buf2 = pool.Acquire(512);
		CHECK(buf2.capacity() >= 1024); // Reused the 1024+ one
		CHECK(pool.GetPoolSize() == 0);
		CHECK(pool.GetTotalMemoryPooled() == 0);
	}
}

TEST_CASE("BufferPool - Sorting and Lower Bound Logic")
{
	CBufferPool pool;

	std::vector<uint8_t> b1; b1.reserve(100);
	std::vector<uint8_t> b2; b2.reserve(200);
	std::vector<uint8_t> b3; b3.reserve(300);

	pool.Release(std::move(b3));
	pool.Release(std::move(b1));
	pool.Release(std::move(b2));

	CHECK(pool.GetPoolSize() == 3);

	SUBCASE("Acquire exact or slightly smaller size grabs the smallest valid buffer")
	{
		auto b = pool.Acquire(150);
		// Should grab the 200 one
		CHECK(b.capacity() >= 200);
		CHECK(b.capacity() < 300);
		CHECK(pool.GetPoolSize() == 2);
	}

	SUBCASE("Acquire larger size than pool creates new")
	{
		auto b = pool.Acquire(500);
		CHECK(b.capacity() >= 500);
		CHECK(pool.GetPoolSize() == 3);
	}
}

TEST_CASE("BufferPool - Max Pool Size Limitations")
{
	CBufferPool pool;

	// Fill pool exactly to MAX_POOL_SIZE
	for (size_t i = 0; i < 64; ++i)
	{
		std::vector<uint8_t> b;
		b.reserve((i + 1) * 10); // sizes: 10, 20, 30... 640
		pool.Release(std::move(b));
	}

	CHECK(pool.GetPoolSize() == 64);

	SUBCASE("Releasing smaller buffer when full discards it")
	{
		std::vector<uint8_t> smallB;
		smallB.reserve(5);
		pool.Release(std::move(smallB));
		
		CHECK(pool.GetPoolSize() == 64);
		// The 5-byte buffer should have been discarded because front is 10
		auto b = pool.Acquire(640);
		CHECK(b.capacity() >= 640); // Proves other buffers still there
	}

	SUBCASE("Releasing larger buffer replaces the smallest")
	{
		std::vector<uint8_t> largeB;
		largeB.reserve(1000);
		pool.Release(std::move(largeB));

		CHECK(pool.GetPoolSize() == 64);
		
		auto smallestCheck = pool.Acquire(15);
		// The original '10' size is gone, so 15 will actually get the '20' buffer
		CHECK(smallestCheck.capacity() >= 20);
	}
}

TEST_CASE("BufferPool - Edge Cases and Clear")
{
	CBufferPool pool;

	SUBCASE("Zero capacity buffer does not enter pool")
	{
		std::vector<uint8_t> emptyBuf;
		pool.Release(std::move(emptyBuf));
		CHECK(pool.GetPoolSize() == 0);
	}

	SUBCASE("Clear wipes everything")
	{
		std::vector<uint8_t> b; b.reserve(100);
		pool.Release(std::move(b));
		CHECK(pool.GetPoolSize() == 1);
		
		pool.Clear();
		CHECK(pool.GetPoolSize() == 0);
		CHECK(pool.GetTotalMemoryPooled() == 0);
	}
}

TEST_CASE("BufferPool - Thread Safety")
{
	CBufferPool pool;
	std::atomic<bool> start{false};
	const int num_threads = 16;
	const int iterations = 1000;

	auto worker = [&]() {
		while (!start) { std::this_thread::yield(); }

		std::random_device rd;
		std::mt19937 gen(rd());
		std::uniform_int_distribution<size_t> dist(10, 5000);

		for (int i = 0; i < iterations; ++i)
		{
			size_t size = dist(gen);
			auto buf = pool.Acquire(size);
			buf.resize(size);
			
			// simulate some tiny work
			if (i % 10 == 0) std::this_thread::yield();

			buf.clear();
			pool.Release(std::move(buf));
		}
	};

	std::vector<std::thread> threads;
	for (int i = 0; i < num_threads; ++i)
	{
		threads.emplace_back(worker);
	}

	start = true;

	for (auto& t : threads)
	{
		t.join();
	}

	CHECK(pool.GetPoolSize() <= 64); // Because MAX_POOL_SIZE is 64
	// Total memory should be consistent with sizes
	CHECK(pool.GetTotalMemoryPooled() > 0); 
}
