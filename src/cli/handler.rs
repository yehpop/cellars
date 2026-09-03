use crate::{backend, cellar, cli::args};

/// Handler for cellars run subcommand.
pub fn run(_args: &args::RunArgs) -> Result<(), String>{
    // What happens if the cellar doesn't exist? Create it? Error out?
    // I was normally thinking construct it if the shell doesnt exist but the config does.
    // But if the config doesnt exist then we should error out.
    if !cellar::Cellar::exists(&_args.name) {
        return Err(format!("Cellar {} does not exist", _args.name));
    } // ?????
    let cellar = cellar::Cellar::load(&_args.name)?;
    backend::nix::run_cellar(&cellar)?; // Before calling this the formerly mentioned issue should be resolved.
                                        // nix run cellar will not create a shell file if non-existent, yet this is the logic i desire. (if there is a TOML)
                                        // Maybe move the cellar exist check into the nix function and handle it there? Or maybe just check for the shell.nix file and if it doesnt exist then create it from the TOML file. (if it exists)
    Ok(())
}

/// Handler for cellars create subcommand.
/// IMPORTANT rn this overwrites existing cellars. CHANGE THIS LATER.
pub fn create(_args: &args::CreateArgs) -> Result<(), String> {
    // We should validate the args

    // As of now we should create a nix shell file
    // And save the name with the new file path in a config file
    let cellar = crate::cellar::Cellar::new(&_args.name);
    cellar.save()?;
    backend::nix::write_shell(&cellar)?;

    // I didnt think of where shell files should be stored, but I guess it could be in a hidden folder in the home directory, like ~/.cellars
    // Since the environments are mainly to be used with projects they could be stored whereever the user calls cellars create 
    // but i havent looked into nix yet either.
    println!("Created environment cellar: {}", _args.name);
    if _args.run {
        // run the environment after creating it
        //backend::run() 
        backend::nix::run_cellar(&cellar)?;
    }
    Ok(())
}
/// Handler for cellars install subcommand.
/// 
/// TODO: Add --name to install from outside the environment, or use the CELLAR_ENV environment variable to install from within the environment.
pub fn install(_args: &args::InstallArgs) -> Result<(), String> {
    let cellar = std::env::var("CELLAR_ENV").map_err(|_| "CELLAR_ENV environment variable not set".to_string())?;
    let mut cellar = cellar::Cellar::load(&cellar)?;
    cellar.add_package(&_args.package);
    cellar.save()?;
    backend::nix::write_shell(&cellar)?;

    println!("Installed package {} in environment cellar: {}", _args.package, cellar.name);
    // And should i source it or just?? how does this even work idk.
    Ok(())
}
/// Handler for cellars config subcommand.
pub fn config(_args: &args::ConfigArgs) -> Result<(), String>{
    Ok(())
}

/// Handler for cellars exit subcommand.
/// 
/// do i even need a exit command? cant you just terminal exit()
/// what would this add to the program?
pub fn exit(_args: &args::ExitArgs) -> Result<(), String>{
    Ok(())
}

/// Handler for cellars kill subcommand.
pub fn kill(_args: &args::KillArgs) -> Result<(), String>{
    // Remove shell.nix file, keep config file for future quick reconstruction of the environment
    let cellar = crate::cellar::Cellar::load(&_args.name)?;
    let env_path = backend::nix::shell_path(&cellar);
    std::fs::remove_file(&env_path)
        .map_err(|e| format!("failed to delete environment: {}", e))?;

    // Garbage collect
    backend::nix::garbage_collect()?;
    println!("cleaned up cellar: {}", _args.name);
    Ok(())
}

/// Handler for cellars discard subcommand.
pub fn discard(_args: &args::DiscardArgs) -> Result<(), String>{
    // Remove the whole folder from cellars/dir
    // Before that check if the shell.nix and nix profiles still exist and.. do something.
    Ok(())
}

/// Handler for cellars list subcommand.
pub fn list(_args: &args::ListArgs) -> Result<(), String>{
    Ok(())
}


/// Probably won't need unit tests for these functions, since they are just handlers for the CLI commands. 
/// The actual logic is in the backend and cellar modules.
/// Keeping for now, in case I need add some logic here as well?
mod tests {}