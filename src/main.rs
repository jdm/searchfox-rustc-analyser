extern crate rls_analysis as analysis;
#[macro_use]
extern crate clap;

mod loader;

use loader::Loader;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

type Host = analysis::AnalysisHost<Loader>;

fn collect_files(id: analysis::Id, host: &Host, files: &mut HashSet<PathBuf>) {
    each_def_from(id, host, &mut |id, def| {
        files.insert(def.span.file.clone());
    });
}

fn each_def_from<F>(id: analysis::Id, host: &Host, f: &mut F)
where
    F: FnMut(analysis::Id, &analysis::Def),
{
    let childs = host.for_each_child_def(id, |child_id, def| {
        f(child_id, def);
        child_id
    });

    if let Ok(childs) = childs {
        for child_id in childs {
            each_def_from(child_id, host, f);
        }
    }
}

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

    let roots = host.def_roots().unwrap();
    let mut files = HashSet::new();
    for &(root_id, ref name) in &roots {
        collect_files(root_id, &host, &mut files);
    }

    println!("{:?}", files);
}
