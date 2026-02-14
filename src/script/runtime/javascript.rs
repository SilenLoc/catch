use super::Run;
use actix_web::HttpRequest;
use ion::*;

pub struct JavaScriptRuntime;
impl Run for JavaScriptRuntime {
    fn run(&self, _req: HttpRequest, value: String) -> core::result::Result<String, String> {
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
        .map_err(|e| e.to_string())?;
        let worker = runtime
            .spawn_worker(JsWorkerOptions::default())
            .map_err(|e| e.to_string())?;
        let context = worker.create_context().map_err(|e| e.to_string())?;

        // Run JavaScript and return based on result
        context
            .exec_blocking(move |env| {
                let value = env.eval_script::<JsUnknown>(&value)?;

                // Catch "null" or "undefined"
                let type_repr = value.value().type_repr();
                if type_repr == "null" || type_repr == "undefined" {
                    return Ok(type_repr.to_string());
                }

                // CLARIFICATION: should string be returned as foo or "foo"
                if type_repr == "string" {
                    let s = value.cast::<JsString>()?;
                    return Ok(s.get_string()?);
                }

                // Otherwise return value as json
                let global_this = env.global_this()?;
                let json = global_this.get_named_property_unchecked::<JsObject>("JSON")?;
                let stringify = json.get_named_property_unchecked::<JsFunction>("stringify")?;
                let result: JsString = stringify.call_with_args(value)?;
                Ok(result.get_string()?)
            })
            .map_err(|e| e.to_string())
    }
}
