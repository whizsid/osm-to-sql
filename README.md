# osm-to-sql

---
![AUR version](https://img.shields.io/aur/version/osm-to-sql)
![GitHub](https://img.shields.io/github/license/whizsid/osm-to-sql)
![Travis CI](https://travis-ci.org/whizsid/osm-to-sql.svg?branch=master)

---

Simple OSM XML data to SQL converter command. This command will create 7 SQL files with foreign key constraints. You can use these SQL files with MySQL/ SQLite or any other SQL server.

## Installation

Currently only supporting to linux distros.

### Arch Based Distros

Install it from arch user repository.

```
yaourt -S osm-to-sql
```

### Debian Based Distros

Download the `.deb` file from [here](https://github.com/whizsid/osm-to-sql/releases/download/0.1.2/osm-to-sql_0.1.2_amd64.deb) and install it using `dpkg`.

```
dpkg -i ./osm-to-sql_0.1.1_amd64.deb
```

## Other Distros

Download the standalone binary file from [here](https://github.com/whizsid/osm-to-sql/releases/download/0.1.2/osm-to-sql) and run it.

## Usage

This is the default help menu for the command line tool.

```

Usage:
    osm-to-sql [OPTIONS] -i <xml_file_path.xml> -d <output_directory>

OPTIONS:
    -i        Input open street map file in XML format.
    -d        Output directory to save output sql files.
    -r        Maximum rows per one SQL insert query. [400]
    -h        Prints help information
    -g        Do not use INSERT IGNORE queries

```

## Table mappings

All tables have foreign key constraints and all tables will creating automatically with these SQL files. And please import with the following order when you importing to your database server.

```
    1.nodes
        id,lat,lng,version,changeset,user,uid,visible,date_time

    2.ways
        id,version,changeset,user,uid,visible,date_time

    3.way_nodes
        way_id,node_id
            -- way_id = ways(id)
            -- node_id = nodes(id)

    4.relations
        id,version,changeset,user,uid,visible,date_time

    5.relation_members
        rm_id,relation_id,node_id,way_id,role
            -- relation_id = relations(id)
            -- node_id = nodes(id)
            -- way_id = ways(id)
            -- sub_relation_id = relations(id)

    6.tags
        id,name

    7.ref_tags
        rt_id,tag_id,node_id,relation_id,way_id,value
            -- tag_id = tags(id)
            -- node_id = nodes(id)
            -- relation_id = relations(id)
            -- way_id = ways(id)
 
```

Sample output files in the `sample/output` folder.

## Contirbutions

All contibutions and issues are welcome. Please help me to make this tool faster and powerfull.
