import re

with open("crates/m2-client/src/py_bridge_game_systems.rs", "r") as f:
    content = f.read()

replacement = """#[no_mangle]
pub extern "C" fn m2_py_gamesys_skill_get_level(skill_id: u32) -> u32 {
    safe_ffi_call(0, || {
        unsafe {
            m2_client_bridge::state::skills::m2_skills_get_global_level(skill_id) as u32
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_skill_get_cool_time(skill_id: u32) -> u32 {
    safe_ffi_call(0, || {
        unsafe {
            let next_use = m2_client_bridge::state::skills::m2_skills_get_global_next_use_time(skill_id);
            let current = crate::timer::get_msec();
            if next_use > current {
                next_use.saturating_sub(current)
            } else {
                0
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_skill_can_use_skill(skill_id: u32) -> bool {
    safe_ffi_call(false, || {
        let skill_level = unsafe { m2_client_bridge::state::skills::m2_skills_get_global_level(skill_id) };
        if skill_level == 0 {
            return false;
        }

        let current_sp = if let Ok(state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
            state.get_sp()
        } else {
            return false;
        };

        let sp_cost = crate::skill_data_cache_v2::m2_skill_get_sp_cost(skill_id, skill_level);

        if (current_sp as u32) < sp_cost {
            return false;
        }

        let next_use = unsafe { m2_client_bridge::state::skills::m2_skills_get_global_next_use_time(skill_id) };
        let current = crate::timer::get_msec();
        if next_use > current {
            return false;
        }

        true
    })
}"""

content = re.sub(r'#\[no_mangle\]\npub extern "C" fn m2_py_gamesys_skill_get_level.*?#\[no_mangle\]\npub extern "C" fn m2_py_gamesys_skill_can_use_skill.*?\n\}\n', replacement + "\n", content, flags=re.DOTALL)

with open("crates/m2-client/src/py_bridge_game_systems.rs", "w") as f:
    f.write(content)

