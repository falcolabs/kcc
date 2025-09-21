/**
 * Kat Compiler Collection
 * Copyright (C) 2025  Tri Phuong Nguyen
 *
 * This program is free software. It comes without any warranty,
 * to the extent permitted by applicable law. You can redistribute
 * it and/or modify it under the terms of the GNU General Public
 * License, version 3, or at your option (required if you want to
 * intergrate it into a proprietary product), the DORAEMON IS THE
 * BEST ANIME PUBLIC LICENSE, version 1 (see LICENSE_DORAEMON).
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License or the DORAEMON IS THE BEST ANIME
 * PUBLIC LICENSE for enforcable terms.
 *
 * You should have received a copy of the DORAEMON IS THE BEST
 * ANIME PUBLIC LICENSE along with this program.  If not, see
 * <https://github.com/falcolabs/kcc/blob/main/LICENSE_DORAEMON/>.
 *
 * You should have also received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
pub mod vm;
use mimalloc::MiMalloc;
use std::fs::File;

use log::{debug, error};
pub use scratch_ast::parser::load_from_directory;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    pretty_env_logger::init();
    if args.len() < 2 {
        error!("no file specified");
        std::process::exit(1);
    }
    let temp_dir = tempfile::tempdir()
        .expect("failed to create a temporary directory to extract project contents");
    if !std::fs::exists(&args[1]).unwrap() {
        error!("file {} does not exist", &args[1]);
        std::process::exit(1);
    }
    let project_file = File::open(&args[1]).unwrap();
    zip::ZipArchive::new(project_file)
        .unwrap()
        .extract(temp_dir.path())
        .unwrap();
    let prj = load_from_directory(temp_dir.path()).expect("unable to load project");
    debug!("Parsing completed, starting execution");
    vm::run(prj.into());
}
