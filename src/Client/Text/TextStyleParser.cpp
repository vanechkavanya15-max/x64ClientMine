#include "TextStyleParser.h"

namespace Client::Text {

TagParseResult TextStyleParser::ParseTag(std::wstring_view src) {
    if (src.empty() || src.size() < 2 || src[0] != L'|') {
        return {TagType::Plain, 1, {}};
    }

    wchar_t cur = src[1];

    if (cur == L'c') { // color
        if (src.size() < 10) {
            return {TagType::Plain, 1, {}};
        }
        std::wstring extraInfo(src.substr(2, 8));
        return {TagType::Color, 10, extraInfo};
    } else if (cur == L'|') { // escape pipe
        return {TagType::Tag, 2, {}};
    } else if (cur == L'r') { // restore color
        return {TagType::RestoreColor, 2, {}};
    } else if (cur == L'H') { // hyperlink start
        return {TagType::HyperlinkStart, 2, {}};
    } else if (cur == L'h') { // hyperlink end
        return {TagType::HyperlinkEnd, 2, {}};
    }

    return {TagType::Plain, 1, {}};
}

std::wstring TextStyleParser::GetOutputString(std::wstring_view src) {
    std::wstring dst;
    int hyperlinkStep = 0;

    for (size_t i = 0; i < src.size(); ) {
        auto [tagType, tagLength, extraInfo] = ParseTag(src.substr(i));

        if (tagType == TagType::Plain || tagType == TagType::Tag) {
            if (hyperlinkStep == 0 || hyperlinkStep == 2) {
                dst += src[i];
            }
        } else if (tagType == TagType::HyperlinkStart) {
            hyperlinkStep = 1;  
        } else if (tagType == TagType::HyperlinkEnd) {
            if (hyperlinkStep == 1) {
                hyperlinkStep = 2;
            } else if (hyperlinkStep == 2) {
                hyperlinkStep = 0;
            }
        }

        i += tagLength;
    }
    return dst;
}

size_t TextStyleParser::GetOutputLen(std::wstring_view src) {
    size_t output_len = 0;
    int hyperlinkStep = 0;

    for (size_t i = 0; i < src.size(); ) {
        auto [tagType, tagLength, extraInfo] = ParseTag(src.substr(i));

        if (tagType == TagType::Plain || tagType == TagType::Tag) {
            if (hyperlinkStep == 0) {
                ++output_len;
            }
        } else if (tagType == TagType::HyperlinkStart) {
            hyperlinkStep = 1;
        } else if (tagType == TagType::HyperlinkEnd) {
            hyperlinkStep = 0;
        }

        i += tagLength;
    }
    return output_len;
}

size_t TextStyleParser::GetInternalPosFromRenderPos(std::wstring_view src, size_t offset) {
    size_t output_len = 0;
    int hyperlinkStep = 0;
    bool color_tag = false;
    size_t internal_offset = 0;

    for (size_t i = 0; i < src.size(); ) {
        auto [tagType, tagLength, extraInfo] = ParseTag(src.substr(i));

        if (tagType == TagType::Color) {
            color_tag = true;
            internal_offset = i;
        } else if (tagType == TagType::RestoreColor) {
            color_tag = false;
        } else if (tagType == TagType::Plain || tagType == TagType::Tag) {
            if (hyperlinkStep == 0 || hyperlinkStep == 2) {
                internal_offset = i;

                if (offset <= output_len) {
                    return internal_offset;
                }

                ++output_len;
            }
        } else if (tagType == TagType::HyperlinkStart) {
            hyperlinkStep = 1;
        } else if (tagType == TagType::HyperlinkEnd) {
            if (hyperlinkStep == 1) {
                hyperlinkStep = 2;
            } else if (hyperlinkStep == 2) {
                hyperlinkStep = 0;
            }
        }

        i += tagLength;
    }

    if (offset >= output_len) {
        return src.size();
    }

    return internal_offset;
}

size_t TextStyleParser::FindColorTagStartPosition(std::wstring_view src) {
    if (src.size() < 2) return 0;
    
    if (src.back() == L'r' && src[src.size() - 2] == L'|') {
        size_t len = src.size();
        
        if (len >= 3 && src[src.size() - 3] == L'|') {
            return 1;
        }
        
        for (size_t i = len - 2; i > 0; --i) {
            if (src[i] == L'c' && src[i - 1] == L'|') {
                return len - i; 
            }
        }
        return src.size(); 
    } else if (src.back() == L'|' && src[src.size() - 2] == L'|') {
        return 1; 
    }

    return 0;
}

size_t TextStyleParser::FindColorTagEndPosition(std::wstring_view src) {
    if (src.size() < 2) return 0;

    if (src.size() >= 4 && src[0] == L'|' && src[1] == L'c') {
        for (size_t i = 2; i + 1 < src.size(); ++i) {
            if (src[i] == L'|' && src[i + 1] == L'r') {
                return i + 1; 
            }
        }
    } else if (src.size() >= 2 && src[0] == L'|' && src[1] == L'|') {
        return 1;
    }

    return 0;
}

} // namespace Client::Text
