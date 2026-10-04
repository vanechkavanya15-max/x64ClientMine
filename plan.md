1.  **Understand the Goal**: The objective is to implement a Rust module `engine_root_motion_actor_sync.rs` and its corresponding tests in `crates/m2-client/tests/test_root_motion_actor_sync.rs`. The module is responsible for integrating root bone (Bip01) translation deltas into the actor's actual 3D coordinates on the map. This involves extracting the root bone's position delta per animation frame (e.g., during a jump or forward attack) and applying it to the instance's 3D world position, maintaining 1:1 behavioral parity with `src/GameLib/ActorInstanceMotion.cpp`. No empty stubs are allowed.

2.  **Analyze C++ Reference**:
    In `src/GameLib/ActorInstance.cpp`:
    ```cpp
    void CActorInstance::__AccumulationMovement(float fRot) {
        if (CRaceMotionData::NAME_WAIT == __GetCurrentMotionIndex()) return;
        D3DXMATRIX s_matRotationZ;
        D3DXMatrixRotationZ(&s_matRotationZ, D3DXToRadian(fRot));
        UpdateTransform(&s_matRotationZ, GetAverageSecondElapsed());
        AddMovement(s_matRotationZ._41, s_matRotationZ._42, s_matRotationZ._43);
    }
    
    void CActorInstance::TransformProcess() {
        if (!IsParalysis()) {
            m_x += m_v3Movement.x;
            m_y += m_v3Movement.y;
            m_z += m_v3Movement.z;
        }
        __InitializeMovement();
        SetPosition(m_x, m_y, m_z);
    }
    
    void CActorInstance::AddMovement(float fx, float fy, float fz) {
        m_v3Movement.x += fx;
        m_v3Movement.y += fy;
        m_v3Movement.z += fz;
    }
    ```
    The translation comes from `UpdateTransform` which evaluates the Granny model matrix `GrannyUpdateModelMatrix(..., (float *) pMatrix, ...)`. In Granny3D, `pMatrix` gets populated with the model's root motion (typically extraction from the root bone delta, often `Bip01`). We are dealing with extracting this delta and applying it.

3.  **Implement `engine_root_motion_actor_sync.rs`**:
    *   Create a struct `RootMotionSync` or functions to extract delta and apply it.
    *   We need a state struct to hold accumulated movement and apply it to a given transform.
    *   Let's create `engine_root_motion_actor_sync.rs` with:
        *   `extract_root_bone_delta`: Given current root bone matrix and previous root bone matrix, extract translation delta.
        *   `accumulate_movement`: Apply rotation to delta (similar to `s_matRotationZ._41`, etc.) and add to movement vector.
        *   `apply_root_motion`: Update the actor's world transform by adding the accumulated movement, similar to `TransformProcess`.
    
4.  **Implement `test_root_motion_actor_sync.rs`**:
    *   Create tests that verify the extraction of the delta, accumulation, and application to the transform.
    
5.  **Pre-commit steps**: Run tests, fix any warnings/errors.
