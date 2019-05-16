#! /bin/bash

pipe="/tmp/sparkpass"

trap "rm -f $pipe" EXIT

if [[ ! -p $pipe ]]; then
    mkfifo $pipe
fi

export SPARKPASS_KEY=$1

while true
do
    if read line <$pipe; then
        if [[ "$line" == 'quit' ]]; then
            break
        fi

        if [[ "$line" == 'show' ]]; then
            cargo run -- -i show  $(cargo run -- -i ls -t | rofi -dmenu) | xclip -sel clip
        fi
        echo "Received: "$line
    fi
done

echo "daemon exit"