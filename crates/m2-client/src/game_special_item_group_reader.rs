use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SpecialItemDrop {
    pub vnum: u32,
    pub count: u32,
    pub prob: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecialItemGroup {
    pub vnum: u32,
    pub group_type: String,
    pub items: Vec<SpecialItemDrop>,
}

#[derive(Debug, Clone)]
pub struct SpecialItemGroupReader {
    groups: HashMap<u32, SpecialItemGroup>,
}

impl SpecialItemGroupReader {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }

    pub fn parse_content(&mut self, content: &str) -> Result<(), &'static str> {
        let mut current_group: Option<SpecialItemGroup> = None;
        let mut in_block = false;

        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            if !in_block {
                if tokens[0].eq_ignore_ascii_case("Group") {
                    if tokens.len() < 2 {
                        return Err("Invalid Group definition");
                    }
                    let vnum = tokens[1].parse::<u32>().map_err(|_| "Invalid Group Vnum")?;
                    current_group = Some(SpecialItemGroup {
                        vnum,
                        group_type: String::new(),
                        items: Vec::new(),
                    });
                } else if tokens[0] == "{" {
                    if current_group.is_none() {
                        return Err("Unexpected '{'");
                    }
                    in_block = true;
                } else {
                    return Err("Unexpected token outside of group block");
                }
            } else {
                if tokens[0] == "}" {
                    if let Some(group) = current_group.take() {
                        self.groups.insert(group.vnum, group);
                    }
                    in_block = false;
                } else if tokens[0].eq_ignore_ascii_case("Type") {
                    if tokens.len() < 2 {
                        return Err("Invalid Type definition");
                    }
                    if let Some(group) = current_group.as_mut() {
                        group.group_type = tokens[1].to_string();
                    }
                } else {
                    // It should be an item definition: <Index> <Vnum> <Count> <Prob>
                    if tokens.len() >= 4 {
                        let _index = tokens[0].parse::<u32>().map_err(|_| "Invalid Item Index")?;
                        let vnum = tokens[1].parse::<u32>().map_err(|_| "Invalid Item Vnum")?;
                        let count = tokens[2].parse::<u32>().map_err(|_| "Invalid Item Count")?;
                        let prob = tokens[3].parse::<u32>().map_err(|_| "Invalid Item Prob")?;

                        if let Some(group) = current_group.as_mut() {
                            group.items.push(SpecialItemDrop {
                                vnum,
                                count,
                                prob,
                            });
                        }
                    } else {
                        return Err("Invalid Item definition");
                    }
                }
            }
        }

        if in_block {
            return Err("Unexpected end of file, missing '}'");
        }

        Ok(())
    }

    pub fn get_group(&self, vnum: u32) -> Option<&SpecialItemGroup> {
        self.groups.get(&vnum)
    }

    pub fn get_all_groups(&self) -> &HashMap<u32, SpecialItemGroup> {
        &self.groups
    }
}
