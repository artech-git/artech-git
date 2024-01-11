use crate::error::BackendResult;

const LISTED_COMMANDS: [&'static str; 1] = [".dbinfo"];

#[derive(Debug)]
enum DBCommands { 
    dbinfo = 1 , 
}

impl DBCommands {
    
    fn from_str(command: &str) -> BackendResult<Self> { 
        for (idx, recoginzed_command) in LISTED_COMMANDS.iter().enumerate() {
            if command == *recoginzed_command {
                return Ok(DBCommands::from_u8(idx as u8)?);
            }
        }

        return Err(CommandErrors::InvalidCommand.into()); 
    }

    fn from_u8(val: u8) -> BackendResult<Self> {
        let i = match val {
            1 => Self::dbinfo, 
            _ => { return Err(CommandErrors::InvalidInput.into()); }
        };

        Ok(i)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandErrors {
    #[error("Invalid database command")]
    InvalidCommand,

    #[error("Field not found for command")]
    InvalidInput,
}