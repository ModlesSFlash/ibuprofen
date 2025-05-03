# ibuprofen
a media search utility that aims to cure your media search headaches

https://github.com/user-attachments/assets/2405f550-4ae9-401b-840f-a1284c3809bb

ibuprofen takes your query and lets you search for images/gifs on Google & Tenor. click on anything you deem worthy and it'll be imported straight into your (currently open) project. here's what it looks like:

https://github.com/user-attachments/assets/7bed1db1-b367-4ffa-84c8-84f904f433eb

currently only supports DaVinci Resolve when it comes to importing files but i'm already looking at PP/AE to let my friends use this too!

treat this as a proof of concept. it has to be refactored for easier integration of new gif/img/svg sources. if it looks like it was made by some idiot who doesn't know how to code - it was made by some idiot who doesn't know how to code (it was made with love though!)

unfortunately for 3 people with eyes out there as soon as i made it work i gave up on making things pretty...
but still! feel free to suggest/implement stuff - ibuprofen seems like a huge time saver so maybe i'll work on your suggestions ...now that i can save some free time with this util 

# requiremenets
google/tenor API token - https://developers.google.com/tenor/guides/quickstart#setup
(you'll need to create a tenor account using your existing google one)

google custom search engine ID - https://programmablesearchengine.google.com/controlpanel/all
(don't forget to enable "image search")

curl - https://curl.se/download.html

ffmpeg - https://ffmpeg.org/download.html

powershell

# how to install
- create a folder and put ibuprofen in there. make sure you have ffmpeg in the same folder (or in your PATH)
- run ibuprofen and go through the initial config setup
- if you're using DaVinci Resolve - put ibuprofen-listener-DVR.py script into the directory below. the script will handle media import for you (if you're not a DVR user - you'll have to deal with the downloaded files yourself for now)
```
%appdata%\Blackmagic Design\DaVinci Resolve\Support\Fusion\Scripts\Utility\
```

# how to run
- open any project in DaVinci Resolve (and run the listener script through Workspace->Scripts)
- run a query in ibuprofen and click on any thumbnail, it should get downloaded into the folder you selected (and imported into the media pool)

# how to build
you'd need nodejs/[npm](https://nodejs.org/en) and rust/[cargo](https://rustup.rs/)
0. clone
```
git clone https://github.com/modlessflash/ibuprofen
```
1. install tauri-cli & build the project
```
cargo install tauri-cli
cargo tauri build --no-bundle
```
