#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Vegetation/SpeedTreeInstanceBufferManager.h"
#include <memory>
#include <vector>

using namespace Client::Vegetation;

class MockHardwareBuffer : public IHardwareBuffer
{
public:
    MockHardwareBuffer(size_t capacity)
        : m_capacity(capacity), m_updateCalled(false)
    {
        m_data.resize(capacity * GetStride());
    }

    bool UpdateData(std::span<const uint8_t> data, uint32_t offset = 0) override
    {
        if (offset + data.size() > m_data.size())
        {
            return false;
        }
        std::copy(data.begin(), data.end(), m_data.begin() + offset);
        m_updateCalled = true;
        m_lastUpdateSize = data.size();
        return m_shouldSucceed;
    }

    size_t GetSize() const noexcept override
    {
        return m_data.size();
    }

    uint32_t GetStride() const noexcept override
    {
        return sizeof(SpeedTreeInstanceData);
    }

    void SetUpdateSuccess(bool success)
    {
        m_shouldSucceed = success;
    }

    bool WasUpdateCalled() const
    {
        return m_updateCalled;
    }

    size_t GetLastUpdateSize() const
    {
        return m_lastUpdateSize;
    }

private:
    size_t m_capacity;
    std::vector<uint8_t> m_data;
    bool m_updateCalled = false;
    bool m_shouldSucceed = true;
    size_t m_lastUpdateSize = 0;
};

TEST_SUITE("SpeedTreeInstanceBufferManager")
{
    TEST_CASE("Initialization sets correct capacity")
    {
        auto buffer = std::make_unique<MockHardwareBuffer>(100);
        SpeedTreeInstanceBufferManager manager(std::move(buffer));

        CHECK(manager.GetMaxCapacity() == 100);
        CHECK(manager.GetInstanceCount() == 0);
    }

    TEST_CASE("AddInstance adds instances successfully until capacity is reached")
    {
        auto buffer = std::make_unique<MockHardwareBuffer>(2);
        SpeedTreeInstanceBufferManager manager(std::move(buffer));

        SpeedTreeInstanceData data1{};
        data1.scale = 1.0f;
        auto result1 = manager.AddInstance(data1);
        CHECK(result1.has_value());
        CHECK(manager.GetInstanceCount() == 1);

        SpeedTreeInstanceData data2{};
        data2.scale = 2.0f;
        auto result2 = manager.AddInstance(data2);
        CHECK(result2.has_value());
        CHECK(manager.GetInstanceCount() == 2);

        SpeedTreeInstanceData data3{};
        data3.scale = 3.0f;
        auto result3 = manager.AddInstance(data3);
        CHECK_FALSE(result3.has_value());
        CHECK(result3.error() == SpeedTreeInstanceError::BufferTooSmall);
        CHECK(manager.GetInstanceCount() == 2);
    }

    TEST_CASE("ClearInstances resets instance count")
    {
        auto buffer = std::make_unique<MockHardwareBuffer>(10);
        SpeedTreeInstanceBufferManager manager(std::move(buffer));

        SpeedTreeInstanceData data{};
        auto _ = manager.AddInstance(data);
        _ = manager.AddInstance(data);

        CHECK(manager.GetInstanceCount() == 2);
        manager.ClearInstances();
        CHECK(manager.GetInstanceCount() == 0);
    }

    TEST_CASE("FlushToHardware flushes data and reports errors correctly")
    {
        auto mockBufferRaw = new MockHardwareBuffer(5);
        auto buffer = std::unique_ptr<MockHardwareBuffer>(mockBufferRaw);
        SpeedTreeInstanceBufferManager manager(std::move(buffer));

        SpeedTreeInstanceData data{};
        auto _ = manager.AddInstance(data);
        _ = manager.AddInstance(data);

        SUBCASE("Successful flush")
        {
            auto result = manager.FlushToHardware();
            CHECK(result.has_value());
            CHECK(mockBufferRaw->WasUpdateCalled());
            CHECK(mockBufferRaw->GetLastUpdateSize() == 2 * sizeof(SpeedTreeInstanceData));
        }

        SUBCASE("Failed flush")
        {
            mockBufferRaw->SetUpdateSuccess(false);
            auto result = manager.FlushToHardware();
            CHECK_FALSE(result.has_value());
            CHECK(result.error() == SpeedTreeInstanceError::UpdateFailed);
        }

        SUBCASE("Empty flush does nothing")
        {
            manager.ClearInstances();
            auto result = manager.FlushToHardware();
            CHECK(result.has_value());
            CHECK_FALSE(mockBufferRaw->WasUpdateCalled());
        }
    }

    TEST_CASE("Error messages are correct and contain no diacritics")
    {
        CHECK(to_string(SpeedTreeInstanceError::None) == "Brak bledu");
        CHECK(to_string(SpeedTreeInstanceError::BufferTooSmall) == "Bufor jest zbyt maly aby pomiescic nowe instancje");
        CHECK(to_string(SpeedTreeInstanceError::UpdateFailed) == "Aktualizacja bufora sprzetowego nie powiodla sie");
        CHECK(to_string(SpeedTreeInstanceError::InvalidInstanceData) == "Nieprawidlowe dane instancji");
    }

    TEST_CASE("Constructor throws on invalid arguments")
    {
        SUBCASE("Null buffer")
        {
            CHECK_THROWS_AS(SpeedTreeInstanceBufferManager(nullptr), std::invalid_argument);
        }

        SUBCASE("Invalid stride")
        {
            class InvalidStrideBuffer : public IHardwareBuffer
            {
            public:
                bool UpdateData(std::span<const uint8_t>, uint32_t) override { return true; }
                size_t GetSize() const noexcept override { return 100; }
                uint32_t GetStride() const noexcept override { return 10; } // Not equal to sizeof(SpeedTreeInstanceData)
            };

            auto buffer = std::make_unique<InvalidStrideBuffer>();
            CHECK_THROWS_AS(SpeedTreeInstanceBufferManager(std::move(buffer)), std::invalid_argument);
        }
    }
}
