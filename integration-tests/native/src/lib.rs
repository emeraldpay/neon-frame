#[macro_use]
extern crate neon_frame_macro;
#[macro_use]
extern crate serde_derive;

mod examples;

use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("no_channel", examples::function_no_channel)?;
  cx.export_function("complex_data", examples::function_complex_data)?;

  cx.export_function("default_channel", examples::function_default_channel)?;
  cx.export_function("channel_at_1", examples::function_channel_at_1)?;

  cx.export_function("immediate_err", examples::function_immediate_err)?;
  cx.export_function("handler_err", examples::function_handler_err)?;
  cx.export_function("no_channel_err", examples::function_no_channel_err)?;

  cx.export_function("always_panic", examples::function_panic)?;

  Ok(())
}
