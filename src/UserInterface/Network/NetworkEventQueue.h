#pragma once

#include <cstdint>
#include <cstring>
#include <span>
#include <atomic>
#include <optional>
#include <array>
#include <vector>
#include <cassert>

namespace UserInterface::Network
{
	struct QueuedPacketHeader
	{
		uint16_t opcode;          // Zunifikowany lub natywny naglowek pakietu
		uint16_t payloadLength;   // Calkowity rozmiar ramki pakietu (wraz z naglowkiem)
		uint32_t recvTimestampMs; // Czas odbioru w NetworkPump
	};

	struct PacketFrameView
	{
		QueuedPacketHeader meta;
		std::span<const uint8_t> data;
	};

	// Bezalokacyjny Ring-Buffer SPSC dla zdekodowanych ramek TCP (Arena 512 KB)
	template <size_t CapacityBytes = 512 * 1024>
	class SpscPacketRingBuffer
	{
		static_assert((CapacityBytes & (CapacityBytes - 1)) == 0, "CapacityBytes must be a power of 2");

	public:
		SpscPacketRingBuffer() noexcept
			: m_writePos(0), m_readPos(0)
		{
		}

		void Clear() noexcept
		{
			m_writePos.store(0, std::memory_order_relaxed);
			m_readPos.store(0, std::memory_order_relaxed);
		}

		// Wywolywane wylacznie w fazie NetworkPump (Producent)
		[[nodiscard]] bool TryEnqueue(uint16_t opcode, std::span<const uint8_t> packetBytes, uint32_t timestampMs) noexcept
		{
			if (packetBytes.empty() || packetBytes.size() > 65000)
				return false;

			const size_t totalEntrySize = AlignUp(sizeof(QueuedPacketHeader) + packetBytes.size());
			const size_t writePos = m_writePos.load(std::memory_order_relaxed);
			const size_t readPos = m_readPos.load(std::memory_order_acquire);

			const size_t usedBytes = writePos - readPos;
			if (CapacityBytes - usedBytes < totalEntrySize + sizeof(QueuedPacketHeader))
				return false; // Bufor pelny

			const size_t offset = writePos & (CapacityBytes - 1);
			const size_t bytesToEnd = CapacityBytes - offset;

			QueuedPacketHeader hdr{ opcode, static_cast<uint16_t>(packetBytes.size()), timestampMs };

			if (bytesToEnd >= sizeof(QueuedPacketHeader))
			{
				std::memcpy(&m_buffer[offset], &hdr, sizeof(QueuedPacketHeader));
				const size_t dataOffset = (offset + sizeof(QueuedPacketHeader)) & (CapacityBytes - 1);
				const size_t dataBytesToEnd = CapacityBytes - dataOffset;

				if (dataBytesToEnd >= packetBytes.size())
				{
					std::memcpy(&m_buffer[dataOffset], packetBytes.data(), packetBytes.size());
				}
				else
				{
					std::memcpy(&m_buffer[dataOffset], packetBytes.data(), dataBytesToEnd);
					std::memcpy(&m_buffer[0], packetBytes.data() + dataBytesToEnd, packetBytes.size() - dataBytesToEnd);
				}
			}
			else
			{
				const uint8_t* pHdr = reinterpret_cast<const uint8_t*>(&hdr);
				std::memcpy(&m_buffer[offset], pHdr, bytesToEnd);
				std::memcpy(&m_buffer[0], pHdr + bytesToEnd, sizeof(QueuedPacketHeader) - bytesToEnd);

				const size_t dataOffset = sizeof(QueuedPacketHeader) - bytesToEnd;
				std::memcpy(&m_buffer[dataOffset], packetBytes.data(), packetBytes.size());
			}

			m_writePos.store(writePos + totalEntrySize, std::memory_order_release);
			return true;
		}

		// Podglad nastepnego pakietu dla konsumenta
		[[nodiscard]] bool HasPacket() const noexcept
		{
			return m_readPos.load(std::memory_order_acquire) < m_writePos.load(std::memory_order_acquire);
		}

		// Wywolywane wylacznie w FixedUpdate (Konsument)
		template <typename HandlerFn>
		size_t DrainAll(HandlerFn&& handler)
		{
			size_t processedCount = 0;
			const size_t writePos = m_writePos.load(std::memory_order_acquire);
			size_t readPos = m_readPos.load(std::memory_order_relaxed);

			while (readPos < writePos)
			{
				const size_t offset = readPos & (CapacityBytes - 1);
				QueuedPacketHeader hdr{};

				if (CapacityBytes - offset >= sizeof(QueuedPacketHeader))
				{
					std::memcpy(&hdr, &m_buffer[offset], sizeof(QueuedPacketHeader));
				}
				else
				{
					uint8_t* pHdr = reinterpret_cast<uint8_t*>(&hdr);
					const size_t firstPart = CapacityBytes - offset;
					std::memcpy(pHdr, &m_buffer[offset], firstPart);
					std::memcpy(pHdr + firstPart, &m_buffer[0], sizeof(QueuedPacketHeader) - firstPart);
				}

				alignas(16) std::array<uint8_t, 4096> stackBuffer;
				std::span<const uint8_t> frameData;
				std::vector<uint8_t> heapBuffer;

				const size_t dataOffset = (offset + sizeof(QueuedPacketHeader)) & (CapacityBytes - 1);
				if (hdr.payloadLength <= stackBuffer.size())
				{
					if (CapacityBytes - dataOffset >= hdr.payloadLength)
					{
						std::memcpy(stackBuffer.data(), &m_buffer[dataOffset], hdr.payloadLength);
					}
					else
					{
						const size_t firstPart = CapacityBytes - dataOffset;
						std::memcpy(stackBuffer.data(), &m_buffer[dataOffset], firstPart);
						std::memcpy(stackBuffer.data() + firstPart, &m_buffer[0], hdr.payloadLength - firstPart);
					}
					frameData = std::span<const uint8_t>(stackBuffer.data(), hdr.payloadLength);
				}
				else
				{
					heapBuffer.resize(hdr.payloadLength);
					if (CapacityBytes - dataOffset >= hdr.payloadLength)
					{
						std::memcpy(heapBuffer.data(), &m_buffer[dataOffset], hdr.payloadLength);
					}
					else
					{
						const size_t firstPart = CapacityBytes - dataOffset;
						std::memcpy(heapBuffer.data(), &m_buffer[dataOffset], firstPart);
						std::memcpy(heapBuffer.data() + firstPart, &m_buffer[0], hdr.payloadLength - firstPart);
					}
					frameData = std::span<const uint8_t>(heapBuffer.data(), hdr.payloadLength);
				}

				bool shouldContinue = handler(PacketFrameView{ hdr, frameData });

				const size_t totalEntrySize = AlignUp(sizeof(QueuedPacketHeader) + hdr.payloadLength);
				readPos += totalEntrySize;
				++processedCount;

				if (!shouldContinue)
					break;
			}

			m_readPos.store(readPos, std::memory_order_release);
			return processedCount;
		}

	private:
		static constexpr size_t AlignUp(size_t val) noexcept
		{
			return (val + 15) & ~size_t(15);
		}

		alignas(64) std::atomic<size_t> m_writePos{ 0 };
		alignas(64) std::atomic<size_t> m_readPos{ 0 };
		alignas(64) std::array<uint8_t, CapacityBytes> m_buffer{};
	};
} // namespace UserInterface::Network
