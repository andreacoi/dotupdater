# Dotfile Updater

Dotfile Updater is a simple and efficient tool that automatically updates your dotfiles from a Git repository every time your system starts. This program saves you from the hassle of manually updating your configurations, ensuring your development environment is always in sync with your latest settings.

## Features

- **Automatic Updates:** Sync your dotfiles with a specified Git repository automatically on system startup.
- **Log File:** All update operations are logged to a file for tracking and debugging purposes.
- **Customizable Paths and Repositories:** Configure the directories and Git repository links through a `.toml` configuration file.
- **Simple Setup:** Easy installation and configuration—just point to your repositories and folders, and you're ready to go.

## Installation

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/yourusername/dotfile-updater.git
   ```

2. Navigate to the project directory:

   ```bash
   cd dotfile-updater
   ```

3. Follow the installation instructions for your operating system (e.g., setting it to run on startup).

## Configuration

The program relies on a `.toml` configuration file to specify the directories to be updated and the corresponding Git repository URLs. A sample configuration file (`config.toml`) is provided in the repository.

Example of `config.toml`:

```toml
[repos]
repo1 = "https://github.com/yourusername/dotfiles.git"
repo2 = "https://github.com/yourusername/other-dotfiles.git"

[folders]
folder1 = "/home/username/.config"
folder2 = "/home/username/.dotfiles"
```
