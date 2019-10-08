use std::path::Path;
use std::process;

#[derive(Debug, Clone)]
pub struct Arguments {
    // XML file downloaded from geofabrick
    pub input_file: String,
    // Varchar length for tag values and names
    pub varchar_length: i32,
    // Path to store sql files
    // osm-to-sql command will export xml file to 9 different files if used output_folder parameter
    // 1. nodes
    // 2. tags
    // 3. node_tags
    // 4. ways
    // 5. way_nodes
    // 6. way_tags
    // 7. relations
    // 8. relation_members
    // 9. relation_tags

    // If you want to export all tables to a single SQL file, use output_dir parameter
    pub output_dir: String,

    // Maximum rows per one insert query.
    pub maximum_rows_per_query: i32,

    // Not allowing INSERT IGNORE queries.
    // We can find some duplicate entries in OSM XML files. When we inserting these
    // queries to our database some errors throwing and whole query is not inserting.
    // We are using INSERT IGNORE queries by default to avoid these type of errors.
    pub insert_ignore: bool,

    // Display the help informations
    pub help: bool
}

impl Default for Arguments {
    fn default()->Arguments {
        Arguments {
            input_file:String::from(""),
            varchar_length: 255,
            output_dir:String::from(""),
            maximum_rows_per_query: 4000,
            insert_ignore: true,
            help: false
        }
    }
}

fn get_arguments_help()->String{
    String::from("
Usage:
    osm-to-sql [OPTIONS] -i <xml_file_path.xml> -d <output_directory>

OPTIONS:
    -i        Input open street map file in XML format.
    -l        Default varchar length to using in table creation. [250]
    -d        Output directory to save output sql files.
    -r        Maximum rows per one SQL insert query. [400]
    -h        Prints help information
    -g        Do not use INSERT IGNORE queries
    ")
}

fn show_arguments_error(e:String){
    eprintln!("Invalid Argument(s) : {}
    {}",e.clone(),get_arguments_help());
    process::exit(128);
}

impl Arguments {

    pub fn parse_args(args:Vec<String>)->Arguments{
        let mut previous_arg:String = String::from("uninit");
        let mut formated_args = Arguments{..Default::default()};
        let empty_string:String = String::from("");

        for arg in args {
            match previous_arg.as_ref() {
                "-i"=> formated_args.input_file = arg.clone(),
                "-l"=> formated_args.varchar_length = arg.clone().parse().unwrap(),
                "-d"=> formated_args.output_dir = arg.clone(),
                "-r"=> formated_args.maximum_rows_per_query = arg.clone().parse().unwrap(),
                "-h"=> formated_args.help = true,
                "-g"=> formated_args.insert_ignore = false,
                _=> {},
            }

            if arg=="-h" {
                formated_args.help = true;
            }

            if arg=="-g" {
                formated_args.insert_ignore = false;
            }

            previous_arg = arg;
        }

        if formated_args.help {
            println!("{}",get_arguments_help());
            process::exit(0);
        }

        if formated_args.output_dir!=empty_string && !Path::new(&formated_args.output_dir).exists() {
            show_arguments_error(String::from("Please check the output directory is exist."));
        }

        if  formated_args.output_dir == empty_string {
            show_arguments_error(String::from("Output directory is required."));
        }

        if formated_args.input_file == empty_string {
            show_arguments_error(String::from("Input file argument is required."));
        }

        if !Path::new(&formated_args.input_file).exists() {
            show_arguments_error(String::from("Input file path is not exists!"));
        }

        return formated_args;
    }
}