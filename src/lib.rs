use lalrpop_util::lalrpop_mod;

lalrpop_mod!(lef);
lalrpop_mod!(def);

mod common_handler;
mod def_handler;
mod lef_handler;

pub use crate::{
    def_handler::{read_def, DEF},
    lef_handler::{read_lef, LEF},
};
