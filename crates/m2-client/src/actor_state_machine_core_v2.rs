/// @brief Actor states representing character animations and logical modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorState {
    Wait,
    Walk,
    Run,
    Attack,
    Combo,
    Skill,
    Damage,
    Stun,
    Knockdown,
    Fishing,
    Mining,
    Dead,
}

impl Default for ActorState {
    /// @brief Default state is Wait.
    /// @return Default ActorState
    fn default() -> Self {
        Self::Wait
    }
}

/// @brief State machine core logical flags and logic matching CActorInstance/__InitializeStateData.
pub struct ActorStateMachine {
    current_state: ActorState,
    is_main_actor: bool,
    is_sleep: bool,
    is_paralysis: bool,
    is_faint: bool,
    is_real_dead: bool,
    is_stun: bool,
    is_hiding: bool,
    is_resist_fallen: bool,
}

impl ActorStateMachine {
    /// @brief Constructs a new actor state machine.
    /// @return Newly initialized ActorStateMachine.
    pub fn new() -> Self {
        Self {
            current_state: ActorState::default(),
            is_main_actor: false,
            is_sleep: false,
            is_paralysis: false,
            is_faint: false,
            is_real_dead: false,
            is_stun: false,
            is_hiding: false,
            is_resist_fallen: false,
        }
    }

    /// @brief Returns the current state.
    /// @return Current ActorState.
    pub fn current_state(&self) -> ActorState {
        self.current_state
    }

    /// @brief Directly sets the current state without validation.
    /// @param state The new state to set.
    pub fn set_state(&mut self, state: ActorState) {
        self.current_state = state;
    }

    /// @brief Initializes state logical variables matching CActorInstance::__InitializeStateData().
    pub fn initialize_state_data(&mut self) {
        self.is_sleep = false;
        self.is_paralysis = false;
        self.is_faint = false;
        self.is_real_dead = false;
        self.is_stun = false;
        self.is_hiding = false;
        self.is_resist_fallen = false;
        self.is_main_actor = false;
        self.current_state = ActorState::Wait;
    }

    /// @brief Sets whether this is the main actor.
    /// @param is_main Boolean flag indicating main actor status.
    pub fn set_main_actor(&mut self, is_main: bool) {
        self.is_main_actor = is_main;
    }
    
    /// @brief Returns whether this is the main actor.
    pub fn is_main_actor(&self) -> bool {
        self.is_main_actor
    }

    /// @brief Checks if the actor can move.
    /// @return True if the actor can move, false otherwise.
    pub fn can_move(&self) -> bool {
        !self.is_real_dead && !self.is_faint && !self.is_stun && !self.is_sleep && !self.is_paralysis
    }

    /// @brief Checks if the actor can attack.
    /// @return True if the actor can attack, false otherwise.
    pub fn can_attack(&self) -> bool {
        !self.is_real_dead && !self.is_faint && !self.is_stun && !self.is_sleep && !self.is_paralysis
    }

    /// @brief Tries to change to a new state with logical checks.
    /// @param new_state The requested new state.
    /// @return Result containing Ok if successful or Err with a reason if failed.
    pub fn change_state(&mut self, new_state: ActorState) -> Result<(), &'static str> {
        match new_state {
            ActorState::Walk | ActorState::Run => {
                if !self.can_move() {
                    return Err("Cannot move in current condition");
                }
            }
            ActorState::Attack | ActorState::Combo | ActorState::Skill => {
                if !self.can_attack() {
                    return Err("Cannot attack in current condition");
                }
            }
            ActorState::Dead => {
                self.is_real_dead = true;
            }
            ActorState::Stun => {
                self.is_stun = true;
            }
            _ => {}
        }
        
        self.current_state = new_state;
        Ok(())
    }
}
