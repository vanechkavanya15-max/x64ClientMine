#include <iostream>
#include <span>
#include <vector>
#include <cstdint>
#include <cstring>
#include <cassert>
#include "EterBase/Result.h"
#include "Client/Network/Handlers/QuestHandler.h"
#include "Client/Network/Protocol/Protocol.h"

using namespace Client::Network::Handlers;

void testHandleWarp() {
    QuestHandler handler;
    std::vector<uint8_t> buffer(sizeof(TPacketGCWarp));
    TPacketGCWarp warp{};
    warp.lX = 100; warp.lY = 200; warp.lAddr = 12345; warp.wPort = 1234;
    std::memcpy(buffer.data(), &warp, sizeof(warp));

    auto result = handler.HandleWarp(buffer);
    assert(result.has_value());
    std::cout << "Test passed (HandleWarp)" << std::endl;
}

void testHandleScript() {
    QuestHandler handler;
    std::string scriptContent = "def test(): pass";
    std::vector<uint8_t> buffer(sizeof(TPacketGCScript) + scriptContent.size() + 1);
    
    TPacketGCScript sp{};
    sp.length = static_cast<uint16_t>(buffer.size());
    sp.skin = 1;
    sp.src_size = static_cast<uint16_t>(scriptContent.size());
    
    std::memcpy(buffer.data(), &sp, sizeof(sp));
    std::memcpy(buffer.data() + sizeof(sp), scriptContent.c_str(), scriptContent.size());
    buffer.back() = '\0';
    
    auto result = handler.HandleScript(buffer);
    assert(result.has_value());
    std::cout << "Test passed (HandleScript)" << std::endl;
}

void testHandleQuestInfo() {
    QuestHandler handler;
    std::vector<uint8_t> buffer(sizeof(TPacketGCQuestInfo) + sizeof(uint8_t) + 31);
    
    TPacketGCQuestInfo qi{};
    qi.flag = QUEST_SEND_IS_BEGIN | QUEST_SEND_TITLE;
    qi.index = 10;
    
    size_t offset = 0;
    std::memcpy(buffer.data(), &qi, sizeof(qi));
    offset += sizeof(qi);
    
    uint8_t isBegin = 1;
    std::memcpy(buffer.data() + offset, &isBegin, sizeof(isBegin));
    offset += sizeof(isBegin);
    
    std::string title = "Test Quest Title";
    std::memcpy(buffer.data() + offset, title.c_str(), title.size());
    buffer[offset + title.size()] = '\0';
    
    auto result = handler.HandleQuestInfo(buffer);
    assert(result.has_value());
    std::cout << "Test passed (HandleQuestInfo)" << std::endl;
}

int main() {
    testHandleWarp();
    testHandleScript();
    testHandleQuestInfo();
    std::cout << "All quest handler tests passed." << std::endl;
    return 0;
}

