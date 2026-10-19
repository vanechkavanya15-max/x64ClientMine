#include "TextureSet.h"
#include <cmath>
#include <fstream>
#include <sstream>
#include <charconv>

namespace Client::Terrain {

TextureSet::TextureSet() {
    Initialize();
}

void TextureSet::Initialize() {
    Clear();
}

void TextureSet::Clear() {
    m_Textures.clear();
    m_stFileName.clear();
    AddEmptyTexture(); // index 0 is eraser
}

void TextureSet::UpdateTransform(TerrainTexture& tex, float terrainTexCoordBase) {
    if (terrainTexCoordBase == 0.0f) {
        return;
    }
    
    float su = terrainTexCoordBase * tex.uScale;
    float sv = -terrainTexCoordBase * tex.vScale;

    tex.transform = Matrix4x4{{
        su, 0.0f, 0.0f, 0.0f,
        0.0f, sv, 0.0f, 0.0f,
        0.0f, 0.0f, 1.0f, 0.0f,
        tex.uOffset, -tex.vOffset, 0.0f, 1.0f
    }};
}

std::expected<void, TextureSetError> TextureSet::Load(std::string_view fileName, float terrainTexCoordBase) {
    if (fileName.empty()) {
        return std::unexpected(TextureSetError::LoadFailed);
    }

    std::string fileNameStr(fileName);
    std::ifstream file(fileNameStr);
    if (!file.is_open()) {
        return std::unexpected(TextureSetError::LoadFailed);
    }

    Clear();
    m_stFileName = fileName;

    std::string line;
    bool inTexture = false;
    TerrainTexture currentTexture;
    int tokenIndex = 0;
    
    while (std::getline(file, line)) {
        if (line.empty()) continue;
        
        std::istringstream iss(line);
        std::string token;
        iss >> token;

        if (token == "Start" || token.starts_with("TextureCount") || token == "TextureSet") {
            if (token == "Start") {
                inTexture = true;
                tokenIndex = 0;
            }
            continue;
        }

        if (token == "End") {
            if (inTexture) {
                UpdateTransform(currentTexture, terrainTexCoordBase);
                m_Textures.push_back(std::move(currentTexture));
                currentTexture = TerrainTexture{};
                inTexture = false;
            }
            continue;
        }

        if (inTexture) {
            // Unquote filename
            if (tokenIndex == 0) {
                size_t first = line.find_first_of('\"');
                size_t last = line.find_last_of('\"');
                if (first != std::string::npos && last != std::string::npos && first != last) {
                    currentTexture.filename = line.substr(first + 1, last - first - 1);
                } else {
                    currentTexture.filename = token;
                }
            } else if (tokenIndex == 1) {
                currentTexture.uScale = std::stof(token);
            } else if (tokenIndex == 2) {
                currentTexture.vScale = std::stof(token);
            } else if (tokenIndex == 3) {
                currentTexture.uOffset = std::stof(token);
            } else if (tokenIndex == 4) {
                currentTexture.vOffset = std::stof(token);
            } else if (tokenIndex == 5) {
                currentTexture.bSplat = std::stoi(token) != 0;
            } else if (tokenIndex == 6) {
                currentTexture.begin = static_cast<uint16_t>(std::stoi(token));
            } else if (tokenIndex == 7) {
                currentTexture.end = static_cast<uint16_t>(std::stoi(token));
            }
            tokenIndex++;
        }
    }
    
    return {};
}

std::expected<void, TextureSetError> TextureSet::Save(std::string_view fileName) {
    if (fileName.empty()) {
        return std::unexpected(TextureSetError::SaveFailed);
    }

    std::string fileNameStr(fileName);
    std::ofstream file(fileNameStr);
    if (!file.is_open()) {
        return std::unexpected(TextureSetError::SaveFailed);
    }

    file << "TextureSet\n\n";
    
    size_t count = GetTextureCount();
    size_t saveCount = count > 0 ? count - 1 : 0;
    
    file << "TextureCount " << saveCount << "\n\n";

    for (size_t i = 1; i < count; ++i) {
        const auto& rTex = m_Textures[i];
        
        file << "Start Texture" << (i < 100 ? (i < 10 ? "00" : "0") : "") << i << "\n";
        file << "    \"" << rTex.filename << "\"\n";
        file << "    " << rTex.uScale << "\n";
        file << "    " << rTex.vScale << "\n";
        file << "    " << rTex.uOffset << "\n";
        file << "    " << rTex.vOffset << "\n";
        file << "    " << (rTex.bSplat ? 1 : 0) << "\n";
        file << "    " << rTex.begin << "\n";
        file << "    " << rTex.end << "\n";
        file << "End Texture" << (i < 100 ? (i < 10 ? "00" : "0") : "") << i << "\n";
    }

    return {};
}

size_t TextureSet::GetTextureCount() const {
    return m_Textures.size();
}

std::expected<std::reference_wrapper<TerrainTexture>, TextureSetError> TextureSet::GetTexture(size_t index) {
    if (index >= m_Textures.size()) {
        return std::unexpected(TextureSetError::InvalidIndex);
    }
    return std::ref(m_Textures[index]);
}

std::expected<std::reference_wrapper<const TerrainTexture>, TextureSetError> TextureSet::GetTexture(size_t index) const {
    if (index >= m_Textures.size()) {
        return std::unexpected(TextureSetError::InvalidIndex);
    }
    return std::cref(m_Textures[index]);
}

std::expected<void, TextureSetError> TextureSet::RemoveTexture(size_t index) {
    if (index == 0 || index >= m_Textures.size()) { // Cannot remove eraser
        return std::unexpected(TextureSetError::InvalidIndex);
    }
    m_Textures.erase(m_Textures.begin() + index);
    return {};
}

std::expected<void, TextureSetError> TextureSet::SetTexture(size_t index,
                           std::string_view fileName,
                           float uScale, float vScale,
                           float uOffset, float vOffset,
                           bool bSplat,
                           uint16_t begin, uint16_t end,
                           float terrainTexCoordBase) {
    if (index >= m_Textures.size()) {
        return std::unexpected(TextureSetError::InvalidIndex);
    }

    if (fileName.empty()) {
        return std::unexpected(TextureSetError::EmptyTextureName);
    }

    auto& tex = m_Textures[index];
    tex.filename = fileName;
    tex.uScale = uScale;
    tex.vScale = vScale;
    tex.uOffset = uOffset;
    tex.vOffset = vOffset;
    tex.bSplat = bSplat;
    tex.begin = begin;
    tex.end = end;

    UpdateTransform(tex, terrainTexCoordBase);

    return {};
}

void TextureSet::Reload(float terrainTexCoordBase) {
    for (size_t i = 1; i < m_Textures.size(); ++i) {
        UpdateTransform(m_Textures[i], terrainTexCoordBase);
    }
}

std::expected<void, TextureSetError> TextureSet::AddTexture(std::string_view fileName,
                           float uScale, float vScale,
                           float uOffset, float vOffset,
                           bool bSplat,
                           uint16_t begin, uint16_t end,
                           float terrainTexCoordBase) {
    
    if (fileName.empty()) {
        return std::unexpected(TextureSetError::EmptyTextureName);
    }

    if (GetTextureCount() >= MAXTERRAINTEXTURES) {
        return std::unexpected(TextureSetError::InvalidIndex);
    }

    TerrainTexture tex;
    tex.filename = fileName;
    tex.uScale = uScale;
    tex.vScale = vScale;
    tex.uOffset = uOffset;
    tex.vOffset = vOffset;
    tex.bSplat = bSplat;
    tex.begin = begin;
    tex.end = end;

    UpdateTransform(tex, terrainTexCoordBase);
    m_Textures.push_back(std::move(tex));
    
    return {};
}

void TextureSet::AddEmptyTexture() {
    m_Textures.emplace_back(); // Slot 0
}

} // namespace Client::Terrain
