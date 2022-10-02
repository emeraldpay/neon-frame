use neon::prelude::{FunctionContext, JsString};

#[derive(Debug)]
pub enum Errors {
    ErrorOne,
    ErrorTwo,

    ErrorReal(String),
}

#[derive(Serialize, Clone)]
pub struct ComplexResult {
    pub foo: String,
    pub bar: usize
}

impl From<Errors> for (usize, String) {

    fn from(e: Errors) -> Self {
        match e {
            Errors::ErrorOne => (1, "Error One".to_string()),
            Errors::ErrorTwo => (2, "Error Two".to_string()),
            Errors::ErrorReal(msg) => (100, format!("Real error: {}", msg)),
        }
    }
}


#[neon_frame_fn]
pub fn function_no_channel(_cx: &mut FunctionContext) -> Result<String, Errors> {
    Ok("Hello World".to_string())
}

#[neon_frame_fn]
pub fn function_complex_data(cx: &mut FunctionContext) -> Result<ComplexResult, Errors> {

    let arg_0 = cx
        .argument::<JsString>(0)
        .map_err(|_| Errors::ErrorReal("Argument 0 is missing".to_string()))?
        .value(cx);

    Ok(ComplexResult {
        bar: arg_0.len(),
        foo: arg_0,
    })
}

#[neon_frame_fn]
pub fn function_no_channel_err(_cx: &mut FunctionContext) -> Result<String, Errors> {
    Err(Errors::ErrorOne)
}

#[neon_frame_fn(channel)]
pub fn function_default_channel<H>(_cx: &mut FunctionContext, handler: H) -> Result<(), Errors>
    where
        H: FnOnce(Result<String, Errors>) + Send + 'static {

    std::thread::spawn(move || {
        let result = "Just Working Function".to_string();
        handler(Ok(result));
    });

    Ok(())
}

#[neon_frame_fn(channel=1)]
pub fn function_channel_at_1<H>(cx: &mut FunctionContext, handler: H) -> Result<(), Errors>
    where
        H: FnOnce(Result<String, Errors>) + Send + 'static {

    let arg_0 = cx
        .argument::<JsString>(0)
        .map_err(|_| Errors::ErrorReal("Argument 0 is missing".to_string()))?
        .value(cx);

    std::thread::spawn(move || {
        let result = format!("Accept handler at pos 1. Argument: {}", arg_0);
        handler(Ok(result));
    });

    Ok(())
}

#[neon_frame_fn(channel=0)]
pub fn function_immediate_err<H>(_cx: &mut FunctionContext, _handler: H) -> Result<(), Errors>
    where
        H: FnOnce(Result<String, Errors>) + Send + 'static {

    Err(Errors::ErrorOne)
}

#[neon_frame_fn(channel=0)]
pub fn function_handler_err<H>(_cx: &mut FunctionContext, handler: H) -> Result<(), Errors>
    where
        H: FnOnce(Result<String, Errors>) + Send + 'static {

    std::thread::spawn(move || {
        handler(Err(Errors::ErrorOne));
    });

    Ok(())
}


#[neon_frame_fn(channel=0)]
pub fn function_panic<H>(_cx: &mut FunctionContext, _handler: H) -> Result<(), Errors>
    where
        H: FnOnce(Result<String, Errors>) + Send + 'static {

    panic!("Just Panic")
}