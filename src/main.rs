use std::path::PathBuf;

use clap::{arg, crate_version, Arg, ArgAction, ArgMatches, Command};
use forester_rs::runtime::builder::builtin::builtin_actions_file;
use forester_rs::runtime::builder::ros_nav::ros_actions_file;
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime_tree_default;
use forester_rs::simulator::builder::SimulatorBuilder;
use forester_rs::visualizer::Visualizer;
use log::LevelFilter;

#[macro_use]
extern crate log;

fn cli() -> Command {
    Command::new("f-tree")
        .about("A console utility to interact with Forester")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .version(crate_version!())
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print verbose logs")
                .global(true)
                .action(ArgAction::SetTrue)
        )
        .subcommand(Command::new("print-std-actions").about("Print the list of std actions from 'import std::actions'"))
        .subcommand(Command::new("print-ros-nav2").about("Print the list of ros actions from 'import ros::nav2'"))
        .subcommand(
            Command::new("sim")
                .about(r#"Runs simulation. Expects a simulation profile"#)
                .arg(arg!(-p --profile <PATH> "a path to a sim profile, empty by default"))
                .arg(arg!(-r --root <ROOT> "a path to a root folder. The <PWD> folder by default"))
                .arg(arg!(-m --main <MAIN> "a path to a main file. The 'main.tree' by default"))
                .arg(arg!(-t --tree <TREE> "a root in a main file. If there is only one root it takes by default"))
        )
        .subcommand(
            Command::new("vis")
                .about(r#"Runs visualization. Output is in svg format."#)
                .arg(arg!(-o --output <OUTPUT> "a file for svg. If none, the name from the main file will be taken."))
                .arg(arg!(-r --root <ROOT> "a path to a root folder. The <PWD> folder by default"))
                .arg(arg!(-m --main <MAIN> "a path to a main file. The 'main.tree' by default"))
                .arg(arg!(-t --tree <TREE> "a root in a main file. If there is only one root it takes by default"))
        )
        .subcommand(
            Command::new("nav2")
                .about(r#"Convert to the XML-compatible format of nav ROS2."#)
                .arg(arg!(-o --output <OUTPUT> "a file for xml. If none, the name from the main file will be taken."))
                .arg(arg!(-r --root <ROOT> "a path to a root folder. The <PWD> folder by default"))
                .arg(arg!(-m --main <MAIN> "a path to a main file. The 'main.tree' by default"))
                .arg(arg!(-t --tree <TREE> "a root in a main file. If there is only one root it takes by default"))
        )
}

fn buf(val: &str, relative: PathBuf) -> PathBuf {
    let path = PathBuf::from(val);
    if path.is_relative() {
        let mut full_path = relative;
        full_path.push(path);
        full_path
    } else {
        path
    }
}

fn sim(matches: &ArgMatches) {
    let pwd = std::env::current_dir().expect("the current directory is present");

    let root = match matches.get_one::<String>("root") {
        Some(root) => buf(root.as_str(), pwd),
        None => pwd,
    };

    let main_file = matches
        .get_one::<String>("main")
        .map(|v| v.to_string())
        .unwrap_or("main.tree".to_string());
    let main_tree = matches.get_one::<String>("tree");

    let mut sb = SimulatorBuilder::new();
    if let Some(p) = matches.get_one::<String>("profile") {
        let sim = buf(p, root.clone());
        sb.profile(sim);
    }
    sb.root(root.clone());
    let mut fb = ForesterBuilder::from_fs();
    fb.main_file(main_file);
    fb.root(root);

    if let Some(tree_str) = main_tree {
        fb.main_tree(tree_str.to_string())
    }

    sb.forester_builder(fb);

    match sb.build() {
        Ok(mut s) => match s.run() {
            Ok(r) => {
                info!("the process is finished with the result: {:?}", r)
            }
            Err(err) => {
                error!("a runtime error occurred: {:?}", err)
            }
        },
        Err(err) => {
            error!("a build error occurred: {:?}", err)
        }
    }
}

fn viz(matches: &ArgMatches) {
    let pwd = std::env::current_dir().expect("the current directory is present");

    let root = match matches.get_one::<String>("root") {
        Some(root) => buf(root.as_str(), pwd),
        None => pwd,
    };

    match Visualizer::project_svg_to_file(
        root,
        matches.get_one::<String>("main"),
        matches.get_one::<String>("tree"),
        matches.get_one::<String>("output"),
    ) {
        Ok(_) => {
            info!("the result is successfully saved to the given file.")
        }
        Err(e) => {
            error!("the visualization failed due to '{:?}'", e);
        }
    }
}

fn export_to_nav(matches: &ArgMatches) {
    let pwd = std::env::current_dir().expect("the current directory is present");

    let root = match matches.get_one::<String>("root") {
        Some(root) => buf(root.as_str(), pwd),
        None => pwd,
    };

    let (rts, output) = runtime_tree_default(
        root,
        matches.get_one::<String>("main"),
        matches.get_one::<String>("tree"),
        matches.get_one::<String>("output"),
        "xml".to_string(),
    )
    .map_err(|e| {
        error!("the export failed due to '{:?}'", e);
    })
    .expect("the runtime tree is built");

    match rts.tree.to_ros_nav(output) {
        Ok(_) => {
            info!("the result is successfully saved to the given file.")
        }
        Err(e) => {
            error!("the export failed due to '{:?}'", e);
        }
    }
}

fn std() {
    let f = builtin_actions_file();
    info!("{f}");
}

fn ros_nav2() {
    let f = ros_actions_file();
    info!("{f}");
}

fn main() {
    let matches = cli().get_matches();

    let mut log_builder = env_logger::builder();

    log_builder.is_test(false);
    if matches.get_flag("verbose") {
        log_builder.filter_level(LevelFilter::max());
    }

    let _ = log_builder.try_init();

    match matches.subcommand() {
        Some(("sim", args)) => {
            sim(args);
        }
        Some(("vis", args)) => {
            viz(args);
        }
        Some(("print-std-actions", _)) => {
            std();
        }
        Some(("print-ros-nav2", _)) => {
            ros_nav2();
        }
        Some(("nav2", args)) => {
            export_to_nav(args);
        }
        Some((e, _)) => {
            error!("the command '{e}' does not match any expected command.");
        }
        None => {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_exposes_expected_subcommands() {
        let cmd = cli();
        let names: Vec<&str> = cmd.get_subcommands().map(|s| s.get_name()).collect();
        for expected in ["sim", "vis", "nav2", "print-std-actions", "print-ros-nav2"] {
            assert!(names.contains(&expected), "missing subcommand '{expected}'");
        }
        assert_eq!(names.len(), 5);
    }

    #[test]
    fn cli_requires_a_subcommand() {
        assert!(cli().is_subcommand_required_set());
        assert!(cli().is_arg_required_else_help_set());
    }

    #[test]
    fn cli_version_matches_cargo() {
        assert_eq!(cli().get_version(), Some(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn verbose_flag_is_parsed() {
        let matches = cli().get_matches_from(["f-tree", "-v", "print-std-actions"]);
        assert!(matches.get_flag("verbose"));
    }

    #[test]
    fn verbose_flag_works_after_subcommand() {
        let matches = cli().get_matches_from(["f-tree", "print-std-actions", "-v"]);
        assert!(matches.get_flag("verbose"));
    }

    #[test]
    fn verbose_flag_defaults_to_false() {
        let matches = cli().get_matches_from(["f-tree", "print-std-actions"]);
        assert!(!matches.get_flag("verbose"));
    }

    #[test]
    fn sim_parses_all_arguments() {
        let matches = cli().get_matches_from([
            "f-tree", "sim",
            "-p", "profile.toml",
            "-r", "/root",
            "-m", "other.tree",
            "-t", "main_root",
        ]);
        let (name, args) = matches.subcommand().unwrap();
        assert_eq!(name, "sim");
        assert_eq!(args.get_one::<String>("profile").unwrap(), "profile.toml");
        assert_eq!(args.get_one::<String>("root").unwrap(), "/root");
        assert_eq!(args.get_one::<String>("main").unwrap(), "other.tree");
        assert_eq!(args.get_one::<String>("tree").unwrap(), "main_root");
    }

    #[test]
    fn sim_arguments_are_optional() {
        let matches = cli().get_matches_from(["f-tree", "sim"]);
        let (_, args) = matches.subcommand().unwrap();
        assert!(args.get_one::<String>("profile").is_none());
        assert!(args.get_one::<String>("root").is_none());
        assert!(args.get_one::<String>("main").is_none());
        assert!(args.get_one::<String>("tree").is_none());
    }

    #[test]
    fn vis_parses_all_arguments() {
        let matches = cli().get_matches_from([
            "f-tree", "vis",
            "-o", "out.svg",
            "-r", "/root",
            "-m", "other.tree",
            "-t", "main_root",
        ]);
        let (name, args) = matches.subcommand().unwrap();
        assert_eq!(name, "vis");
        assert_eq!(args.get_one::<String>("output").unwrap(), "out.svg");
        assert_eq!(args.get_one::<String>("root").unwrap(), "/root");
        assert_eq!(args.get_one::<String>("main").unwrap(), "other.tree");
        assert_eq!(args.get_one::<String>("tree").unwrap(), "main_root");
    }

    #[test]
    fn nav2_parses_all_arguments() {
        let matches = cli().get_matches_from([
            "f-tree", "nav2",
            "-o", "out.xml",
            "-r", "/root",
            "-m", "other.tree",
            "-t", "main_root",
        ]);
        let (name, args) = matches.subcommand().unwrap();
        assert_eq!(name, "nav2");
        assert_eq!(args.get_one::<String>("output").unwrap(), "out.xml");
        assert_eq!(args.get_one::<String>("root").unwrap(), "/root");
        assert_eq!(args.get_one::<String>("main").unwrap(), "other.tree");
        assert_eq!(args.get_one::<String>("tree").unwrap(), "main_root");
    }

    #[test]
    fn print_subcommands_parse_without_arguments() {
        for sub in ["print-std-actions", "print-ros-nav2"] {
            let matches = cli().get_matches_from(["f-tree", sub]);
            let (name, _) = matches.subcommand().unwrap();
            assert_eq!(name, sub);
        }
    }

    #[test]
    fn buf_resolves_relative_path_against_base() {
        assert_eq!(
            buf("sub/dir", PathBuf::from("/base")),
            PathBuf::from("/base/sub/dir")
        );
    }

    #[test]
    fn buf_keeps_absolute_path_unchanged() {
        assert_eq!(
            buf("/abs/dir", PathBuf::from("/base")),
            PathBuf::from("/abs/dir")
        );
    }
}
