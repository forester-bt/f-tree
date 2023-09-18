use std::path::PathBuf;

use clap::{arg, value_parser, Arg, ArgAction, ArgMatches, Command};
use forester_rs::runtime::action::Tick;
use forester_rs::runtime::builder::builtin::{builtin_actions_file};
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime::builder::ros_nav::ros_actions_file;
use forester_rs::runtime::RtResult;
use forester_rs::runtime_tree_default;
use forester_rs::simulator::builder::SimulatorBuilder;
use forester_rs::tree::TreeError;
use forester_rs::visualizer::Visualizer;
use log::LevelFilter;

#[macro_use]
extern crate log;

fn cli() -> Command {
    Command::new("f-tree")
        .about("A console utility to interact with Forester")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .version("0.2.2")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Print debug logs")
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
                .arg(arg!(-o --output <OUTPUT> "a file for svg. If  no, the name from the main file will be taken."))
                .arg(arg!(-r --root <ROOT> "a path to a root folder. The <PWD> folder by default"))
                .arg(arg!(-m --main <MAIN> "a path to a main file. The 'main.tree' by default"))
                .arg(arg!(-t --tree <TREE> "a root in a main file. If there is only one root it takes by default"))
        )
        .subcommand(
            Command::new("nav2")
                .about(r#"Convert to the xml compatable format of nav ros2."#)
                .arg(arg!(-o --output <OUTPUT> "a file for xml. If  no, the name from the main file will be taken."))
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
    let pwd = std::env::current_dir().expect("the current directory is presented");

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

    if main_tree.is_some() {
        fb.main_tree(main_tree.unwrap().to_string())
    }

    sb.forester_builder(fb);

    match sb.build() {
        Ok(mut s) => match s.run() {
            Ok(r) => {
                info!("the process is finished with the result: {:?}", r)
            }
            Err(err) => {
                error!("the runtime error occurred : {:?}", err)
            }
        },
        Err(err) => {
            error!("the building error occurred: {:?}", err)
        }
    }
}

fn viz(matches: &ArgMatches) {
    let pwd = std::env::current_dir().expect("the current directory is presented");

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
            error!("the visualization is failed due to '{:?}'", e);
        }
    }
}

fn export_to_nav(matches: &ArgMatches) {
    let pwd = std::env::current_dir().expect("the current directory is presented");

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
    ).map_err(|e|{
        error!("the export is failed due to '{:?}'", e);
    }).expect("the runtime tree is built");

    match rts.tree.to_ros_nav(output) {
        Ok(_) => {
            info!("the result is successfully saved to the given file.")
        }
        Err(e) => {
            error!("the export is failed due to '{:?}'", e);
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
    if matches.get_flag("debug") {
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
            error!("the command '{e}' does not match the expected commands. ");
        }
        None => {
            unreachable!();
        }
    }
}
