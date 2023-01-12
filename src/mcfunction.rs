use std::fmt::Display;

pub struct MCFunction {
    commands: Vec<Command>,
}

impl MCFunction {
    pub fn new() -> MCFunction {
        MCFunction { commands: vec![] }
    }

    pub fn new_command(&mut self, command: Command) {
        self.commands.push(command)
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
    Function { function: MCFunctionID },
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Scoreboard { command } => write!(f, "{}", command),
            Command::Function { function } => write!(f, "function {}", function),
        }
    }
}

pub enum ScoreboardCommand {
    ObjectivesAdd {
        id: String,
        criteria: ObjectiveCriteria,
        name: Option<String>,
    },
    ObjectivesRemove {
        id: String,
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
    PlayersReset {
        target: CommandTarget,
        objective: Option<String>,
    },
}

impl From<ScoreboardCommand> for Command {
    fn from(command: ScoreboardCommand) -> Self {
        Command::Scoreboard { command }
    }
}

impl Display for ScoreboardCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreboardCommand::ObjectivesAdd { id, criteria, name } => match name {
                Some(n) => write!(f, "scoreboard objectives add {} {} {}", id, criteria, n),
                None => write!(f, "scoreboard objectives add {} {}", id, criteria),
            },
            ScoreboardCommand::ObjectivesRemove { id } => {
                write!(f, "scoreboard objectives remove {}", id)
            }
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
            ScoreboardCommand::PlayersReset { target, objective } => match objective {
                Some(o) => write!(f, "scoreboard players reset {} {}", target, o),
                None => write!(f, "scoreboard players reset {}", target),
            },
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

pub struct MCFunctionID {
    pub namespace: String,
    pub path: Vec<String>,
}

impl Display for MCFunctionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path.join("/"))
    }
}
