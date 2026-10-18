#include "BufferPool.h"
#include <algorithm>

CBufferPool::CBufferPool()
	: m_totalAllocated(0)
	, m_totalPooledMemory(0)
{
}

CBufferPool::~CBufferPool()
{
	Clear();
}

std::vector<uint8_t> CBufferPool::Acquire(size_t minSize)
{
	std::vector<uint8_t> result;
	
	{
		std::lock_guard<std::mutex> lock(m_mutex);

		// m_pool is maintained in sorted order by capacity.
		// Use lower_bound to find the first buffer that is large enough.
		auto it = std::lower_bound(m_pool.begin(), m_pool.end(), minSize,
			[](const TPooledBuffer& a, size_t val) {
				return a.capacity < val;
			});

		if (it != m_pool.end())
		{
			result = std::move(it->buffer);
			m_totalPooledMemory -= it->capacity;
			m_pool.erase(it);
		}
		else
		{
			m_totalAllocated++;
		}
	}

	if (result.capacity() >= minSize)
	{
		result.clear();
		return result;
	}

	std::vector<uint8_t> newBuffer;
	newBuffer.reserve(minSize);
	return newBuffer;
}

void CBufferPool::Release(std::vector<uint8_t>&& buffer)
{
	size_t capacity = buffer.capacity();

	if (capacity == 0 || capacity > MAX_BUFFER_SIZE)
	{
		// Destructor of 'buffer' runs here, outside the lock, freeing memory.
		return;
	}

	TPooledBuffer newPooledBuf(std::move(buffer));
	std::vector<uint8_t> toDiscard;

	{
		std::lock_guard<std::mutex> lock(m_mutex);

		auto it = std::lower_bound(m_pool.begin(), m_pool.end(), capacity,
			[](const TPooledBuffer& a, size_t val) {
				return a.capacity < val;
			});

		if (m_pool.size() >= MAX_POOL_SIZE)
		{
			// Since pool is sorted, the smallest is at m_pool.front().
			if (capacity <= m_pool.front().capacity)
			{
				// New buffer is smaller or equal to the smallest, discard it.
				// By swapping, we delay destruction until after unlocking.
				toDiscard = std::move(newPooledBuf.buffer);
				return;
			}
			
			// Discard the smallest and insert the new one.
			toDiscard = std::move(m_pool.front().buffer);
			m_totalPooledMemory -= m_pool.front().capacity;
			m_pool.erase(m_pool.begin());
			
			// Need to re-calculate iterator because erase invalidated it.
			it = std::lower_bound(m_pool.begin(), m_pool.end(), capacity,
				[](const TPooledBuffer& a, size_t val) {
					return a.capacity < val;
				});
		}

		m_totalPooledMemory += capacity;
		m_pool.insert(it, std::move(newPooledBuf));
	}
	// 'toDiscard' and 'newPooledBuf' memory freed here, outside lock.
}

size_t CBufferPool::GetPoolSize() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_pool.size();
}

size_t CBufferPool::GetTotalAllocated() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_totalAllocated;
}

size_t CBufferPool::GetTotalMemoryPooled() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_totalPooledMemory;
}

void CBufferPool::Clear()
{
	std::vector<TPooledBuffer> oldPool;
	{
		std::lock_guard<std::mutex> lock(m_mutex);
		oldPool = std::move(m_pool);
		m_totalPooledMemory = 0;
	}
	// Memory for oldPool elements freed here, outside lock.
}
