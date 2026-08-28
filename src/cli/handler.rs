use crate::{backend, backend::nix, cli::args};

pub fn run(_args: &args::RunArgs) -> Result<(), String>{
    Ok(())
}

pub fn create(_args: &args::CreateArgs) -> Result<(), String> {
    // We should validate the args

    // As of now we should create a nix shell file

    // And save the name with the new file path in a config file

    // I didnt think of where shell files should be stored, but I guess it could be in a hidden folder in the home directory, like ~/.cellars
    // Since the environments are mainly to be used with projects they could be stored whereever the user calls cellars create 
    // but i havent looked into nix yet either.

    if _args.run {
        // run the environment after creating it
        backend::run() 
    }
    Ok(())
}

pub fn config(_args: &args::ConfigArgs) -> Result<(), String>{
    Ok(())
}

pub fn exit(_args: &args::ExitArgs) -> Result<(), String>{
    Ok(())
}

pub fn kill(_args: &args::KillArgs) -> Result<(), String>{
    Ok(())
}

pub fn discard(_args: &args::DiscardArgs) -> Result<(), String>{
    Ok(())
}

pub fn list(_args: &args::ListArgs) -> Result<(), String>{
    Ok(())
}