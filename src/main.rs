extern crate rls_analysis as analysis;
#[macro_use]
extern crate clap;

mod loader;

use loader::Loader;
use std::path::{Path, PathBuf};

fn main() {
    let matches = app_from_crate!()
        .args_from_usage(
            "<src>    'Points to the source root'
             <input>  'Points to the deps/save-analysis directory'
             <output> 'Points to the directory where searchfox metadata should'"
        )
        .get_matches();

    let src_dir = Path::new(matches.value_of("src").unwrap());
    let input_dir = Path::new(matches.value_of("input").unwrap());
    let output_dir = Path::new(matches.value_of("output").unwrap());

    let loader = Loader::new(PathBuf::from(input_dir));


    if false {
        let crates = analysis::read_analysis_from_files(
            &loader,
            Default::default(),
            &[],
        );

        println!("{:?}", crates);
    }

    let host = analysis::AnalysisHost::new_with_loader(loader);
    host.reload(src_dir.clone(), src_dir.clone()).unwrap();

    let file = src_dir.join("src/gl_context.rs");
    println!("{:?}", file.display());
    let symbols = host.symbols(&file).unwrap();
    println!("{:?}", symbols);
}
