use std::{io::Read, sync::Arc};

use swc::{
    common::{FileName, SourceMap},
    config::{util::BoolOrObject, JsMinifyFormatOptions, JsMinifyOptions, Options},
    Compiler,
};
use swc_ecma_minifier::option::terser::TerserEcmaVersion;

/// SWC loves to generate code that relies on regenerator-runtime, but we don't want the user to
/// have to worry about that. So we'll just bundle it and slap it at the top of the output.
const REGENERATOR_RUNTIME_SOURCE: &str = include_str!("./runtime.js");

fn main() {
    let source_map = Arc::new(SourceMap::default());
    let compiler = Compiler::new(source_map.clone());

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("couldn't read javascript from stdin");

    let input = source_map.new_source_file(FileName::Anon, input);
    let minified = swc::try_with_handler(source_map.clone(), true, |handler| {
        let transpiled = compiler
            .process_js_file(input, handler, &Options::default())
            .map(|transformed| transformed.code)
            .expect("could not transpile JavaScript");
        let transpiled = source_map.new_source_file(
            FileName::Anon,
            transpiled.replace(
                "import regeneratorRuntime from \"regenerator-runtime\";",
                REGENERATOR_RUNTIME_SOURCE,
            ),
        );

        // Because we hackily insert code of regeneration-runtime, we can't do the minification in
        // the compiler pass above. So we have to do it here.
        compiler
            .minify(
                transpiled,
                handler,
                &JsMinifyOptions {
                    compress: BoolOrObject::Bool(true),
                    mangle: BoolOrObject::Bool(true),
                    format: JsMinifyFormatOptions::default(),
                    ecma: TerserEcmaVersion::default(),
                    keep_classnames: false,
                    keep_fnames: false,
                    module: false,
                    safari10: false,
                    toplevel: false,
                    source_map: BoolOrObject::Bool(false),
                    output_path: None,
                    inline_sources_content: false,
                },
            )
            .map(|transformed| transformed.code)
    })
    .expect("could not minify JavaScript");

    print!("{minified}");
}
