#pragma once

#include "ServerType.h"

namespace Network
{
    /**
     * @brief Singleton zarzadzajacy aktywnym profilem serwera (Official vs Beavium).
     */
    class MultiServerManager
    {
    public:
        static MultiServerManager& Instance()
        {
            static MultiServerManager instance;
            return instance;
        }

        void SetServerType(ServerType type)
        {
            m_currentType = type;
        }

        ServerType GetServerType() const
        {
            return m_currentType;
        }

        bool IsOfficial() const
        {
            return m_currentType == ServerType::Official;
        }

        bool IsBeavium() const
        {
            return m_currentType == ServerType::Beavium;
        }

        bool IsAuto() const
        {
            return m_currentType == ServerType::Auto;
        }

    private:
        MultiServerManager() : m_currentType(ServerType::Official) {}
        ~MultiServerManager() = default;

        ServerType m_currentType;
    };
}
