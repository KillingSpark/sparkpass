#! /bin/bash

pipe="/tmp/sparkpass"

trap "rm -f $pipe" EXIT

rm -f $pipe

if [[ ! -p $pipe ]]; then
    mkfifo $pipe
fi

export SPARKPASS_KEY=$1

if [[ $2 != '' ]]; then
    export SPARKPASS_REPO=$2
fi

while true
do
    if read line <$pipe; then
        if [[ "$line" == 'quit' ]]; then
            break
        fi

        if [[ "$line" == 'show' ]]; then
            cargo run --bin spass -- -i show  $(cargo run --bin spass -- -i ls -t | rofi -dmenu) | xclip -sel clip
        fi
        echo "Received: "$line
    fi
done

rm -f $pipe