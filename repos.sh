rcd() {
    # Gradle Testing
    REPO=$(set -o pipefail && ./gradlew -q run --args="expand $1 --clone" --rerun-tasks | tee /dev/tty)

    # Compiled Testing
    # REPO=$(set -o pipefail && ./app/build/native/nativeCompile/repos expand $1 --clone | tee /dev/tty)

    # Live
    # REPO=$(set -o pipefail && repos expand $1 --clone | tee /dev/tty)

    if [ $? -eq 0 ]; then
        # Testing
        echo $REPO

        # Live
        # cd $REPO
    fi
}
