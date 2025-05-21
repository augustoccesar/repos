package se.augustocesar.repos.commands;

import picocli.CommandLine;
import picocli.CommandLine.Command;
import se.augustocesar.repos.Constants;
import se.augustocesar.repos.Git;
import se.augustocesar.repos.RepositoryInfo;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Optional;
import java.util.concurrent.Callable;

@Command(
        name = "track",
        mixinStandardHelpOptions = true,
        description = "Move repository under the repos tracked structure."
)
public class TrackCommand implements Callable<Integer> {
    @CommandLine.ParentCommand
    private Repos reposCommand;

    @Override
    public Integer call() throws IOException {
        Optional<String> remote = Git.getRemote();

        if (remote.isEmpty()) {
            System.err.println("""
                    Could not resolve the repository remote.
                    Are you sure that you are on a repository path and that it contains a 'origin' remote?
                    """);

            return 1;
        }

        String workingDir = System.getProperty("user.dir");
        if (workingDir.startsWith(Constants.REPOS_DIR_PATH)) {
            System.err.println("The repository is already under repos directory.");

            return 1;
        }

        RepositoryInfo repo = RepositoryInfo.of(reposCommand.config(), remote.get());

        if (Files.exists(repo.localPath())) {
            System.out.println("The repository already exists under repos folder: " + repo.localPath());

            return 1;
        }

        System.out.println("This command will move the repository:");
        System.out.println("From: " + workingDir);
        System.out.println("To:   " + repo.localPath());
        System.out.println("Do you want to continue? (y,N)");

        var reader = new BufferedReader(new InputStreamReader(System.in));
        var response = reader.readLine();

        if ((response == null || !response.equalsIgnoreCase("y"))) {
            System.out.println("Aborted!");

            return 1;
        }

        Files.createDirectories(repo.localPath().getParent());
        Files.move(Path.of(workingDir), repo.localPath());
        System.out.println("Done! 🎉");

        return 0;
    }
}
