**An enhanced, self-contained fork of the Amberol music player.**

This project is a fork of the beautiful and simple [Amberol music player](https://gitlab.gnome.org/World/amberol). The primary goal of this fork is to simplify the build process, create a single self-contained binary, and to serve as a platform for adding new features and quality-of-life improvements.

Amberol's philosophy is to play music, and nothing else. This fork respects that core principle while enhancing the underlying structure for greater portability and easier development.

## Key Technical Changes

This fork introduces several significant technical modifications to the original Amberol build and configuration process. These changes are designed to create a more streamlined and portable application.

### Refactored Build System

The build process has been completely overhauled and is now managed by a single `build.rs` script. This script handles the compilation of all necessary resources and their integration into the final executable. This approach simplifies the build process, removing the need for external scripts and complex configuration steps.

### Embedded Configuration

A major change in this fork is the elimination of the `config.rs` file. All configuration data is now embedded directly into the binary. This simplifies the application's structure and ensures that the executable is fully self-contained, without reliance on external configuration files.

### Integrated Resources

To create a single, portable executable, all application resources are now embedded directly into the binary. This includes:

*   **Localization files:** All language translations are compiled into the application, removing the need for external `.mo` files.
*   **UI definitions:** The GTK `.ui` files that define the user interface are also embedded, ensuring that the application can run without needing to access external UI definition files.

## Patches and Quality-of-Life Improvements

This fork includes a variety of patches and fixes that have not been merged into the mainline Amberol project. These changes address various minor issues and improve the overall user experience.

## Future Plans (TODO)

The following features are planned for future releases:

*   **Discord Rich Presence (RPC) Integration:** A key planned feature is the integration of Discord RPC to display the currently playing song as your Discord status.
