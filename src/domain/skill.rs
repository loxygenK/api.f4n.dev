use serde::{Serialize, Deserialize};

#[derive(juniper::GraphQLEnum, Serialize, Deserialize)]
pub enum SkillType {
    Frontend,
    Backend,
    Mobile,
    Infrastructure,
    Miscellaneous,
}

#[derive(juniper::GraphQLEnum, Serialize, Deserialize)]
pub enum SkilledLevel {
    NoLongerUsed,
    Advanced,
    Available,
    Fundamental,
    Studying,
}

#[derive(juniper::GraphQLObject, Serialize, Deserialize)]
pub struct Skill {
    name: String,
    skill_type: SkillType,
    level: SkilledLevel,
    fa_icon_identifier: String,
}

impl Skill {
    pub fn new(
        name: String,
        skill_type: SkillType,
        level: SkilledLevel,
        fa_icon_identifier: String,
    ) -> Self {
        Self {
            name,
            skill_type,
            level,
            fa_icon_identifier,
        }
    }
}
