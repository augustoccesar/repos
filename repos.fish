function rcd
    set REPO (repos expand $argv[1] --clone | tee /dev/tty)

    if test $pipestatus[1] -eq 0
        cd $REPO
    end
end