#[derive(juniper::GraphQLEnum)]
pub enum SkillType {
    Frontend,
    Backend,
    Mobile,
    Infrastructure,
    Miscellaneous,
}

#[derive(juniper::GraphQLEnum)]
pub enum SkilledLevel {
    NoLongerUsed,
    Advanced,
    Available,
    Fundamental,
    Studying,
}

#[derive(juniper::GraphQLObject)]
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
