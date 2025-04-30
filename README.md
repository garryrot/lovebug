# Telekinesis

Bluetooth Toy Integration & Bone Tracking for Fallout4

Some of you might already know [Bluetooth Toy Integration for Skyrim](https://github.com/garryrot/telekinesis), this is the correspsonding Fallou4 Port.

## Features

- AAF Integration (With Mesh Collision/Bone Tracking support): Move devices based on the currently played AAF scene. By default, it will detect collisions with player body parts and move the devices, tagged with matching body parts, accordingly. *WARNING*: This is (and always will be) an experimental proof of concept, I do not take any responsobility of you use it on your body. 

- Devious Devices Integration: Controls devices based on In-Game Vibration or Inflation Events, for devices worn by the Player Character.

- Device Config from within MCM: Enable/Disable devices and tag them to body-parts from within the MCM.

## Installation/Quickstart

Telekinesis connects to your devices using the operating systems bluetooth controller, in the default `In-Process` Connection this happens from within from this mods F4S4-plugin-dll and needs no other software.

0. Install Dependencies:
    - Address Library for F4SE
    - Mod Configuration Menu
    - AAF v1.7 or higher (optional)
    - Devious Devices RC9 (optional, if other versions work, report back)
1. Install `Telekinesis-FO4-$VERSION.zip` with your mod manager, select
2. Pair/Connect your bluetooth toys in your System Control
3. Start Fallout4
4. Enable your devices in the Telekinesis MCM, once they are connected.
5. Trigger any AAF scene or Devious Devices Inflating/Vibrating Plug Event

# FAQ

## Where Are all the Settings

Most settings are json files `Fallout4\Data\F4SE\Telekineis\*`, you can find the documentation [here](./docs/3-Config.md)

## Bug Reports

If anything fails or behaves in an unexpected way, include the Papyrus logs `Pyprus.0.log` and the Logs of this plugin (`%USERPROFILE%/My Games/Fallout4/F4S4/Telekinesis.log`)

## License

This mod is free software and can be used under the terms of the [Apache License V2](LICENSE) 

## Changelog

### 2.0.0.rc3

#### Triggers

- Support trigger through Actor Values

#### Bone Tracking

- Activate lowest-possible default vibration during bone-tracked scenes, until first penetration has happened

- Add more custom race presets for races that don't work out of the box
    - Ghouls
    - Synths
    - Mutant Hounds (FEVHound)

- Body Type Support
    - Adapt fomod installer: FusionGirl also works for CBBE
    - Include race preset for "Male Super Hero Body" (the NFSW version, untested)
