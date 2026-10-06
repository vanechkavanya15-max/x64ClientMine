import re

with open('crates/m2-client/src/py_bridge_texttail_full.rs', 'r') as f:
    content = f.read()

# Remove the mock pointers
content = content.replace("""    // We mock the pointers as Strings or bools for logic
    pub has_pOwnerTextInstance: bool,
    pub has_pMarkInstance: bool,
    pub stGuildName: Option<String>,
    pub stTitle: Option<String>,
    pub title_color: Option<D3DXCOLOR>,
    pub stLevel: Option<String>,
""", """    // We use usize to represent pointers to mock 1:1 C++ behavior since we don't have the original struct types here
    // In C++ these are: CGraphicTextInstance * pTextInstance, CGraphicObjectInstance * pOwnerTextInstance, etc.
    // For pure UI/FFI tests we map them to raw integer equivalents.
    pub pOwnerTextInstance: usize,
    pub pMarkInstance: usize,
    pub pGuildNameTextInstance: usize,
    pub pTitleTextInstance: usize,
    pub pLevelTextInstance: usize,
    pub pTextInstance: usize,
    
    // Additional fields mapped from C++
    pub stGuildName: Option<String>,
    pub stTitle: Option<String>,
    pub title_color: Option<D3DXCOLOR>,
    pub stLevel: Option<String>,
""")

content = content.replace("""            has_pOwnerTextInstance: false,
            has_pMarkInstance: false,
""", """            pOwnerTextInstance: 0,
            pMarkInstance: 0,
            pGuildNameTextInstance: 0,
            pTitleTextInstance: 0,
            pLevelTextInstance: 0,
            pTextInstance: 0,
""")

with open('crates/m2-client/src/py_bridge_texttail_full.rs', 'w') as f:
    f.write(content)

print("Structs updated.")
