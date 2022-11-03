use std::collections::HashMap;
use std::fmt::Display;

pub struct DataPack {
    pub pub_namespace: NameSpace,
    pub mc_namespace: NameSpace,
}

impl DataPack {
    pub fn new() -> DataPack {
        DataPack {
            pub_namespace: NameSpace::new("pub"),
            mc_namespace: NameSpace::new("minecraft"),
        }
    }
}

pub struct NameSpace {
    pub id: String,
    pub functions: HashMap<String, MCFunction>,
}

impl NameSpace {
    pub fn new(id: &str) -> NameSpace {
        NameSpace {
            id: id.to_owned(),
            functions: HashMap::new(),
        }
    }
}

pub struct MCFunction {
    pub filename: String,
    pub commands: Vec<Command>,
}

impl MCFunction {
    pub fn new(name: &str) -> MCFunction {
        MCFunction {
            filename: name.to_owned(),
            commands: vec![],
        }
    }
}

impl Display for MCFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for command in &self.commands {
            f.write_fmt(format_args!("{}\n", command))?;
        }
        write!(f, "")
    }
}

pub enum Command {
    Scoreboard { command: ScoreboardCommand },
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Scoreboard { command } => write!(f, "{}", command),
        }
    }
}

pub enum ScoreboardCommand {
    ObjectivesAdd {
        id: String,
        criteria: ObjectiveCriteria,
        name: Option<String>,
    },
    PlayersSet {
        target: CommandTarget,
        objective: String,
        score: i32,
    },
    PlayersAdd {
        target: CommandTarget,
        objective: String,
        to_add: i32,
    },
    PlayersRemove {
        target: CommandTarget,
        objective: String,
        to_remove: i32,
    },
    PlayersOperation {
        target: CommandTarget,
        objective: String,
        operation: ScoreboardOperation,
        source: CommandTarget,
        source_objective: String,
    },
}

impl Display for ScoreboardCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreboardCommand::ObjectivesAdd { id, criteria, name } => match name {
                Some(n) => write!(f, "scoreboard objectives add {} {} {}", id, criteria, n),
                None => write!(f, "scoreboard objectives add {} {}", id, criteria),
            },
            ScoreboardCommand::PlayersSet {
                target,
                objective,
                score,
            } => write!(
                f,
                "scoreboard players set {} {} {}",
                target, objective, score
            ),
            ScoreboardCommand::PlayersAdd {
                target,
                objective,
                to_add,
            } => write!(
                f,
                "scoreboard players add {} {} {}",
                target, objective, to_add
            ),
            ScoreboardCommand::PlayersRemove {
                target,
                objective,
                to_remove,
            } => write!(
                f,
                "scoreboard players remove {} {} {}",
                target, objective, to_remove
            ),
            ScoreboardCommand::PlayersOperation {
                target,
                objective,
                operation,
                source,
                source_objective,
            } => write!(
                f,
                "scoreboard players operation {} {} {} {} {}",
                target, objective, operation, source, source_objective
            ),
        }
    }
}

pub enum ObjectiveCriteria {
    Dummy,
}

impl Display for ObjectiveCriteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectiveCriteria::Dummy => write!(f, "dummy"),
        }
    }
}

pub enum ScoreboardOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Assign,
    Min,
    Max,
    Swap,
}

impl Display for ScoreboardOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreboardOperation::Addition => write!(f, "+="),
            ScoreboardOperation::Subtraction => write!(f, "-="),
            ScoreboardOperation::Multiplication => write!(f, "*="),
            ScoreboardOperation::Division => write!(f, "/="),
            ScoreboardOperation::Modulo => write!(f, "%="),
            ScoreboardOperation::Assign => write!(f, "="),
            ScoreboardOperation::Min => write!(f, "<"),
            ScoreboardOperation::Max => write!(f, ">"),
            ScoreboardOperation::Swap => write!(f, "><"),
        }
    }
}

pub enum CommandTarget {
    Name { name: String },
}

impl Display for CommandTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandTarget::Name { name } => write!(f, "{}", name),
        }
    }
}
