#! /bin/bash

pipe="/tmp/sparkpass"

if [ ! -p  $pipe ]; then
    echo "Socket is not open. Start the deamon first" | rofi -dmenu
fi

echo "show" > /tmp/sparkpass