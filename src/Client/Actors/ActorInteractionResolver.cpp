#include "ActorInteractionResolver.h"
#include "Client/Actors/ActorTypeClassifier.h"
#include <cmath>

namespace Client::Actors {

ActorInteractionResolver::ActorInteractionResolver(const Client::World::ActorRegistry& registry) noexcept
    : m_registry(registry)
{
}

std::expected<bool, InteractionError> ActorInteractionResolver::CheckAdvancingCollision(
    Client::World::EntityVid actorVid, 
    const Client::Actor::ActorCollisionComponent& actorCollision) const noexcept
{
    if (!actorCollision.IsInitialized()) {
        return std::unexpected(InteractionError::InvalidCollision);
    }

    bool hasCollision = false;

    m_registry.for_each_actor([&](const Client::World::ActorRecord& record) {
        if (hasCollision || record.vid == actorVid) {
            return;
        }

        // Drzwi i bramy blokuja ruch w CheckAdvancing()
        if (ActorTypeClassifier::IsDoor(record.race)) {
            // Zakladamy domyslny promien dla drzwi na podstawie record (np. 150.0f jesli brak komponentu)
            float dx = record.x - actorCollision.GetCenter().x;
            float dy = record.y - actorCollision.GetCenter().y;
            float distanceSq = dx * dx + dy * dy;
            
            float combinedRadius = actorCollision.GetRadius() + 150.0f; // Default door radius
            
            if (distanceSq <= combinedRadius * combinedRadius) {
                hasCollision = true;
            }
        }
    });

    return hasCollision;
}

std::expected<Client::World::EntityVid, InteractionError> ActorInteractionResolver::ProcessAttack(
    Client::World::EntityVid attackerVid, 
    const Client::Actor::ActorCollisionComponent& attackerWeaponCollision) const noexcept
{
    if (!attackerWeaponCollision.IsInitialized()) {
        return std::unexpected(InteractionError::InvalidCollision);
    }

    Client::World::EntityVid targetVid{0};
    float minDistanceSq = -1.0f;

    m_registry.for_each_actor([&](const Client::World::ActorRecord& record) {
        if (record.vid == attackerVid || record.isDead) {
            return;
        }

        // Nie atakujemy drzwi i NPC
        if (ActorTypeClassifier::IsDoor(record.race) || ActorTypeClassifier::IsNPC(record.race)) {
            return;
        }

        float dx = record.x - attackerWeaponCollision.GetCenter().x;
        float dy = record.y - attackerWeaponCollision.GetCenter().y;
        float distanceSq = dx * dx + dy * dy;

        float combinedRadius = attackerWeaponCollision.GetRadius() + 100.0f; // Default enemy radius

        if (distanceSq <= combinedRadius * combinedRadius) {
            if (minDistanceSq < 0.0f || distanceSq < minDistanceSq) {
                minDistanceSq = distanceSq;
                targetVid = record.vid;
            }
        }
    });

    if (targetVid.value() != 0) {
        return targetVid;
    }

    return std::unexpected(InteractionError::TargetNotFound);
}

} // namespace Client::Actors
