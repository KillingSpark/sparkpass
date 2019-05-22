#!/usr/bin/env bash
# export passwords to external file

shopt -s nullglob globstar
prefix=${PASSWORD_STORE_DIR:-$HOME/.password-store}

echo -n "Password for sparkpass:" 
read -s password

for file in "$prefix"/**/*.gpg; do                           
    file="${file/$prefix//}"
    path="${file%.*}"
    content="$(pass "${file%.*}")" 
    
    echo "$path"

    export SPARKPASS_KEY="$password"

    spass insert "pass_import/$path" \""$content"\"
    
done