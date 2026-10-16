#include "PlayerQuestManager.h"
#include <utility>

namespace Client::Player {

    PlayerQuestManager::PlayerQuestManager(IQuestNetworkContract* networkContract)
        : networkContract_(networkContract),
          isWaitingForConfirm_(false),
          currentConfirmRequestPID_(0)
    {
    }

    void PlayerQuestManager::OnQuestConfirm(const std::string& /*msg*/, int32_t /*timeout*/, uint32_t requestPID) {
        // Zapisujemy PID aby uzyc go w odpowiedzi
        isWaitingForConfirm_ = true;
        currentConfirmRequestPID_ = requestPID;
        
        // Tutaj docelowo mogloby powstac zdarzenie domenowe otwierajace UI (EterBase::EventBus),
        // ale obecny cel to tylko wydzielenie stanu.
    }

    Core::ExpectedResult<void, QuestManagerError> PlayerQuestManager::AnswerQuestConfirm(uint8_t answer) {
        if (!networkContract_) {
            return Core::MakeUnexpected(QuestManagerError::NullNetworkContract);
        }

        if (!isWaitingForConfirm_) {
            return Core::MakeUnexpected(QuestManagerError::NotWaitingForConfirm);
        }

        networkContract_->SendQuestConfirmAnswer(answer, currentConfirmRequestPID_);
        
        // Zresetuj stan po odpowiedzi
        isWaitingForConfirm_ = false;
        currentConfirmRequestPID_ = 0;

        return {};
    }

    Core::ExpectedResult<void, QuestManagerError> PlayerQuestManager::AnswerScript(int32_t answerIndex) {
        if (!networkContract_) {
            return Core::MakeUnexpected(QuestManagerError::NullNetworkContract);
        }

        networkContract_->SendScriptAnswer(answerIndex);

        return {};
    }

    bool PlayerQuestManager::IsWaitingForConfirm() const noexcept {
        return isWaitingForConfirm_;
    }

    uint32_t PlayerQuestManager::GetCurrentConfirmRequestPID() const noexcept {
        return currentConfirmRequestPID_;
    }

}
