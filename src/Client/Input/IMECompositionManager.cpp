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

void IMECompositionManager::SetCompositionWide(std::wstring_view wstr, int32_t caretPos) {
    if (wstr.empty()) {
        Clear();
        return;
    }
    std::string utf8;
    utf8.reserve(wstr.size() * 3);
    for (size_t i = 0; i < wstr.size(); ++i) {
        wchar_t c1 = wstr[i];
        uint32_t cp = static_cast<uint32_t>(c1);
        if (c1 >= 0xD800 && c1 <= 0xDBFF) {
            if (i + 1 < wstr.size()) {
                wchar_t c2 = wstr[i + 1];
                if (c2 >= 0xDC00 && c2 <= 0xDFFF) {
                    cp = ((static_cast<uint32_t>(c1) - 0xD800) << 10) + (static_cast<uint32_t>(c2) - 0xDC00) + 0x10000;
                    ++i;
                }
            }
        }
        if (cp < 0x80) {
            utf8.push_back(static_cast<char>(cp));
        } else if (cp < 0x800) {
            utf8.push_back(static_cast<char>(0xC0 | (cp >> 6)));
            utf8.push_back(static_cast<char>(0x80 | (cp & 0x3F)));
        } else if (cp < 0x10000) {
            utf8.push_back(static_cast<char>(0xE0 | (cp >> 12)));
            utf8.push_back(static_cast<char>(0x80 | ((cp >> 6) & 0x3F)));
            utf8.push_back(static_cast<char>(0x80 | (cp & 0x3F)));
        } else {
            utf8.push_back(static_cast<char>(0xF0 | (cp >> 18)));
            utf8.push_back(static_cast<char>(0x80 | ((cp >> 12) & 0x3F)));
            utf8.push_back(static_cast<char>(0x80 | ((cp >> 6) & 0x3F)));
            utf8.push_back(static_cast<char>(0x80 | (cp & 0x3F)));
        }
    }
    SetCompositionString(utf8, caretPos);
}

std::wstring IMECompositionManager::GetWideString() const {
    if (m_composition.empty()) {
        return {};
    }
    std::wstring out;
    out.reserve(m_composition.size());
    size_t i = 0;
    while (i < m_composition.size()) {
        unsigned char c = static_cast<unsigned char>(m_composition[i]);
        uint32_t cp = 0;
        int extra = 0;
        if (c < 0x80) {
            cp = c;
            extra = 0;
        } else if ((c & 0xE0) == 0xC0) {
            cp = c & 0x1F;
            extra = 1;
        } else if ((c & 0xF0) == 0xE0) {
            cp = c & 0x0F;
            extra = 2;
        } else if ((c & 0xF8) == 0xF0) {
            cp = c & 0x07;
            extra = 3;
        } else {
            ++i;
            continue;
        }
        ++i;
        bool valid = true;
        for (int k = 0; k < extra; ++k) {
            if (i >= m_composition.size()) { valid = false; break; }
            unsigned char nextC = static_cast<unsigned char>(m_composition[i++]);
            if ((nextC & 0xC0) != 0x80) { valid = false; break; }
            cp = (cp << 6) | (nextC & 0x3F);
        }
        if (!valid) continue;

        if (cp <= 0xFFFF) {
            out.push_back(static_cast<wchar_t>(cp));
        } else {
            cp -= 0x10000;
            out.push_back(static_cast<wchar_t>((cp >> 10) + 0xD800));
            out.push_back(static_cast<wchar_t>((cp & 0x3FF) + 0xDC00));
        }
    }
    return out;
}

size_t IMECompositionManager::GetLength() const noexcept {
    size_t count = 0;
    size_t offset = 0;
    while (offset < m_composition.length()) {
        auto lenRes = GetCodepointLength(m_composition[offset]);
        if (!lenRes) break;
        offset += lenRes.value();
        count++;
    }
    return count;
}

void IMECompositionManager::SetRawAttributes(const uint8_t* pAttr, size_t count) {
    if (!pAttr || count == 0) {
        m_attributes.clear();
        return;
    }
    std::vector<IMECompositionAttribute> attrs;
    attrs.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        switch (pAttr[i]) {
            case 0: attrs.push_back(IMECompositionAttribute::Input); break;
            case 1: attrs.push_back(IMECompositionAttribute::TargetConverted); break;
            case 2: attrs.push_back(IMECompositionAttribute::Converted); break;
            case 3: attrs.push_back(IMECompositionAttribute::TargetNotConverted); break;
            case 4: attrs.push_back(IMECompositionAttribute::InputError); break;
            case 5: attrs.push_back(IMECompositionAttribute::FixedConverted); break;
            default: attrs.push_back(IMECompositionAttribute::Input); break;
        }
    }
    SetAttributes(attrs);
}

std::pair<int, int> IMECompositionManager::GetTargetClauseRange() const noexcept {
    int start = 0;
    int end = 0;
    int count = static_cast<int>(m_attributes.size());
    for (start = 0; start < count; ++start) {
        if (m_attributes[start] == IMECompositionAttribute::TargetConverted ||
            m_attributes[start] == IMECompositionAttribute::TargetNotConverted) {
            break;
        }
    }
    for (end = start; end < count; ++end) {
        if (m_attributes[end] != m_attributes[start]) {
            break;
        }
    }
    return {start, end};
}

} // namespace Client::Input
