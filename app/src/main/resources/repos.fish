function __repos_expand_with_clone
    set output (repos expand --clone $argv | tee /dev/tty)

    if test $pipestatus[1] -ne 0
        return 1
    end

    set lines (string split \n -- $output | string trim --right)
    for i in (seq (count $lines) -1 1)
        if test -n "$lines[$i]"
            echo $lines[$i]
            return 0
        end
    end

    return 1
end

function rcl
    __repos_expand_with_clone $argv[1] > /dev/null
end

function rop
    set REPO (repos expand -m remote $argv[1])

    if test $status -eq 0
        # TODO: Use xdg-open for Linux, when it is added as an option.
        open $REPO
    end
end

function rcd
    set REPO (__repos_expand_with_clone $argv[1])

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
    set REPO (__repos_expand_with_clone $argv[1])

    if test $status -eq 0
        @EDITOR@ $REPO
    end
end
