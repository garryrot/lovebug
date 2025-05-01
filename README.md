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

## Nothing happens (In General)

Check the following:

- Did you enable your device in MCM settings page (Devices)?
- Try to vibrate the device through Intiface Central, if that doesn't work consult Buttplug.io/Intiface Documentation.
- Try to vibrate the device through MCM Debug page to check that the fallout4 plugin is working.

## Things work anywhere expect in AAF animations

By default, things only happen if the bone monitoring detects an oral or anal/vaginal penetration.

If it doesn't, you can do one of the following steps:

- Try different animations, if it never works, you probably don't use a supported body type, read Telekinesis.log `%USERPROFILE%/My Games/Fallout4/F4S4/Telekinesis.log`

- If specific animations don't work:
     - Define your own trigger.json

- It is possible to define custom behavior for each scene, that will disable bone tracking and use a custom device movement.
   
    - See NukaRide and BP70 triggers in the mod files for examples on how to do this
- If no collision happens, you can find out why by reading , it will dump stats ont he collision data and whether bones are found on your body type.

## Where Are all the Settings

Most settings are json files `Fallout4\Data\F4SE\Telekineis2\*`, you can find the documentation [here](./docs/3-Config.md)

## Bug Reports

If anything fails or behaves in an unexpected way, include the Papyrus logs `Pyprus.0.log` and the Logs of this plugin (`%USERPROFILE%/My Games/Fallout4/F4S4/Telekinesis.log`)

## License

This mod is free software and can be used under the terms of the [Apache License V2](LICENSE) 

## Changelog

### 2.0.0.rc3

Did a lot of things under the hood, mainly added the ability to trigger actions based on actor value changes and some internal improvements.

Note: Calling these "rc (release candidate)" is kind of a stretch, but I commited to it. Once its stable I will switch to regular versioning.

#### Triggers

- Support triggers through Actor Values
    - Support logical conditions on actor values i.e. START if "x > 5 and y == 5" STOP if "x < 5" etc...

#### Bone Tracking

- Activate lowest-possible default vibration during bone-tracked scenes, until first penetration has happened so that people aren't confused

- Add more custom race presets for races that don't work out of the box
    - Ghouls
    - Synths
    - Mutant Hounds (FEVHound)

- Body Type Support
    - Adapt fomod installer: FusionGirl also works for CBBE
    - Include race preset for "Male Super Hero Body" (the NFSW version, untested)

#### Bugfixes

- Fixed some issue that caused debug actions to not work due to string capitalization
- Many more I've forgot about probably
