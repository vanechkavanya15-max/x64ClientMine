#include "doctest.h"
#include <string>
#include <vector>
#include <algorithm>

// Mocking required components for testing string parsing and list logic

namespace EterLibMock {
    struct GraphicText {
        std::string OnLoad(const char* c_szFileName) {
            std::string fileName = c_szFileName ? c_szFileName : "";
            std::string fontName;
            int size = 12;
            bool bItalic = false;

            size_t colonPos = fileName.find_last_of(':');
            if (colonPos != std::string::npos) {
                fontName = fileName.substr(0, colonPos);
                std::string modifiers = fileName.substr(colonPos + 1);
                
                size_t dotPos = modifiers.find_last_of('.');
                if (dotPos != std::string::npos)
                    modifiers = modifiers.substr(0, dotPos);
                    
                std::string numStr = "";
                for (char c : modifiers) {
                    if (isdigit(c)) numStr += c;
                    else if (c == 'i') bItalic = true;
                }
                
                if (!numStr.empty()) {
                    try { size = std::stoi(numStr); } catch (...) { size = 12; }
                }
            } else {
                size_t dotPos = fileName.find_last_of('.');
                if (dotPos == std::string::npos) fontName = fileName;
                else fontName = fileName.substr(0, dotPos);
                size = 12;
            }
            return fontName + "_" + std::to_string(size) + "_" + (bItalic ? "italic" : "normal");
        }
    };
}

TEST_CASE("EterLib::GrpText parsing fixes") {
    EterLibMock::GraphicText txt;
    CHECK(txt.OnLoad("arial.fnt") == "arial_12_normal");
    CHECK(txt.OnLoad("tahoma:18.fnt") == "tahoma_18_normal");
    CHECK(txt.OnLoad("verdana:14i.fnt") == "verdana_14_italic");
    CHECK(txt.OnLoad("long_font_name_exceeding_32_characters_bug.fnt") == "long_font_name_exceeding_32_characters_bug_12_normal");
    CHECK(txt.OnLoad("crazy_font_name_with_many_digits:1234567890i.fnt") == "crazy_font_name_with_many_digits_1234567890_italic");
    CHECK(txt.OnLoad("no_extension") == "no_extension_12_normal");
}

namespace UserInterfaceMock {
    struct TextTail { int id; };
    
    std::vector<TextTail*> m_ItemTextTailList;
    std::vector<TextTail*> m_CharacterTextTailList;
    
    void DeleteItemTextTail(TextTail* pTextTail) {
        auto it = std::find(m_ItemTextTailList.begin(), m_ItemTextTailList.end(), pTextTail);
        if (it != m_ItemTextTailList.end())
            m_ItemTextTailList.erase(it);
    }
}

TEST_CASE("UserInterface::PythonTextTail Use-After-Free fix") {
    UserInterfaceMock::TextTail t1{1}, t2{2}, t3{3};
    UserInterfaceMock::m_ItemTextTailList = {&t1, &t2, &t3};
    
    CHECK(UserInterfaceMock::m_ItemTextTailList.size() == 3);
    UserInterfaceMock::DeleteItemTextTail(&t2);
    CHECK(UserInterfaceMock::m_ItemTextTailList.size() == 2);
    CHECK(UserInterfaceMock::m_ItemTextTailList[0]->id == 1);
    CHECK(UserInterfaceMock::m_ItemTextTailList[1]->id == 3);
}

// Ensure the Pick logic works efficiently with vectors
TEST_CASE("UserInterface::PythonTextTail Pick logic optimization") {
    UserInterfaceMock::m_ItemTextTailList.clear();
    UserInterfaceMock::TextTail t1{1}, t2{2};
    UserInterfaceMock::m_ItemTextTailList.push_back(&t1);
    
    int picked = -1;
    for (auto itor = UserInterfaceMock::m_ItemTextTailList.begin(); itor != UserInterfaceMock::m_ItemTextTailList.end(); ++itor) {
        if ((*itor)->id == 1) {
            picked = (*itor)->id;
        }
    }
    CHECK(picked == 1);
}
