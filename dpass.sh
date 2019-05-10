#! /bin/bash

RES="$(target/debug/sparkpass ls $1 | rofi -dmenu)"

NAME=$(echo $RES | cut -d " " -f 2)

# "recursion" into deeper sub directories
[[ "${RES}" =~ ^DIR ]] && ./dpass.sh "$1"/"${NAME}"

# end result into clipboard
[[ "${RES}" =~ ^ENT ]] && target/debug/sparkpass show "$1"/$NAME | xclip -sel clip