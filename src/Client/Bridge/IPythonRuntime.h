#pragma once

#include <string>
#include <string_view>
#include <span>
#include <vector>
#include <type_traits>
#include "EterBase/Result.h"

namespace Client::Bridge {

enum class PythonRuntimeError : uint8_t;

    /**
     * @brief Interfejs dla Pythona
     */
    class IPythonRuntime {
    public:
        virtual ~IPythonRuntime() = default;
        virtual void Initialize() noexcept = 0;
        virtual void Shutdown() noexcept = 0;
        virtual EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> ExecuteMethod(
            std::string_view moduleName, 
            std::string_view methodName, 
            std::span<const std::string> args = {}) = 0;
        
        template <typename... Args>
        EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> Invoke(
            std::string_view moduleName, 
            std::string_view methodName, 
            Args&&... args)
        {
            std::vector<std::string> stringArgs;
            if constexpr (sizeof...(Args) > 0) {
                stringArgs = { FormatArg(std::forward<Args>(args))... };
            }
            return ExecuteMethod(moduleName, methodName, stringArgs);
        }
        
    protected:
        template <typename T>
        static std::string FormatArg(T&& val) {
            using DecayedT = std::remove_cvref_t<T>;
            if constexpr (std::is_convertible_v<DecayedT, std::string_view>) {
                return std::string(std::string_view(val));
            } else if constexpr (std::is_arithmetic_v<DecayedT>) {
                return std::to_string(val);
            } else {
                return "[unsupported_type]";
            }
        }
    };
} // namespace Client::Bridge
