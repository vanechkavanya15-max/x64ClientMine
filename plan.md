1. **Understand Architectural Goal & Module Role**:
   - `chat`, `messenger`, `guild`, `safebox` manage game communications, social features, guild, and player vault.
   - Tied closely to Python UI using Python method bindings.
   
2. **Draft the Markdown Document (`docs/04_python_bindings/05_chat_messenger_guild_api.md`)**:
   - Use plain ASCII characters for the Polish text.
   - Include Architecture and Data Flow (Mermaid diagrams).
   - Document Memory Layout & Structs (`CPythonChat::SChatLine`, `CWhisper::SChatLine`, `CPythonGuild::TGuildInfo`, `CPythonSafeBox` items).
   - Document Classes & Methods (`CPythonChat`, `CPythonMessenger`, `CPythonGuild`, `CPythonSafeBox`) detailing parameters, return values, and inner algorithms.
   - Document Cross-Subsystem Integration (Python UI calls, DirectX for rendering chat texts via `CGraphicTextInstance`, pool memory management `CDynamicPool`).
   - Note limitations, pitfalls (e.g. `CHAT_LINE_MAX_NUM` limit in deque, `CGraphicTextInstance` deletion needs).

3. **Verify and Commit**:
   - Read the generated file to ensure it meets requirements.
   - Call `pre_commit_instructions` tool to run checks.
   - Execute git commit as requested.
