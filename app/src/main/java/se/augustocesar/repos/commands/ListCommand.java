package se.augustocesar.repos.commands;

import se.augustocesar.repos.ReposDir;

import java.util.*;

public class ListCommand implements Command {
    public record Args(String filter) {
        public static Args parse(ArrayDeque<String> rawArgs) throws InvalidCommandArg {
            String filter = null;
            if (!rawArgs.isEmpty()) {
                var arg = rawArgs.pop();
                switch (arg) {
                    case "-f", "--filter":
                        try {
                            filter = rawArgs.pop();

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
        String output = ReposDir.load().displayTree(this.args.filter);

        System.out.println(output);

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
