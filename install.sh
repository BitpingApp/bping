
#!/usr/bin/env bash

set -e

# Function to detect OS and architecture
detect_platform() {
    local os arch

    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)

    case "$os" in
        "linux")
            case "$arch" in
                "x86_64")
                    echo "bping-x86_64-linux-gnu"
                    ;;
                "aarch64"|"arm64")
                    echo "bping-aarch64-linux-gnu"
                    ;;
                "armv7l")
                    echo "bping-armv7-linux-gnueabihf"
                    ;;
                "armv6l"|"arm")
                    echo "bping-arm-linux-gnueabi"
                    ;;
                "i686"|"i386")
                    echo "bping-i686-linux-gnu"
                    ;;
                *)
                    echo "Unsupported architecture: $arch" >&2
                    exit 1
                    ;;
            esac
            ;;
        "darwin")
            case "$arch" in
                "x86_64")
                    echo "bping-x86_64-darwin"
                    ;;
                "arm64")
                    echo "bping-aarch64-darwin"
                    ;;
                *)
                    echo "Unsupported architecture: $arch" >&2
                    exit 1
                    ;;
            esac
            ;;
        *)
            echo "Unsupported operating system: $os" >&2
            exit 1
            ;;
    esac
}

# Function to get the latest release URL
get_latest_release() {
    local asset_name="$1"
    local release_url="https://api.github.com/repos/BitpingApp/bping/releases/latest"
    
    # Get the download URL for the specific asset
    curl -s "$release_url" | \
        grep -o "\"browser_download_url\": \"[^\"]*$asset_name\"" | \
        cut -d'"' -f4
}


# Function to find the best installation directory
find_install_dir() {
    # Standard binary locations, in order of preference
    local preferred_dirs=(
        "$HOME/.local/bin"
        "$HOME/bin"
        "/usr/local/bin"
    )

    # Check preferred directories if they're in PATH and writable
    for dir in "${preferred_dirs[@]}"; do
        if [[ ":$PATH:" == *":$dir:"* ]] && [ -w "$dir" ] && [[ "$dir" != *"/opt/"* ]]; then
            echo "$dir"
            return 0
        fi
    done

    # If no suitable directory found, create and use ~/.local/bin
    local user_bin="$HOME/.local/bin"
    mkdir -p "$user_bin"
    echo "$user_bin"
    return 0
}

# Main installation
main() {
    echo "Detecting platform..."
    local asset_name=$(detect_platform)
    echo "Detected asset: $asset_name"

    echo "Getting latest release URL..."
    local download_url=$(get_latest_release "$asset_name")
    
    if [ -z "$download_url" ]; then
        echo "Failed to find download URL for $asset_name" >&2
        exit 1
    fi

    echo "Downloading bping..."
    local tmp_dir=$(mktemp -d)
    curl -L "$download_url" -o "$tmp_dir/bping"

    echo "Finding suitable installation directory..."
    local install_dir=$(find_install_dir)
    echo "Selected installation directory: $install_dir"

    echo "Installing bping..."
    chmod +x "$tmp_dir/bping"
    
    if [ ! -d "$install_dir" ]; then
        mkdir -p "$install_dir"
    fi

    mv "$tmp_dir/bping" "$install_dir/bping"

    # Check if the installation directory is in PATH
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        echo "Adding $install_dir to PATH..."
        
        # Detect shell and configure PATH
        local shell_profile
        if [[ $SHELL == */zsh ]]; then
            shell_profile="$HOME/.zshrc"
        elif [[ $SHELL == */bash ]]; then
            if [[ "$OSTYPE" == "darwin"* ]]; then
                shell_profile="$HOME/.bash_profile"
            else
                shell_profile="$HOME/.bashrc"
            fi
        else
            shell_profile="$HOME/.profile"
        fi

        echo "export PATH=\"\$PATH:$install_dir\"" >> "$shell_profile"
        echo "Please run 'source $shell_profile' or start a new terminal session to use bping"
    fi

    # Cleanup
    rm -rf "$tmp_dir"

    echo "bping has been installed to $install_dir/bping"
    if command -v bping >/dev/null 2>&1; then
        echo "Installation complete! Try running 'bping --help'"
    else
        echo "Installation complete! Please restart your terminal or run 'source $shell_profile', then try 'bping --help'"
    fi
}

main
