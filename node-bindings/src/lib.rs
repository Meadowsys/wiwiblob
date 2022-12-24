use neon::prelude::*;

mod read;
mod wiwiblob;
mod write;

#[neon::main]
fn init(mut cx: ModuleContext) -> NeonResult<()> {
	let cx = &mut cx;

	let exports = cx.exports_object()?;

	// wiwiblob
	{
		let wiwiblob = cx.empty_object();

		let new_wiwiblob = JsFunction::new(cx, wiwiblob::new)?;
		wiwiblob.set(cx, "new_wiwiblob", new_wiwiblob)?;

		let new_wiwiblob_with_spoolsize = JsFunction::new(cx, wiwiblob::with_spoolsize)?;
		wiwiblob.set(cx, "new_wiwiblob_with_spoolsize", new_wiwiblob_with_spoolsize)?;

		let reader_builder = JsFunction::new(cx, wiwiblob::reader_builder)?;
		wiwiblob.set(cx, "reader_builder", reader_builder)?;

		let writer_builder = JsFunction::new(cx, wiwiblob::writer_builder)?;
		wiwiblob.set(cx, "writer_builder", writer_builder)?;

		let writer_builder_with_spoolsize = JsFunction::new(cx, wiwiblob::writer_builder_with_spoolsize)?;
		wiwiblob.set(cx, "writer_builder_with_spoolsize", writer_builder_with_spoolsize)?;

		exports.set(cx, "wiwiblob", wiwiblob)?;
	}

	// writer_builder
	{
		let writer_builder = cx.empty_object();

		let set_filename = JsFunction::new(cx, write::set_filename)?;
		writer_builder.set(cx, "set_filename", set_filename)?;

		let set_owner = JsFunction::new(cx, write::set_owner)?;
		writer_builder.set(cx, "set_owner", set_owner)?;

		let build = JsFunction::new(cx, write::build)?;
		writer_builder.set(cx, "build", build)?;

		exports.set(cx, "writer_builder", writer_builder)?;
	}

	Ok(())
}
