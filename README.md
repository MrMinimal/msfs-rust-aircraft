# General

Microsofts MSFS SDK Sample GaugeAircraft in Rust 🦀

Because I didn't want to write logic in Microsoft's cursed RPN language, XML or C++.

# Requirements
1. Git for Windows (with Git Bash)
1. Rust
1. MSFS SDK

# Developing

### Compile the Rust logic
1. `cd Sources/Rust`
1. `cargo build -v --target wasm32-wasip1 --release && wasm-opt -O1 --signext-lowering --enable-bulk-memory -o ../../../../../Packages/mycompany-aircraft-wasm-gauge/SimObjects/Airplanes/MyCompany_Gauge_Aircraft/panel/nvg_demo.wasm target/wasm32-wasip1/release/nvg_demo.wasm`

### Open project in MSFS
1. Enable MSFS developer mode
1. File -> Open Project -> open the rust_aircraft.xml project file in this repo
1. Debug -> Console
1. Disable the "Autoset" checkbox in the top right of the console and disable any filters
1. Editors -> Project Editor
1. Clear all messages in Console
1. Press the "Build All" button in the Project Editor
1. Search for the "Gauge Aircraft" to fly

### Updating code
1. Change something in the Rust code and build using the command above
1. Project Manager: File -> close project (optional but makes reload way faster)
1. Tools -> Behaviours
1. In the Behaviours window: "Quick reload" (rebuilds the project if it wasn't closed before. Rebuild only required if file naming or project structure changes)

### Checking WASM state
1. Start a flight with the "GaugeAicraft" that should now be available in the sim
1. Debug -> Display WASM Debug Window
1. Select the .wasm file from the dropdown


# License

Microsoft Flight Simulator © Microsoft Corporation. This project was created under Microsoft's "Game Content Usage Rules" using assets from Microsoft Flight Simulator, and it is not endorsed by or affiliated with Microsoft.

The contents of distribution packages built from the sources in this repository are therefore licensed as follows:
    in the case of original source code from FBW or compiled artifacts generated from it, under GPLv3.
    in the case of assets covered by the "Game Content Usage Rules", under the license granted by those rules.

