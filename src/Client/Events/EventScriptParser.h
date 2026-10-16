#pragma once

#include <string>
#include <string_view>
#include <vector>
#include <expected>
#include <cstdint>

namespace Client::Events {

enum class ParserError {
    EmptyInput,
    UnmatchedQuotes,
    EmptyCommandName
};

struct ScriptArgument {
    std::string value;

    bool operator==(const ScriptArgument& other) const = default;
};

struct ScriptCommand {
    std::string name;
    std::vector<ScriptArgument> arguments;

    bool operator==(const ScriptCommand& other) const = default;
};

class EventScriptParser {
public:
    static std::expected<std::vector<ScriptCommand>, ParserError> Parse(std::string_view script);
};

} // namespace Client::Events
