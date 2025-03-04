#!/bin/sh

if command -v -- "repos" >/dev/null 2>&1; then
    printf '%s\n' "repos is already installed." 1>&2
    exit 0
fi

OS=$(uname -s)
ARCH=$(uname -m)

FETCH_OS=''
FETCH_ARCH=''
case "$OS" in
Darwin*)
    FETCH_OS='apple-darwin'
    case "$ARCH" in
    arm64 | aarch64)
        FETCH_ARCH='aarch64'
        ;;
    x86_64)
        FETCH_ARCH='x86_64'
        ;;
    esac
    ;;
Linux*)
    FETCH_OS='unknown-linux-gnu'
    case "$ARCH" in
    arm64 | aarch64)
        FETCH_ARCH='aarch64'
        ;;
    x86_64)
        FETCH_ARCH='x86_64'
        ;;
    esac
    ;;
esac

if [ -z "$FETCH_OS" ] || [ -z "$FETCH_ARCH" ]; then
    printf '%s\n' "Unsupported OS/Arch combination: $OS/$ARCH" 1>&2
    exit 1
fi

LOOKUP_FILE_DOWNLOAD_URL="https://github.com/augustoccesar/repos/releases/download/.*/repos-.*-$FETCH_ARCH-$FETCH_OS.tar.gz"
FILE_DOWNLOAD_URL=$(
    curl -sL \
        -H "Accept: application/vnd.github+json" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/repos/augustoccesar/repos/releases/latest |
        grep -Eio "$LOOKUP_FILE_DOWNLOAD_URL"
)

if [ -z "$FILE_DOWNLOAD_URL" ]; then
    printf '%s\n' "Could not find file with pattern '$LOOKUP_FILE_DOWNLOAD_URL' on the latest GitHub release." 1>&2
    exit 1
fi

printf '%s\n' "Downloading: $FILE_DOWNLOAD_URL" 1>&2
curl -sLO --output-dir "/tmp" $FILE_DOWNLOAD_URL

LOCAL_FILE_PATH="/tmp/$(basename $FILE_DOWNLOAD_URL)"

printf '%s\n' "Decompressing $LOCAL_FILE_PATH" 1>&2
tar -xzf $LOCAL_FILE_PATH -C /tmp

mkdir -p $HOME/.repos/bin
mv /tmp/repos $HOME/.repos/bin/
printf '%s\n' "repos installed on $HOME/.repos/bin/repos" 1>&2

rm "$LOCAL_FILE_PATH"

case ":$PATH:" in
*":$HOME/.repos/bin:"*)
    # PATH already contains the directory
    ;;
*)
    SHELL_NAME=$(basename "$SHELL")
    case "$SHELL_NAME" in
    fish)
        # TODO: Create file $HOME/.config/fish/config.d/repos.fish with the content `source "$HOME/.repos/repos.fish"`
        ;;
    *)
        # TODO: Show the user what they should source in their shell. Something like: "Add `source "$HOME/.repos/repos.sh"`" to .zshrc"
        ;;
    esac
    ;;
esac
