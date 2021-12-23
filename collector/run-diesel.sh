#!/bin/bash

# ===============================================================
# Nifty Bash program to run migrations for all collectors at once
#
#  Usage: run-diesel.sh <start> <end> <...diesel arguments>
#
#  For example: run-diesel.sh 5 10 migration run
#    Runs migrations for collectors 5 to 10
# ===============================================================

# Check for arguments
if [ $# -lt 3 ]; then
   echo "Usage: $0 <start> <end> <...diesel arguments>";
   return;
fi;

# Read the start and end
START=$1
NUM_COLLECTORS=$2
shift 2

# Load the .env file, if it exists
#  Handle all variable expansion and special characters
if [ -f .env ]; then
   export $(echo $(grep -v '^#' .env | awk '/=/ {print $1}') | envsubst)
fi

# Loop over all collectors
for i in $(seq $START $NUM_COLLECTORS); do
   echo "Handle Collector $i...";

   # Parse the URL from the collector variables
   url=$(eval "echo \$C${i}_DATABASE_URL");
   diesel --database-url "$url" $@

   # Stop the loop if one of the commands fails
   if [ $? -ne 0 ]; then
      break;
   fi
done
