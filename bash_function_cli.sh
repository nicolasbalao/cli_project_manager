#!/bin/sh

function pm_cli(){
    output=$(/home/nicolas/dev/rust/cli_project_manager/target/debug/cli_project_manager "$@")

    if [ -d "$output" ]; then
        cd "$output" 
    else
        echo "$output"
        exit 0
    fi
}
