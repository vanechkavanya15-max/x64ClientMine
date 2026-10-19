#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Terrain/TextureSet.h"
#include "Client/Terrain/TerrainTypes.h"
#include <string>
#include <type_traits>

namespace doctest {
    template<> struct StringMaker<Client::Terrain::TextureSetError> {
        static String convert(const Client::Terrain::TextureSetError& err) {
            switch(err) {
                case Client::Terrain::TextureSetError::InvalidIndex: return "InvalidIndex";
                case Client::Terrain::TextureSetError::EmptyTextureName: return "EmptyTextureName";
                case Client::Terrain::TextureSetError::TextureNotFound: return "TextureNotFound";
                case Client::Terrain::TextureSetError::LoadFailed: return "LoadFailed";
                case Client::Terrain::TextureSetError::SaveFailed: return "SaveFailed";
                default: return "Unknown";
            }
        }
    };
}

using namespace Client::Terrain;

TEST_SUITE("TerrainTypes POD Requirements") {
    TEST_CASE("POD Types verification") {
        CHECK(std::is_standard_layout_v<TerrainSplat>);
        CHECK(std::is_standard_layout_v<TerrainSplatPatch>);
        CHECK(std::is_standard_layout_v<TerrainVBuffer>);
        CHECK(std::is_standard_layout_v<TerrainMaterial>);
        CHECK(std::is_standard_layout_v<TerrainGlobals>);
        CHECK(std::is_standard_layout_v<Matrix4x4>);
    }

    TEST_CASE("Initial values verification") {
        TerrainGlobals globals;
        CHECK_EQ(globals.pageUVLength, 0.0f);
        CHECK_EQ(globals.squaresPerTexture, 0);
        
        Matrix4x4 identity = Matrix4x4::Identity();
        CHECK_EQ(identity.m[0], 1.0f);
        CHECK_EQ(identity.m[15], 1.0f);
    }
}

TEST_SUITE("TextureSet Manager operations") {
    TEST_CASE("Add and Get Texture") {
        TextureSet set;
        CHECK_EQ(set.GetTextureCount(), 1u); // Eraser

        auto result = set.AddTexture("texture1.png", 1.0f, 1.0f, 0.0f, 0.0f, true, 0, 100, 1.0f);
        CHECK(result.has_value());
        CHECK_EQ(set.GetTextureCount(), 2u);

        auto texResult = set.GetTexture(1);
        CHECK(texResult.has_value());
        CHECK_EQ(texResult.value().get().filename, "texture1.png");
        CHECK_EQ(texResult.value().get().bSplat, true);
    }

    TEST_CASE("Remove Texture") {
        TextureSet set;
        set.AddTexture("texture1.png", 1.0f, 1.0f, 0.0f, 0.0f, true, 0, 100, 1.0f);
        CHECK_EQ(set.GetTextureCount(), 2u);

        auto removeResult = set.RemoveTexture(1);
        CHECK(removeResult.has_value());
        CHECK_EQ(set.GetTextureCount(), 1u); // Eraser remains
    }

    TEST_CASE("Set Texture") {
        TextureSet set;
        set.AddTexture("texture1.png", 1.0f, 1.0f, 0.0f, 0.0f, true, 0, 100, 1.0f);

        auto setResult = set.SetTexture(1, "texture2.png", 2.0f, 2.0f, 0.1f, 0.1f, false, 10, 50, 2.0f);
        CHECK(setResult.has_value());

        auto texResult = set.GetTexture(1);
        CHECK(texResult.has_value());
        CHECK_EQ(texResult.value().get().filename, "texture2.png");
        CHECK_EQ(texResult.value().get().uScale, 2.0f);
        CHECK_EQ(texResult.value().get().bSplat, false);
    }

    TEST_CASE("Error handling") {
        TextureSet set;

        SUBCASE("Invalid index on Get") {
            auto texResult = set.GetTexture(1); // 0 is eraser, 1 is out of bounds
            CHECK_FALSE(texResult.has_value());
            CHECK_EQ(texResult.error(), TextureSetError::InvalidIndex);
        }

        SUBCASE("Invalid index on Remove (Eraser)") {
            auto removeResult = set.RemoveTexture(0);
            CHECK_FALSE(removeResult.has_value());
            CHECK_EQ(removeResult.error(), TextureSetError::InvalidIndex);
        }

        SUBCASE("Empty filename on Add") {
            auto addResult = set.AddTexture("", 1.0f, 1.0f, 0.0f, 0.0f, true, 0, 100, 1.0f);
            CHECK_FALSE(addResult.has_value());
            CHECK_EQ(addResult.error(), TextureSetError::EmptyTextureName);
        }

        SUBCASE("Invalid index on Set") {
            auto setResult = set.SetTexture(1, "test.png", 1.0f, 1.0f, 0.0f, 0.0f, true, 0, 100, 1.0f);
            CHECK_FALSE(setResult.has_value());
            CHECK_EQ(setResult.error(), TextureSetError::InvalidIndex);
        }
        
        SUBCASE("Empty filename on Load") {
            auto loadResult = set.Load("", 1.0f);
            CHECK_FALSE(loadResult.has_value());
            CHECK_EQ(loadResult.error(), TextureSetError::LoadFailed);
        }
        
        SUBCASE("Empty filename on Save") {
            auto saveResult = set.Save("");
            CHECK_FALSE(saveResult.has_value());
            CHECK_EQ(saveResult.error(), TextureSetError::SaveFailed);
        }
    }
}
