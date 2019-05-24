# Sparkpass
This is an implementation of a somewhat "pass" compatible password manager.

## Why
Standard pass stores entries with their names in the clear. I liked the concept of pass but I didnt want everyone with read access to see 
what kind of passwords I have stored.

### How is this better
No one can read the names of the entries without knowing the key used while creating the entry.

Note that someone with read access can still see how many passwords there are and how they are grouped.

## How does it work
Like pass it just stores entries in files. These files are encrypted using openssl::symm aes_256 (pass uses gpg for encryption). 
Unlike pass sparkpass encrypts the entry names the same way.

If you want to access the passwords without having to retype the master-password you can use the environment variable
"SPARKPASS_KEY".

If you want to have your repo located somewhere else than the default "$HOME/.sparkpass" use "SPARKPASS_REPO".

If you want to have your passwords copied to the clipboard you can just pipe sparkpass into "xclip" like this:  

``` SPARKPASS_KEY="tHiSiSaSeCuRePaSsWoRd" sparkpass --repo "~/path/to/repo" show ebay.com/pass | xclip -sel clip ```
THIS WILL STORE YOUR KEY IN THE ~/.bash_history IF YOU DONT ENTER A SPACE AT THE START!
You can also use ```unset HISTFILE``` before you enter any command.

Of course there is also an interactive prompt for the key but right now it is visible in clear on the screen.

## Convenient usage
There is a simple daemon that listens on a socket in /tmp/sparkpass. You need to give it the repo password only once.
If it reads "show" from this it will show "rofi -dmenu" with all entries in the repo, and will copy the content of the file into xclip.

After you are done logging into all the things you can kill the daemon and your repo will be secure again.