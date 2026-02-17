use super::RuntimeError;
use ion::{
    JsFunction, JsObject, JsObjectValue, JsRuntime, JsRuntimeOptions, JsString, JsUnknown, JsValue,
    JsWorkerOptions,
};

pub fn run(script: String) -> Result<String, RuntimeError> {
    // Initialize JavaScript runtime & context
    let runtime = JsRuntime::initialize_once(JsRuntimeOptions {
        transformers: vec![
            ion::transformers::json(),
            // ion::transformers::ts(),
            // ion::transformers::tsx(),
        ],
        extensions: vec![
            ion::extensions::event_target(),
            ion::extensions::console(),
            ion::extensions::set_timeout(),
            ion::extensions::set_interval(),
            ion::extensions::test(),
            ion::extensions::global_this(),
        ],
        ..Default::default()
    })
    .map_err(|_| {
        RuntimeError::InternalError("JavaScript Runtime initialization failed".to_owned())
    })?;
    let worker = runtime
        .spawn_worker(JsWorkerOptions::default())
        .map_err(|_| {
            RuntimeError::InternalError("JavaScript Worker initialization failed".to_owned())
        })?;
    let context = worker.create_context().map_err(|_| {
        RuntimeError::InternalError("JavaScript Context initialization failed".to_owned())
    })?;

    // Run JavaScript and return based on result
    context
        .exec_blocking(move |env| {
            let value = env.eval_script::<JsUnknown>(&script)?;

            // Catch "null" or "undefined"
            let type_repr = value.value().type_repr();
            if type_repr == "null" || type_repr == "undefined" {
                return Ok(type_repr.to_string());
            }

            // Catch "string"
            if type_repr == "string" {
                let s = value.cast::<JsString>()?;
                return s.get_string();
            }

            // Otherwise return value as json
            let global_this = env.global_this()?;
            let json = global_this.get_named_property_unchecked::<JsObject>("JSON")?;
            let stringify = json.get_named_property_unchecked::<JsFunction>("stringify")?;
            let result: JsString = stringify.call_with_args(value)?;
            result.get_string()
        })
        .map_err(|e| RuntimeError::UserError(e.to_string()))
}
