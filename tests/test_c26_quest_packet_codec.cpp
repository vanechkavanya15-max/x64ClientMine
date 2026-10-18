#include <cstdint>
#include <vector>
#include <string>
#include <cassert>
#include <iostream>
#include <cstring>

#include "Client/Network/QuestPacketCodec.h"
#include "Client/Network/Protocol/Protocol.h"

using namespace Client::Network;

void test_DecodeQuestInfo_BufferUnderflow() {
    std::vector<uint8_t> buffer(sizeof(TPacketGCQuestInfo) - 1, 0);
    auto result = QuestPacketCodec::DecodeQuestInfo(buffer);
    assert(!result.has_value());
    assert(result.error() == EterBase::PacketError::BufferUnderflow);
}

void test_DecodeQuestInfo_Success() {
    std::vector<uint8_t> buffer(sizeof(TPacketGCQuestInfo), 0);
    TPacketGCQuestInfo* packet = reinterpret_cast<TPacketGCQuestInfo*>(buffer.data());
    packet->header = GC::QUEST_INFO;
    packet->length = sizeof(TPacketGCQuestInfo);
    packet->index = 42;
    packet->flag = 1;

    auto result = QuestPacketCodec::DecodeQuestInfo(buffer);
    assert(result.has_value());
    assert(result.value().header == GC::QUEST_INFO);
    assert(result.value().index == 42);
    assert(result.value().flag == 1);
}

void test_DecodeQuestConfirm_BufferUnderflow() {
    std::vector<uint8_t> buffer(sizeof(TPacketGCQuestConfirm) - 1, 0);
    auto result = QuestPacketCodec::DecodeQuestConfirm(buffer);
    assert(!result.has_value());
    assert(result.error() == EterBase::PacketError::BufferUnderflow);
}

void test_DecodeQuestConfirm_Success() {
    std::vector<uint8_t> buffer(sizeof(TPacketGCQuestConfirm), 0);
    TPacketGCQuestConfirm* packet = reinterpret_cast<TPacketGCQuestConfirm*>(buffer.data());
    packet->header = GC::QUEST_CONFIRM;
    packet->length = sizeof(TPacketGCQuestConfirm);
    packet->timeout = 1000;
    packet->requestPID = 12345;
    std::strncpy(packet->msg, "Hello Quest", sizeof(packet->msg) - 1);

    auto result = QuestPacketCodec::DecodeQuestConfirm(buffer);
    assert(result.has_value());
    assert(result.value().header == GC::QUEST_CONFIRM);
    assert(result.value().timeout == 1000);
    assert(result.value().requestPID == 12345);
    assert(std::string(result.value().msg) == "Hello Quest");
}

void test_DecodeScript_BufferUnderflow_Header() {
    std::vector<uint8_t> buffer(sizeof(TPacketGCScript) - 1, 0);
    auto result = QuestPacketCodec::DecodeScript(buffer);
    assert(!result.has_value());
    assert(result.error() == EterBase::PacketError::BufferUnderflow);
}

void test_DecodeScript_BufferUnderflow_Data() {
    std::vector<uint8_t> buffer(sizeof(TPacketGCScript) + 5, 0);
    TPacketGCScript* header = reinterpret_cast<TPacketGCScript*>(buffer.data());
    header->header = GC::SCRIPT;
    header->length = sizeof(TPacketGCScript) + 10; // claims more data than buffer size

    auto result = QuestPacketCodec::DecodeScript(buffer);
    assert(!result.has_value());
    assert(result.error() == EterBase::PacketError::BufferUnderflow);
}

void test_DecodeScript_Success() {
    std::string scriptText = "Hello Script";
    std::vector<uint8_t> buffer(sizeof(TPacketGCScript) + scriptText.size() + 1, 0);
    
    TPacketGCScript* header = reinterpret_cast<TPacketGCScript*>(buffer.data());
    header->header = GC::SCRIPT;
    header->length = static_cast<uint16_t>(buffer.size());

    std::memcpy(buffer.data() + sizeof(TPacketGCScript), scriptText.data(), scriptText.size());

    auto result = QuestPacketCodec::DecodeScript(buffer);
    assert(result.has_value());
    assert(result.value() == "Hello Script");
}

void test_EncodeScriptAnswer() {
    auto buffer = QuestPacketCodec::EncodeScriptAnswer(5);
    assert(buffer.size() == sizeof(TPacketCGScriptAnswer));

    const TPacketCGScriptAnswer* packet = reinterpret_cast<const TPacketCGScriptAnswer*>(buffer.data());
    assert(packet->header == CG::SCRIPT_ANSWER);
    assert(packet->length == sizeof(TPacketCGScriptAnswer));
    assert(packet->answer == 5);
}

void test_EncodeQuestConfirmAnswer() {
    auto buffer = QuestPacketCodec::EncodeQuestConfirmAnswer(1, 9999);
    assert(buffer.size() == sizeof(TPacketCGQuestConfirm));

    const TPacketCGQuestConfirm* packet = reinterpret_cast<const TPacketCGQuestConfirm*>(buffer.data());
    assert(packet->header == CG::QUEST_CONFIRM);
    assert(packet->length == sizeof(TPacketCGQuestConfirm));
    assert(packet->answer == 1);
    assert(packet->requestPID == 9999);
}

int main() {
    test_DecodeQuestInfo_BufferUnderflow();
    test_DecodeQuestInfo_Success();
    test_DecodeQuestConfirm_BufferUnderflow();
    test_DecodeQuestConfirm_Success();
    test_DecodeScript_BufferUnderflow_Header();
    test_DecodeScript_BufferUnderflow_Data();
    test_DecodeScript_Success();
    test_EncodeScriptAnswer();
    test_EncodeQuestConfirmAnswer();
    std::cout << "All quest packet codec tests passed!" << std::endl;
    return 0;
}

