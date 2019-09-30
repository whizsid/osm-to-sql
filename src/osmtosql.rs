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

    // All 9 tables dump to this sql file if you use this parameter.
    pub output_file: String,

    // Maximum rows per one insert query.
    pub maximum_rows_per_query: i32
}

impl Default for Arguments {
    fn default()->Arguments {
        Arguments {
            input_file:String::from(""),
            varchar_length: 255,
            output_dir:String::from(""),
            output_file: String::from(""),
            maximum_rows_per_query: 4000
        }
    }
}

impl Arguments {

    pub fn error(e:String){
        eprintln!("{}",e.clone());
    }

    pub fn parse_args(args:Vec<String>){
        let mut previous_arg:String = String::from("unintialized");
        let mut formated_args = Arguments{..Default::default()};

        for arg in args {
            match previous_arg.as_ref() {
                "-i"=> formated_args.input_file = arg.clone(),
                "-l"=> formated_args.varchar_length = arg.clone().parse().unwrap(),
                "-d"=> formated_args.output_dir = arg.clone(),
                "-f"=> formated_args.output_file = arg.clone(),
                "-r"=> formated_args.maximum_rows_per_query = arg.clone().parse().unwrap(),
                _=> {},
            }

            previous_arg = arg;
        }
    }
}