pub use az_app_variables::*;

#[derive(AzAppVariablesNew, AzAppVariablesInit, Debug)]
pub struct AppVariables {
    pub azure_keyvault_name: String,
}
