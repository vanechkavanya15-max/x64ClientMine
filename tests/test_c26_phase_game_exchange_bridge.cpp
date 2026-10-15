#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#ifndef TEST_MODE_DISABLE_STDAFX
#define TEST_MODE_DISABLE_STDAFX
#endif

#include <doctest.h>
#include "Client/Bridge/PhaseGameExchangeBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include <cstring>
#include <vector>

using namespace Client::Bridge;
using namespace Client::Network;

TEST_SUITE("PhaseGameExchangeBridge") {

    TEST_CASE("HandleExchangePacket routes to Start Exchange") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "StartExchange", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 1);
            REQUIRE(args[0] == "1234");
            return "ok";
        });
        
        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::START;
        pack.arg1 = 1234;

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].moduleName == "exchange");
        CHECK(history[0].methodName == "StartExchange");
        CHECK(history[0].args[0] == "1234");
    }

    TEST_CASE("HandleExchangePacket routes to ItemAdd for self") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "SetItemToSelf", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });
        runtime.RegisterMock("exchange", "SetItemMetinSocketToSelf", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });
        runtime.RegisterMock("exchange", "SetItemAttributeToSelf", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });
        runtime.RegisterMock("exchange", "RefreshExchangeWindow", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ITEM_ADD;
        pack.is_me = 1;
        pack.arg1 = 50201; // vnum
        pack.arg2.cell = 2; // pos
        pack.arg3 = 5; // count
        for (int i = 0; i < ITEM_SOCKET_SLOT_MAX_NUM; ++i) pack.alValues[i] = i * 10;
        for (int i = 0; i < ITEM_ATTRIBUTE_SLOT_MAX_NUM; ++i) {
            pack.aAttr[i].bType = i;
            pack.aAttr[i].sValue = i * 2;
        }

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        // 1 for SetItemToSelf
        // 3 for SetItemMetinSocketToSelf (ITEM_SOCKET_SLOT_MAX_NUM)
        // 7 for SetItemAttributeToSelf (ITEM_ATTRIBUTE_SLOT_MAX_NUM)
        // 1 for RefreshExchangeWindow
        REQUIRE(history.size() == 1 + ITEM_SOCKET_SLOT_MAX_NUM + ITEM_ATTRIBUTE_SLOT_MAX_NUM + 1);
        
        CHECK(history[0].methodName == "SetItemToSelf");
        CHECK(history[0].args[0] == "2");
        CHECK(history[0].args[1] == "50201");
        CHECK(history[0].args[2] == "5");
        
        CHECK(history[1].methodName == "SetItemMetinSocketToSelf");
        CHECK(history[4].methodName == "SetItemAttributeToSelf");
        CHECK(history.back().methodName == "RefreshExchangeWindow");
    }

    TEST_CASE("HandleExchangePacket routes to ItemAdd for target") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "SetItemToTarget", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });
        runtime.RegisterMock("exchange", "SetItemMetinSocketToTarget", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });
        runtime.RegisterMock("exchange", "SetItemAttributeToTarget", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });
        runtime.RegisterMock("exchange", "RefreshExchangeWindow", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ITEM_ADD;
        pack.is_me = 0;
        pack.arg1 = 50202; // vnum
        pack.arg2.cell = 3; // pos
        pack.arg3 = 10; // count

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() > 0);
        CHECK(history[0].methodName == "SetItemToTarget");
        CHECK(history[0].args[0] == "3");
        CHECK(history[0].args[1] == "50202");
        CHECK(history[0].args[2] == "10");
        CHECK(history.back().methodName == "RefreshExchangeWindow");
    }

    TEST_CASE("HandleExchangePacket routes to ItemDel for self") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "DelItemOfSelf", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 1);
            REQUIRE(args[0] == "2");
            return "ok";
        });
        runtime.RegisterMock("exchange", "RefreshExchangeWindow", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ITEM_DEL;
        pack.is_me = 1;
        pack.arg1 = 2; // pos

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 2);
        CHECK(history[0].methodName == "DelItemOfSelf");
        CHECK(history[1].methodName == "RefreshExchangeWindow");
    }
    
    TEST_CASE("HandleExchangePacket routes to ItemDel for target") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "DelItemOfTarget", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 1);
            REQUIRE(args[0] == "3");
            return "ok";
        });
        runtime.RegisterMock("exchange", "RefreshExchangeWindow", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ITEM_DEL;
        pack.is_me = 0;
        pack.arg1 = 3; // pos

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 2);
        CHECK(history[0].methodName == "DelItemOfTarget");
        CHECK(history[1].methodName == "RefreshExchangeWindow");
    }

    TEST_CASE("HandleExchangePacket routes to ElkAdd for self") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "SetElkToSelf", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 1);
            REQUIRE(args[0] == "10000");
            return "ok";
        });
        runtime.RegisterMock("exchange", "RefreshExchangeWindow", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ELK_ADD;
        pack.is_me = 1;
        pack.arg1 = 10000; // gold

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 2);
        CHECK(history[0].methodName == "SetElkToSelf");
        CHECK(history[1].methodName == "RefreshExchangeWindow");
    }

    TEST_CASE("HandleExchangePacket routes to ElkAdd for target") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "SetElkToTarget", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 1);
            REQUIRE(args[0] == "50000");
            return "ok";
        });
        runtime.RegisterMock("exchange", "RefreshExchangeWindow", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ELK_ADD;
        pack.is_me = 0;
        pack.arg1 = 50000; // gold

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 2);
        CHECK(history[0].methodName == "SetElkToTarget");
        CHECK(history[1].methodName == "RefreshExchangeWindow");
    }

    TEST_CASE("HandleExchangePacket routes to Accept for self") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "SetAcceptToSelf", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 1);
            REQUIRE(args[0] == "1");
            return "ok";
        });
        runtime.RegisterMock("exchange", "RefreshExchangeWindow", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ACCEPT;
        pack.is_me = 1;
        pack.arg1 = 1; // 1 = accepted

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 2);
        CHECK(history[0].methodName == "SetAcceptToSelf");
        CHECK(history[1].methodName == "RefreshExchangeWindow");
    }

    TEST_CASE("HandleExchangePacket routes to Accept for target") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "SetAcceptToTarget", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 1);
            REQUIRE(args[0] == "1");
            return "ok";
        });
        runtime.RegisterMock("exchange", "RefreshExchangeWindow", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ACCEPT;
        pack.is_me = 0;
        pack.arg1 = 1; // 1 = accepted

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 2);
        CHECK(history[0].methodName == "SetAcceptToTarget");
        CHECK(history[1].methodName == "RefreshExchangeWindow");
    }

    TEST_CASE("HandleExchangePacket routes to End") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "EndExchange", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::END;

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].methodName == "EndExchange");
    }

    TEST_CASE("HandleExchangePacket routes to Already") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "TradeAlready", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::ALREADY;

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].methodName == "TradeAlready");
    }

    TEST_CASE("HandleExchangePacket routes to LessElk") {
        MockPythonRuntime runtime;
        runtime.Initialize();
        runtime.RegisterMock("exchange", "TradeLessElk", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "ok"; });

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = ExchangeSub::GC::LESS_ELK;

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true);

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].methodName == "TradeLessElk");
    }

    TEST_CASE("HandleExchangePacket ignores unknown subheader") {
        MockPythonRuntime runtime;
        runtime.Initialize();

        PhaseGameExchangeBridge bridge(runtime);

        TPacketGCExchange pack{};
        pack.header = GC::EXCHANGE;
        pack.subheader = 255; // unknown

        bool result = bridge.HandleExchangePacket(pack);
        CHECK(result == true); // Should return true as it's handled (ignored)

        auto history = runtime.GetHistory();
        REQUIRE(history.size() == 0);
    }
}
