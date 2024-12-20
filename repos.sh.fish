###begin:repos
# repos expand will return the path of the repo locally and this
# function will only be responsible for cd'ing into the folder if successful
# or print the output if it fails.
function rcd
    set OUTPUT (repos expand $argv[1])
    set EXIT $status

    if test $EXIT -eq 0
        cd $OUTPUT
    else if test $EXIT -eq 8
        repos expand $argv[1] --clone
        if test $status -eq 0
            set OUTPUT (repos expand $argv[1])
            if test $status -eq 0
                cd $OUTPUT
            else
                echo $OUTPUT
            end
        end
    else
        echo $OUTPUT
    end
end
###end:repos