use rayon::prelude::*;

use catalog_index_tools::{searches, RecordCountComparison};
fn main() {
    searches().par_iter().for_each(|search| {
        let old = search.old_count();
        let new = search.new_count();
        println!(
            "[{:?}] {} - Current prod has {}, future prod has {}",
            RecordCountComparison::compare(old, new),
            search.name(),
            old,
            new
        );
    });
}
