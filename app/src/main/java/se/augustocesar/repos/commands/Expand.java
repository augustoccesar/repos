package se.augustocesar.repos.commands;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.util.concurrent.Callable;

import picocli.CommandLine.Command;
import picocli.CommandLine.Option;
import picocli.CommandLine.Parameters;
import se.augustocesar.repos.Config;
import se.augustocesar.repos.Git;
import se.augustocesar.repos.RepositoryInfo;

@Command(name = "expand", mixinStandardHelpOptions = true, description = """
        This command expands the passed on repository name to the full path.

        For cases where the fields are not all present on the name, they will be resolved by:

            host:
                1. What is on the `default_host` of the config.
                2. Default to "github.com".

            username:
                1. What is on the `default_username` of the config.
                2. Default to whoami::username()

            Supported formats:
                - git@{host}:{username}/{repo}.git
                - {host}/{username}/{repo}
                - {username}/{repo}
                - {repo}
        """)
public class Expand implements Callable<Integer> {
    @Parameters(index = "0", description = "repository name on one of the supported formats")
    String name;

    @Option(names = { "-c", "--clone" }, description = "clone the repository if not exists locally")
    boolean clone;

    private Config config;

    public Expand(final Config config) {
        this.config = config;
    }

    @Override
    public Integer call() {
        var info = RepositoryInfo.of(this.config, this.name);
        if (Files.exists(info.localPath())) {
            System.out.println(info.localPath());
            return 0;
        }

        if (this.clone) {
            Git.clone(info.cloneUri(), info.localPath().toString());
            System.out.println(info.localPath());
            return 0;
        }

        System.out.println("Repository not found locally.");
        System.out.println("Looking at: " + info.localPath());
        System.out.println("Remote repository: " + info.cloneUri());
        System.out.println("Do you want to clone it? (y, N)");

        try {
            var reader = new BufferedReader(new InputStreamReader(System.in));
            var response = reader.readLine();

            if (response != null && response.toLowerCase().equals("y")) {
                Git.clone(info.cloneUri(), info.localPath().toString());
                System.out.println(info.localPath());
            } else {
                System.out.println("Exiting...");
            }
        } catch (IOException e) {
            System.err.println("Failed to read input.");
            return 1;
        }

        return 0;
    }
}
