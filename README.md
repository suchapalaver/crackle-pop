# CracklePop

A program that prints out the numbers 1 to 100 (inclusive).
If the number is divisible by 3, print Crackle instead of the number. 
If it's divisible by 5, print Pop. 
If it's divisible by both 3 and 5, print CracklePop.

Written for an application to attend the
[Recurse Center](https://www.recurse.com/).

## How to Run this Application

### Install Rust and Cargo

If you haven't already, you need to install Rust on your system. 

Visit the official Rust website at https://www.rust-lang.org/ to get installation
instructions for your platform.

### Clone this repository

Cloning a GitHub repository allows you to make a copy of the repository on your
local machine, which you can then work with and contribute to.

Here's a step-by-step guide on how to clone a GitHub repository:

1. **Install Git:**
   If you don't have Git installed on your machine, you'll need to install it first.
   You can download Git from the official website: https://git-scm.com/downloads

2. **Find the Repository:**
   Go to the GitHub repository you want to clone. You can find it by navigating to the
   repository's URL, like `https://github.com/username/repository-name`.

3. **Copy the Repository URL:**
   Click on the "Code" button (it might say "Code" or "Clone" depending on the repository)
   located near the top right of the repository page. This will open a dropdown menu with
   the repository's URL. Click the clipboard icon to copy the URL.

4. **Open Terminal (or Command Prompt on Windows):**
   Open a terminal window on your local machine. You'll use this terminal to run Git commands.

5. **Navigate to the Location:**
   Use the `cd` command to navigate to the directory where you want to clone the repository.
   For example, to clone the repository into your home directory, you can use:
   
   ```bash
   cd ~
   ```

6. **Clone the Repository:**
   In the terminal, use the `git clone` command followed by the repository URL you copied earlier.
   
   For example:
   
   ```bash
   git clone https://github.com/username/repository-name.git
   ```

   Replace `https://github.com/username/repository-name.git` with the actual URL of the repository.

7. **Enter Credentials (if required):**
   If the repository is private or requires authentication, Git might prompt you to enter your
   GitHub username and password or a personal access token. Enter the required information.

8. **Repository Cloned:**
   Git will now clone the repository from GitHub to your local machine. Once the cloning process
   is complete, you'll see a message indicating that the clone was successful.

9. **Navigate to Repository Directory:**
   Use the `cd` command to navigate into the newly created repository directory:
   
   ```bash
   cd repository-name
   ```

10. **Start Working:**
    Now you have a local copy of the GitHub repository on your machine. You can make changes,
    create branches, and interact with the repository using Git commands.

Remember that the process might vary slightly based on your operating system and the tools
you're using, but the general steps should remain the same.

### Compile and run

Open a terminal in the same directory as your cloned `CracklePop` repository and execute the
following command:

```bash
cargo run
```