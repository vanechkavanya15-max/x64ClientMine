content = open("crates/m2-client/src/horse_controller_complete.rs", "r").read()

content = content.replace("    pub fn initialize(&mut self) {\n        self.is_mounting = false;\n        self.actor = None;\n    }\n}\n\npub fn destroy", "    pub fn initialize(&mut self) {\n        self.is_mounting = false;\n        self.actor = None;\n    }\n\n    pub fn destroy")

content += "\n}\n"

open("crates/m2-client/src/horse_controller_complete.rs", "w").write(content)
