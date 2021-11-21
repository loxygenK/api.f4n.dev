use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLEnum, Serialize, Deserialize)]
pub enum SkillType {
    Frontend,
    Backend,
    Infrastructure,
    Utility,
}

#[derive(juniper::GraphQLEnum, Serialize, Deserialize)]
pub enum SkilledLevel {
    Favorite,
    Advanced,
    Experienced,
    Beginner,
    Studying,
}

#[derive(juniper::GraphQLObject, Serialize, Deserialize)]
pub struct Skill {
    name: String,
    skill_type: SkillType,
    level: SkilledLevel,
    emoji: String
}

impl Skill {
    pub fn new(
        name: String,
        skill_type: SkillType,
        level: SkilledLevel,
        emoji: String
    ) -> Self {
        Self {
            name,
            skill_type,
            level,
            emoji
        }
    }
}
