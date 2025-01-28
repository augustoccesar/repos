function rcd
    # Gradle Testing
    set REPO (./gradlew -q run --args="expand $argv[1] --clone" --rerun-tasks | tee /dev/tty)

    # Compiled Testing
    # set REPO (./app/build/native/nativeCompile/repos expand $argv[1] --clone | tee /dev/tty)

    # Live
    # set REPO (repos expand $argv[1] --clone | tee /dev/tty)

    if test $pipestatus[1] -eq 0
        # Testing
        echo $REPO

        # Live
        # cd $REPO
    end
end