package se.augustocesar.repos;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

import com.fasterxml.jackson.databind.ObjectMapper;

import picocli.CommandLine;
import picocli.CommandLine.Command;
import se.augustocesar.repos.commands.Expand;

@Command(name = "repos", mixinStandardHelpOptions = true)
public class Repos {
    static void setupSystem() throws IOException {
        new File(Constants.REPOS_DIR_PATH).mkdirs();

        var configFile = new File(Constants.CONFIG_FILE_PATH);
        if (!configFile.exists()) {
            configFile.createNewFile();

            try (var writer = new BufferedWriter(new FileWriter(configFile))) {
                writer.write("{\n}");
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

        var commandLine = new CommandLine(new Repos());
        commandLine.addSubcommand("expand", new Expand(config));

        var exitCode = commandLine.execute(args);
        System.exit(exitCode);
    }
}
