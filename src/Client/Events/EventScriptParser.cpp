#include "EventScriptParser.h"
#include <cctype>

namespace Client::Events {

std::expected<std::vector<ScriptCommand>, ParserError> EventScriptParser::Parse(std::string_view script) {
    auto start = script.find_first_not_of(" \t\r\n");
    if (start == std::string_view::npos) {
        return std::unexpected(ParserError::EmptyInput);
    }

    std::vector<ScriptCommand> commands;
    size_t i = 0;

    while (i < script.length()) {
        // Skip whitespace including newlines
        while (i < script.length() && std::isspace(static_cast<unsigned char>(script[i]))) {
            i++;
        }
        if (i == script.length()) break;

        ScriptCommand current_cmd;
        bool has_cmd_name = false;

        while (i < script.length() && script[i] != '\n' && script[i] != '\r') {
            // Skip spaces but not newlines
            while (i < script.length() && script[i] != '\n' && script[i] != '\r' && std::isspace(static_cast<unsigned char>(script[i]))) {
                i++;
            }
            if (i == script.length() || script[i] == '\n' || script[i] == '\r') break;

            std::string current_token;
            bool in_quotes = false;
            bool token_started = false;

            while (i < script.length() && script[i] != '\n' && script[i] != '\r') {
                if (script[i] == '"') {
                    in_quotes = !in_quotes;
                    token_started = true;
                    i++;
                } else if (!in_quotes && std::isspace(static_cast<unsigned char>(script[i]))) {
                    break;
                } else {
                    current_token += script[i];
                    token_started = true;
                    i++;
                }
            }

            if (in_quotes) {
                return std::unexpected(ParserError::UnmatchedQuotes);
            }

            if (token_started) {
                if (!has_cmd_name) {
                    if (current_token.empty()) {
                        return std::unexpected(ParserError::EmptyCommandName);
                    }
                    current_cmd.name = current_token;
                    has_cmd_name = true;
                } else {
                    current_cmd.arguments.push_back(ScriptArgument{current_token});
                }
            }
        }

        if (has_cmd_name) {
            commands.push_back(current_cmd);
        }

        if (i < script.length() && script[i] == '\r') i++;
        if (i < script.length() && script[i] == '\n') i++;
    }

    return commands;
}

} // namespace Client::Events
