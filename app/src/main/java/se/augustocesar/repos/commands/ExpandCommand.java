package se.augustocesar.repos.commands;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.util.concurrent.Callable;

import picocli.CommandLine.ParentCommand;
import picocli.CommandLine.Command;
import picocli.CommandLine.Parameters;
import picocli.CommandLine.Option;
import se.augustocesar.repos.Constants;
import se.augustocesar.repos.Git;
import se.augustocesar.repos.RepositoryInfo;

@Command(name = "expand", mixinStandardHelpOptions = true, description = "Expands the passed on repository name to the full path.")
public class ExpandCommand implements Callable<Integer> {
    @ParentCommand
    private Repos reposCommand;

    @Parameters(index = "0", description = """
            Name or index of the repository to expand.
            
            The index of a repository can be checked on the config.toml file or by running `repos list`.
            
            For cases where the fields are not all present on the name, they will be resolved by:
            
            host:
                1. What is on the `host` of the config.toml.
                2. Default to "github.com".
            
            username:
                1. What is on the `username` of the config.toml.
                2. Default to 'user.name' system property.
            
            Supported formats:
                - @{index}
                - git@{host}:{username}/{repository}.git
                - {host}/{username}/{repository}
                - {username}/{repository}
                - {repository}
            """)
    String name;

    @Option(names = {"-c", "--clone"}, description = "Clone the repository if not exists locally.")
    boolean shouldClone;

    @Option(names = {"-y"}, description = "Skip prompts by automatic confirming.")
    boolean skipPrompt;

    @Override
    public Integer call() {
        if (this.name.equals("@")) {
            System.out.println(Constants.REPOS_DIR_PATH);
            
            return 0;
        }
        
        var info = RepositoryInfo.of(this.reposCommand.config(), this.name);
        if (Files.exists(info.localPath())) {
            System.out.println(info.localPath());

            return 0;
        }

        if (this.shouldClone) {
            try {
                if (!this.skipPrompt) {
                    System.out.println("Repository not found locally.");
                    System.out.println("Local path: " + info.localPath());
                    System.out.println("Git repository: " + info.cloneUri());
                    System.out.println("Do you want to clone it? (y, N)");

                    var reader = new BufferedReader(new InputStreamReader(System.in));
                    var response = reader.readLine();

                    if ((response == null || !response.equalsIgnoreCase("y"))) {
                        System.out.println("Aborted!");

                        return 1;
                    }
                }

                if (Git.clone(info.cloneUri(), info.localPath().toString())) {
                    System.out.println(info.localPath());

                    return 0;
                } else {
                    return 1;
                }
            } catch (IOException e) {
                System.out.println("Failed to read input.");

                return 1;
            }
        } else {
            System.out.println("Repository not found locally.");

            return 1;
        }
    }
}
