#!/usr/bin/bash

OSM_FILE_DOWNLOAD_URL="https://download.geofabrik.de/asia/bhutan-latest.osm.bz2" 
TEST_DATA_DIR=test_data
OSM_FILE_ARCHIVE_NAME=bhutan-latest.osm.bz2
OSM_FILE_NAME=bhutan-latest.osm
ROWS_PER_QUERY=500

echo "INFO: Creating Test Data Directory."
mkdir -p $TEST_DATA_DIR || { echo "ERROR: Can not create $TEST_DATA_DIR directory"; exit 1; }
echo "INFO: Checking the weather OSM archive exist."
if [ -f $TEST_DATA_DIR/$OSM_FILE_ARCHIVE_NAME ]; then
	echo "INFO: OSM archive exists."
else
	echo "INFO: OSM archive doesn't exist. Downloading.."
	wget $OSM_FILE_DOWNLOAD_URL -P $TEST_DATA_DIR || { echo "ERROR: Download failed"; exit 1; }
	echo "INFO: Downloaded an OSM archive"
fi

if ! command -v bzip2 &> /dev/null
then
	echo "ERROR: bzip2 command not found. Please install.";
	exit 1;
fi

echo "INFO: Checking the weather OSM XML file exist or not."
if [ -f $TEST_DATA_DIR/$OSM_FILE_NAME ]; then
	echo "INFO: OSM XML file exist."
else
	echo "INFO: OSM XML file not exist. Extracting the archive."
	bzip2 -k -d $TEST_DATA_DIR/$OSM_FILE_ARCHIVE_NAME || { echo "ERROR: Can not extract the archive"; exit 1; }
fi

echo "INFO: Creating the output directory."
mkdir -p $TEST_DATA_DIR/output || { echo "ERROR: Can not create the output directory"; exit 1; }

echo "INFO: Cleaning the output directory"
rm -rf $TEST_DATA_DIR/output/*

echo "INFO: Compiling and running the osm-to-sql Command."
cargo run --release -- -i $TEST_DATA_DIR/$OSM_FILE_NAME -d $TEST_DATA_DIR/output -r $ROWS_PER_QUERY || { echo "ERROR: Can not run the osm-to-sql command."; exit 1; }

PASSED=0
TOTAL=0

function assert_count {
	TAG_NAME=$1
	SQL_FILE_NAME=$2

	ACTUAL_COUNT=$(fgrep -o "<$TAG_NAME" $TEST_DATA_DIR/$OSM_FILE_NAME | wc -l)
	((SQL_LINE_COUNT=($ACTUAL_COUNT+$ROWS_PER_QUERY-1)/$ROWS_PER_QUERY ))
	SQL_SEPARATOR_COUNT=$(fgrep -o "),(" $TEST_DATA_DIR/output/$SQL_FILE_NAME.sql | wc -l)
	((SQL_COUNT=$SQL_LINE_COUNT+$SQL_SEPARATOR_COUNT))
	((TOTAL=$TOTAL+1))
	if (( $SQL_COUNT == $ACTUAL_COUNT )); then
		((PASSED=$PASSED+1))
		echo "TEST$TOTAL: PASSED $SQL_FILE_NAME count $SQL_COUNT==$ACTUAL_COUNT"
	else
		echo "TEST$TOTAL: FAILED $SQL_FILE_NAME count $SQL_COUNT==$ACTUAL_COUNT"
	fi

}

echo "INFO: Testing nodes count"
assert_count "node" "nodes"
echo "INFO: Testing ways count"
assert_count "way" "ways"
echo "INFO: Testing relations count"
assert_count "relation" "relations"
echo "INFO: Testing relation_members count"
assert_count "member" "relation_members"
echo "INFO: Testing way_nodes count."
assert_count "nd" "way_nodes"
echo "INFO: Testing ref_tags count."
assert_count "tag" "ref_tags"

