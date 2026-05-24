# War Thunder Skin Manager

This application is intented to be used as a helper to manage your War Thunder Skins. It works by using the api of [https://live.warthunder.com/](https://live.warthunder.com/). This allows you to install skins as if you were browsing on the website.

When you click on "install" the application will automatically download and extract the archive to your game's skins folder. If the downloaded archive contains several or no folders, one will be created by the manager to avoid cluttering the skins folder.

## Usage

1. Set your game's skins directory
![config](images/config.png)

2. Refine your search with filters
![filters](images/filters.png)

3. Install the skins you want
![filters](images/install-panel.png)

4. Uninstall unwanted skins 
![filters](images/uninstall-panel.png)
_Note: For the moment this feature will only work for skins installed through the manager_

## Technical Aspects

I wrote this software using Rust entierly. 

The crates used are:

- Dioxus for the interface
- Tokio for async
- Serde_Json for, you guessed it, the json
- Reqwest to do http requests to the api and download files
- Directories to save the config in the correct directory for each platform
- Zip to extract the archives

The application makes heavy used of async and threads to keep the ui responsive during downloads and process them in parrallel.

## Disclaimer

I made this project in two weeks after more than a year without touching Rust. My Rust skills were already bad so I struggled to keep the codebase clean and readable. So don't be surprised if the application crashes, or if any dataloss occurs. I am not responsible for either of them ! This software is provided "as is".

## Roadmap

Todo:
- [ ] Clean up the code !!!
- [ ] Separate the logic from the UI
- [ ] Add a settings page
- [ ] Add a search bar

## License
