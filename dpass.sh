#! /bin/bash

RES="$(target/debug/sparkpass ls $1 | dmenu)"

NAME=$(echo $RES | cut -d " " -f 2)

[[ "${RES}" =~ ^DIR ]] && ./dpass.sh "$1"/"${NAME}"

[[ "${RES}" =~ ^ENT ]] && target/debug/sparkpass show "$1"/$NAME | xclip -sel clip