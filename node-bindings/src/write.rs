use neon::prelude::*;
use neon::types::buffer::TypedArray;
use std::cell::RefCell;
use std::io::Write;
use wiwibloblib::write::Writer as RawWriter;
use wiwibloblib::write::WriterBuilder as RawWriterBuilder;

pub struct WriterBuilder {
	pub inner: RefCell<RawWriterBuilder>
}

impl Finalize for WriterBuilder {}

pub struct Writer {
	pub inner: RefCell<Option<RawWriter>>
}

impl Finalize for Writer {}

pub fn set_filename(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let cx = &mut cx;

	let writer_builder = cx.argument::<JsBox<WriterBuilder>>(0)?;
	let mut writer_builder = writer_builder.inner.borrow_mut();
	let filename = cx.argument::<JsString>(1)?.value(cx);

	match writer_builder.set_filename(filename) {
		Ok(_) => { Ok(cx.undefined()) }
		Err(e) => {
			let e = cx.error(e.to_string())?;
			cx.throw(e)
		}
	}
}

pub fn set_owner(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let cx = &mut cx;

	let writer_builder = cx.argument::<JsBox<WriterBuilder>>(0)?;
	let mut writer_builder = writer_builder.inner.borrow_mut();
	let owner = cx.argument::<JsString>(1)?.value(cx);

	match writer_builder.set_owner(owner) {
		Ok(_) => { Ok(cx.undefined()) }
		Err(e) => {
			let e = cx.error(e.to_string())?;
			cx.throw(e)
		}
	}
}

pub fn set_other_meta(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let cx = &mut cx;

	let writer_builder = cx.argument::<JsBox<WriterBuilder>>(0)?;
	let mut writer_builder = writer_builder.inner.borrow_mut();

	let k = cx.argument::<JsString>(1)?.value(cx);
	let v = cx.argument::<JsString>(2)?.value(cx);

	writer_builder.set_other_meta(k, v);
	Ok(cx.undefined())
}

pub fn build(mut cx: FunctionContext) -> JsResult<JsPromise> {
	let cx = &mut cx;

	let writer_builder = cx.argument::<JsBox<WriterBuilder>>(0)?
		.inner
		.borrow()
		.clone();

	let promise = cx.task(|| writer_builder.build())
		.promise(|mut cx, res| {
			let writer = match res {
				Ok(writer) => { writer }
				Err(e) => {
					let e = cx.error(e.to_string())?;
					cx.throw(e)?
				}
			};

			Ok(cx.boxed(Writer {
				inner: RefCell::new(Some(writer))
			}))
		});

	Ok(promise)
}

pub fn write_all(mut cx: FunctionContext) -> JsResult<JsPromise> {
	let cx = &mut cx;

	let writer_arg = cx.argument::<JsBox<Writer>>(0)?;
	let mut writer_opt = writer_arg.inner.borrow_mut();
	let mut writer = writer_opt.take().unwrap();
	drop(writer_opt);
	let writer_arg = writer_arg.root(cx);
	let buf = cx.argument::<JsBuffer>(1)?.as_slice(cx).to_vec();

	let promise = cx.task(move || {
		let res = writer.write_all(&buf);
		(res, writer)
	}).promise(|mut cx, (res, writer)| {
		let cx = &mut cx;

		match res {
			Ok(_) => {
				let writer_arg = writer_arg.into_inner(cx);
				let mut writer_opt = writer_arg.inner.borrow_mut();
				*writer_opt = Some(writer);
				Ok(cx.undefined())
			}
			Err(e) => {
				let e = cx.error(e.to_string())?;
				cx.throw(e)?
			}
		}
	});

	Ok(promise)
}

pub fn finish(mut cx: FunctionContext) -> JsResult<JsPromise> {
	let cx = &mut cx;

	let writer = cx.argument::<JsBox<Writer>>(0)?;
	let mut writer_opt = writer.inner.borrow_mut();
	let writer = writer_opt.take().unwrap();

	let promise = cx.task(|| writer.finish())
		.promise(|mut cx, res| {
			let cx = &mut cx;

			match res {
				Ok(hash) => { Ok(cx.string(hash)) }
				Err(e) => {
					let e = cx.error(e.to_string())?;
					cx.throw(e)?
				}
			}
		});

	Ok(promise)
}
