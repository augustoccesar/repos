function __expand_repo_with_clone --argument repo
    set REPO (repos expand $repo --clone | tee /dev/tty)

    if test $pipestatus[1] -eq 0
        echo $REPO

        return 0
    else
        return 1
    end
end

function rcd
    set REPO (__expand_repo_with_clone $argv[1])

    if test $status -eq 0
        cd $REPO
    end
end

function rll
    if test -z "$argv[1]"
        repos list
    else
        repos list -f $argv[1]
    end
end

function red
    set REPO (__expand_repo_with_clone $argv[1])

    if test $status -eq 0
        @EDITOR@ $REPO
    end
end
