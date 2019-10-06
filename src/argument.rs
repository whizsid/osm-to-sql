use std::path::Path;

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

    pub help: bool
}

impl Default for Arguments {
    fn default()->Arguments {
        Arguments {
            input_file:String::from(""),
            varchar_length: 255,
            output_dir:String::from(""),
            maximum_rows_per_query: 4000,
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
    ")
}

fn show_arguments_error(e:String){
    panic!("Invalid Argument(s) : {}
    {}",e.clone(),get_arguments_help());
}

impl Arguments {

    pub fn parse_args(args:Vec<String>)->Arguments{
        let mut previous_arg:String = String::from("uninit");
        let mut formated_args = Arguments{..Default::default()};
        let empty_string:String = String::from("");

        for arg in args {
            match previous_arg.as_ref() {
                "i"=> formated_args.input_file = arg.clone(),
                "l"=> formated_args.varchar_length = arg.clone().parse().unwrap(),
                "d"=> formated_args.output_dir = arg.clone(),
                "r"=> formated_args.maximum_rows_per_query = arg.clone().parse().unwrap(),
                "h"=> formated_args.help = true,
                _=> {},
            }

            if arg=="h" {
                formated_args.help = true;
            }

            previous_arg = arg;
        }

        if formated_args.help {
            panic!("{}",get_arguments_help());
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