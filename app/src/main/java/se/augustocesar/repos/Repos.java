package se.augustocesar.repos;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.util.*;

import com.fasterxml.jackson.databind.ObjectMapper;

import se.augustocesar.repos.commands.ExpandCommand;
import se.augustocesar.repos.commands.InvalidCommandArg;
import se.augustocesar.repos.commands.ListCommand;

public class Repos {
    static void setupSystem() throws IOException {
        var reposDir = new File(Constants.REPOS_DIR_PATH);

        if (reposDir.exists() || reposDir.mkdirs()) {
            var configFile = new File(Constants.CONFIG_FILE_PATH);

            if (configFile.exists() || configFile.createNewFile()) {
                try (var writer = new BufferedWriter(new FileWriter(configFile))) {
                    writer.write("{\n}");
                }
            }
        }
    }

    public static void main(String[] args) {
        try {
            setupSystem();
        } catch (IOException e) {
            System.err.println("Failed to setup the repos folder: " + e.getMessage());
            System.exit(1);
        }

        ObjectMapper mapper = new ObjectMapper();
        var config = Config.load(mapper);

        var argsDeque = new ArrayDeque<>(Arrays.asList(args));

        while (!argsDeque.isEmpty()) {
            var arg = argsDeque.pop();

            try {
                switch (arg) {
                    case "expand":
                        var expandCommandArgs = ExpandCommand.Args.parse(argsDeque);
                        var expandCommand = new ExpandCommand(expandCommandArgs, config);

                        if (argsDeque.contains("--help")) {
                            System.out.println(expandCommand.help());
                            System.exit(0);
                        } else {
                            expandCommand.run(System.out);
                        }

                        break;
                    case "list":
                        var listCommandArgs = ListCommand.Args.parse(argsDeque);
                        var listCommand = new ListCommand(listCommandArgs);

                        if (argsDeque.contains("--help")) {
                            System.out.println(listCommand.help());
                            System.exit(0);
                        } else {
                            int status = listCommand.run(System.out);
                            System.exit(status);
                        }

                        break;
                    case "--help":
                        var text = """
                                Commands:
                                  - expand: Expands the repository name to the full path.
                                  - list: List all the available repositories.
                                """;

                        System.out.println(text);

                        break;
                    default:
                        System.err.println("Invalid arg: " + arg);

                        System.exit(1);
                }
            } catch (InvalidCommandArg | IOException e) {
                System.err.println(e.getMessage());

                System.exit(1);
            }
        }
    }
}
