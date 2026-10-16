#include "IMECompositionManager.h"
#include <algorithm>

namespace Client::Input {

void IMECompositionManager::SetCompositionString(const std::string& str, int32_t caretPos) {
    m_composition = str;
    m_caretPos = caretPos;
    
    if (m_caretPos < 0) {
        m_caretPos = 0;
    } else if (static_cast<size_t>(m_caretPos) > m_composition.length()) {
        m_caretPos = static_cast<int32_t>(m_composition.length());
    }
}

void IMECompositionManager::SetReadingString(const std::string& str) {
    m_reading = str;
}

void IMECompositionManager::SetClauses(const std::vector<uint32_t>& clauseLengths) {
    m_clauses.clear();
    m_clauses.reserve(clauseLengths.size());
    
    uint32_t currentPos = 0;
    for (uint32_t length : clauseLengths) {
        IMEClause clause{};
        clause.startPos = currentPos;
        currentPos += length;
        clause.endPos = currentPos;
        m_clauses.push_back(clause);
    }
}

Client::Core::Result<std::string, IMEError> IMECompositionManager::GetClauseText(size_t clauseIndex) const {
    if (clauseIndex >= m_clauses.size()) {
        return std::unexpected(IMEError::ClauseIndexOutOfBounds);
    }
    
    const auto& clause = m_clauses[clauseIndex];
    if (clause.startPos >= m_composition.length()) {
        return std::unexpected(IMEError::ClauseIndexOutOfBounds);
    }
    
    size_t safeEnd = std::min(static_cast<size_t>(clause.endPos), m_composition.length());
    return m_composition.substr(clause.startPos, safeEnd - clause.startPos);
}

void IMECompositionManager::SetAttributes(const std::vector<IMECompositionAttribute>& attributes) {
    m_attributes = attributes;
}

Client::Core::Result<IMECompositionAttribute, IMEError> IMECompositionManager::GetAttribute(size_t byteIndex) const {
    if (byteIndex >= m_attributes.size()) {
        return std::unexpected(IMEError::AttributeIndexOutOfBounds);
    }
    return m_attributes[byteIndex];
}

void IMECompositionManager::Clear() noexcept {
    m_composition.clear();
    m_reading.clear();
    m_caretPos = 0;
    m_clauses.clear();
    m_attributes.clear();
}

Client::Core::Result<size_t, IMEError> IMECompositionManager::GetCodepointLength(char leadByte) noexcept {
    unsigned char uByte = static_cast<unsigned char>(leadByte);
    
    if (uByte <= 0x7F) {
        return 1;
    } else if ((uByte & 0xE0) == 0xC0) {
        return 2;
    } else if ((uByte & 0xF0) == 0xE0) {
        return 3;
    } else if ((uByte & 0xF8) == 0xF0) {
        return 4;
    }
    
    return std::unexpected(IMEError::InvalidUTF8Sequence);
}

Client::Core::Result<size_t, IMEError> IMECompositionManager::CodepointOffsetToByteOffset(size_t codepointOffset) const noexcept {
    size_t currentByteOffset = 0;
    size_t currentCodepointOffset = 0;
    
    while (currentByteOffset < m_composition.length() && currentCodepointOffset < codepointOffset) {
        auto lenResult = GetCodepointLength(m_composition[currentByteOffset]);
        if (!lenResult) {
            return std::unexpected(lenResult.error());
        }
        
        size_t len = lenResult.value();
        if (currentByteOffset + len > m_composition.length()) {
            return std::unexpected(IMEError::InvalidUTF8Sequence);
        }
        
        currentByteOffset += len;
        currentCodepointOffset++;
    }
    
    if (currentCodepointOffset < codepointOffset) {
        return m_composition.length();
    }
    
    return currentByteOffset;
}

Client::Core::Result<size_t, IMEError> IMECompositionManager::ByteOffsetToCodepointOffset(size_t byteOffset) const noexcept {
    if (byteOffset > m_composition.length()) {
        byteOffset = m_composition.length();
    }
    
    size_t currentByteOffset = 0;
    size_t currentCodepointOffset = 0;
    
    while (currentByteOffset < byteOffset) {
        auto lenResult = GetCodepointLength(m_composition[currentByteOffset]);
        if (!lenResult) {
            return std::unexpected(lenResult.error());
        }
        
        size_t len = lenResult.value();
        if (currentByteOffset + len > m_composition.length()) {
            return std::unexpected(IMEError::InvalidUTF8Sequence);
        }
        
        currentByteOffset += len;
        currentCodepointOffset++;
    }
    
    return currentCodepointOffset;
}

} // namespace Client::Input
