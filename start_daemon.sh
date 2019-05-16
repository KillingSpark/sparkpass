#! /bin/bash
echo "Enter key"
read -s key

setsid ./sparkpass_daemon.sh $key >/dev/null 2>&1 < /dev/null &
echo "Started"