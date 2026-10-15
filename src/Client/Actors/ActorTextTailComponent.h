#pragma once

#include <EterBase/StrongTypes.h>
#include <EterBase/Result.h>
#include <string>
#include <optional>

namespace Client::Actor {

enum class AlignmentKarma {
    Cruel,
    Neutral,
    Chivalric
};

struct ActorTextTailComponent {
    std::string name;
    std::optional<std::string> guildName;
    std::optional<std::string> title;
    AlignmentKarma karma{AlignmentKarma::Neutral};
    float modelHeight{0.0f};

    bool showHpBar{false};
    float hpPercentage{0.0f};
};

} // namespace Client::Actor
