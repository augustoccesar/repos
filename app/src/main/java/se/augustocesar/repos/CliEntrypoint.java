package se.augustocesar.repos;

import picocli.CommandLine;
import se.augustocesar.repos.commands.Repos;

import java.io.File;
import java.io.IOException;
import java.util.Optional;

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

        try {
            var gitHub = new GitHub();
            var config = Config.load();

            String providedVersion = new VersionProvider().getVersion()[0];
            var version = Version.parse(providedVersion);

            var repos = new Repos(config, version, gitHub);

            CommandLine cmd = new CommandLine(repos);
            CommandLine.ParseResult result = cmd.parseArgs(args);

            String subcommand = result.subcommand() != null
                    ? result.subcommand().commandSpec().name()
                    : null;

            if (!"update".equals(subcommand)) {
                Optional<GitHub.Release> latestRelease = gitHub.fetchLatestRelease(OS.fromSystem(), Arch.fromSystem(), true);
                latestRelease.ifPresent(release -> {
                    if (release.version().isNewerThan(version)) {
                        System.out.println(Color.YELLOW.apply("New version available! Use 'repos update' to update the CLI.\n"));
                    }
                });
            }

            int status = cmd.execute(args);
            System.exit(status);
        } catch (Version.InvalidFormat e) {
            System.err.println("Failed to resolve CLI version.");
            System.exit(1);
        }
    }
}
