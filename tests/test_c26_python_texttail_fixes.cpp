#include <iostream>
#include <cassert>
#include <vector>
#include <unordered_map>
#include <algorithm>

// Standalone mockup to verify logic of Use-After-Free memory fix 
// outside of the complete DirectX dependent `PythonTextTail` engine

typedef unsigned long DWORD;

struct TTextTail {
    DWORD dwVirtualID;
};

std::unordered_map<DWORD, TTextTail*> m_CharacterTextTailMap;
std::vector<TTextTail*> m_CharacterTextTailList;

void MockDeleteTextTail(TTextTail* p) {
    delete p;
}

void MockDeleteCharacterTextTail(DWORD VirtualID) {
    auto itorCharacter = m_CharacterTextTailMap.find(VirtualID);
    
    if (m_CharacterTextTailMap.end() != itorCharacter)
    {
        TTextTail * pTextTail = itorCharacter->second;
        
        auto it = std::find(m_CharacterTextTailList.begin(), m_CharacterTextTailList.end(), pTextTail);
        if (it != m_CharacterTextTailList.end())
            m_CharacterTextTailList.erase(it);

        MockDeleteTextTail(pTextTail);
        m_CharacterTextTailMap.erase(itorCharacter);
    }
}

int main()
{
    std::cout << "Testing CPythonTextTail Use-After-Free fixes logic directly..." << std::endl;
    
    TTextTail* tail = new TTextTail();
    tail->dwVirtualID = 100;
    
    m_CharacterTextTailMap[100] = tail;
    m_CharacterTextTailList.push_back(tail);
    
    assert(m_CharacterTextTailMap.size() == 1);
    assert(m_CharacterTextTailList.size() == 1);
    
    // Test the logic implemented in PythonTextTail.cpp
    MockDeleteCharacterTextTail(100);
    
    assert(m_CharacterTextTailMap.empty());
    assert(m_CharacterTextTailList.empty()); // The erase block logic works properly
    
    std::cout << "CPythonTextTail fixes test passed." << std::endl;
    return 0;
}
