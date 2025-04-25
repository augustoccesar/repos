package se.augustocesar.repos.commands;

import java.util.ArrayDeque;
import java.util.NoSuchElementException;
import java.util.Optional;

public class ListCommand implements Command {
    public record Args(Optional<String> filter) {
        public static Args parse(ArrayDeque<String> rawArgs) throws InvalidCommandArg {
            Optional<String> filter = Optional.empty();
            if (!rawArgs.isEmpty()) {
                var arg = rawArgs.pop();
                switch (arg) {
                    case "-f", "--filter":
                        try {
                            var filterValue = rawArgs.pop();
                            filter = Optional.of(filterValue);

                            break;
                        } catch (NoSuchElementException e) {
                            throw new InvalidCommandArg("Missing value for filter");
                        }
                    default:
                        throw new InvalidCommandArg("Invalid arg: " + arg);
                }
            }

            return new Args(filter);
        }
    }

    private final Args args;

    public ListCommand(Args args) {
        this.args = args;
    }

    @Override
    public Integer call() {
        System.out.println("List command");
        System.out.println(this.args.filter);

        return 0;
    }

    @Override
    public Optional<String> help() {
        var help = """
                List all the available repositories.
                
                Args:
                  - filter: Text to look for on repositories path.
                """;

        return Optional.of(help);
    }
}
