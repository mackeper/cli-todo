# Setup

The windows installer is build using [Inno Setup](https://jrsoftware.org/isinfo.php). 

## Build the windows installer
1. First build the program with release flags.
2. Install Inno Setup, either by downloading it from the website or using chocolatey.
    a. `choco install innosetup`
3. Run the Inno Setup GUI
4. Open the script `setup.iss`
    a. Update the version if needed.
5. Compile it.
6. The installer should now be at `targets/release/setup`
