package se.augustocesar.repos;

import picocli.CommandLine;
import se.augustocesar.repos.commands.Repos;

import java.io.File;
import java.io.IOException;


public class CliEntrypoint {
    static void setupSystem() throws IOException {
        var reposDir = new File(Constants.REPOS_DIR_PATH);

        if (reposDir.exists() || reposDir.mkdirs()) {
            var configFile = new File(Constants.CONFIG_FILE_PATH);

            boolean _created = configFile.createNewFile();
        }
    }

    public static void main(String[] args) {
        try {
            setupSystem();
        } catch (IOException e) {
            System.err.println("Failed to setup the repos folder: " + e.getMessage());
            System.exit(1);
        }

        var config = Config.load();

        var repos = new Repos(config);
        var status = new CommandLine(repos).execute(args);

        System.exit(status);
    }
}
