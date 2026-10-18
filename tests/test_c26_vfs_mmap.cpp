#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include "../src/Client/Core/VfsMemoryMappedBuffer.h"
#include <fstream>
#include <filesystem>
#include <string>

using namespace Client::Core;

TEST_CASE("VfsMemoryMappedBuffer tests") {
    const std::string testFilePath = "test_vfs_mmap_file.bin";
    const std::string testData = "Hello, VFS Mmap World! This is a test of memory mapped files for the C++23 client.";
    
    // Create test file
    {
        std::ofstream out(testFilePath, std::ios::binary);
        out.write(testData.c_str(), testData.size());
    }
    
    SUBCASE("Map existing file successfully") {
        auto result = VfsMemoryMappedBuffer::MapFile(testFilePath);
        REQUIRE(result.has_value());
        
        const auto& buffer = result.value();
        REQUIRE(buffer.IsMapped());
        REQUIRE(buffer.GetSize() == testData.size());
        
        auto span = buffer.GetSpan();
        REQUIRE(span.size() == testData.size());
        
        std::string mappedContent(reinterpret_cast<const char*>(span.data()), span.size());
        REQUIRE(mappedContent == testData);
    }
    
    SUBCASE("Map non-existent file fails") {
        auto result = VfsMemoryMappedBuffer::MapFile("non_existent_file_123.bin");
        REQUIRE(!result.has_value());
        REQUIRE(result.error() == VfsMemoryMappedBufferError::FileNotFound);
    }
    
    SUBCASE("Move constructor works correctly") {
        auto result = VfsMemoryMappedBuffer::MapFile(testFilePath);
        REQUIRE(result.has_value());
        
        VfsMemoryMappedBuffer buffer1 = std::move(result.value());
        REQUIRE(buffer1.IsMapped());
        REQUIRE(buffer1.GetSize() == testData.size());
        
        VfsMemoryMappedBuffer buffer2(std::move(buffer1));
        
        REQUIRE(!buffer1.IsMapped());
        REQUIRE(buffer1.GetSpan().empty());
        REQUIRE(buffer1.GetSize() == 0);
        
        REQUIRE(buffer2.IsMapped());
        REQUIRE(buffer2.GetSize() == testData.size());
        
        auto span = buffer2.GetSpan();
        REQUIRE(span.size() == testData.size());
        std::string mappedContent(reinterpret_cast<const char*>(span.data()), span.size());
        REQUIRE(mappedContent == testData);
    }

    SUBCASE("Move assignment works correctly") {
        auto result = VfsMemoryMappedBuffer::MapFile(testFilePath);
        REQUIRE(result.has_value());
        
        VfsMemoryMappedBuffer buffer1 = std::move(result.value());
        REQUIRE(buffer1.IsMapped());
        REQUIRE(buffer1.GetSize() == testData.size());
        
        VfsMemoryMappedBuffer buffer2;
        buffer2 = std::move(buffer1);
        
        REQUIRE(!buffer1.IsMapped());
        REQUIRE(buffer1.GetSpan().empty());
        REQUIRE(buffer1.GetSize() == 0);
        
        REQUIRE(buffer2.IsMapped());
        REQUIRE(buffer2.GetSize() == testData.size());
        
        auto span = buffer2.GetSpan();
        REQUIRE(span.size() == testData.size());
        std::string mappedContent(reinterpret_cast<const char*>(span.data()), span.size());
        REQUIRE(mappedContent == testData);
    }

    SUBCASE("Move assignment gracefully handles existing mapping") {
        const std::string secondTestPath = "test_vfs_mmap_file_2.bin";
        const std::string secondData = "A completely different set of test data.";
        {
            std::ofstream out(secondTestPath, std::ios::binary);
            out.write(secondData.c_str(), secondData.size());
        }

        auto result1 = VfsMemoryMappedBuffer::MapFile(testFilePath);
        REQUIRE(result1.has_value());
        
        auto result2 = VfsMemoryMappedBuffer::MapFile(secondTestPath);
        REQUIRE(result2.has_value());

        VfsMemoryMappedBuffer buffer1 = std::move(result1.value());
        VfsMemoryMappedBuffer buffer2 = std::move(result2.value());

        REQUIRE(buffer1.GetSize() == testData.size());
        REQUIRE(buffer2.GetSize() == secondData.size());

        // Move buffer2 into buffer1
        buffer1 = std::move(buffer2);

        // buffer1 should now have secondData
        REQUIRE(buffer1.IsMapped());
        REQUIRE(buffer1.GetSize() == secondData.size());
        auto span1 = buffer1.GetSpan();
        std::string mappedContent(reinterpret_cast<const char*>(span1.data()), span1.size());
        REQUIRE(mappedContent == secondData);

        // buffer2 should be empty
        REQUIRE(!buffer2.IsMapped());
        REQUIRE(buffer2.GetSize() == 0);

        buffer1.Unmap();
        std::error_code ec;
        std::filesystem::remove(secondTestPath, ec);
    }
    
    // Clean up
    std::error_code ec;
    std::filesystem::remove(testFilePath, ec);
}

TEST_CASE("VfsMemoryMappedBuffer edge cases") {
    SUBCASE("Map empty file fails with InvalidSize") {
        const std::string emptyFilePath = "test_empty_file.bin";
        {
            std::ofstream out(emptyFilePath, std::ios::binary);
        }
        
        auto result = VfsMemoryMappedBuffer::MapFile(emptyFilePath);
        REQUIRE(!result.has_value());
        REQUIRE(result.error() == VfsMemoryMappedBufferError::InvalidSize);
        
        std::error_code ec;
        std::filesystem::remove(emptyFilePath, ec);
    }
}
