/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`test_nullable_required_param`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestNullableRequiredParamError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// 
pub fn test_nullable_required_param(configuration: &configuration::Configuration, username: &str, dummy_required_nullable_param: Option<&str>) -> Result<(), Error<TestNullableRequiredParamError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fake/user/{username}", local_var_configuration.base_path, username=crate::apis::urlencode(username));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    match dummy_required_nullable_param {
        Some(local_var_param_value) => { local_var_req_builder = local_var_req_builder.header("dummy_required_nullable_param", local_var_param_value.to_string()); },
        None => { local_var_req_builder = local_var_req_builder.header("dummy_required_nullable_param", ""); },
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<TestNullableRequiredParamError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

