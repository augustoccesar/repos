package se.augustocesar.repos.commands;

import java.util.concurrent.Callable;

import picocli.CommandLine.Command;
import picocli.CommandLine.Parameters;
import se.augustocesar.repos.Config;
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

    private Config config;

    public Expand(final Config config) {
        this.config = config;
    }

    @Override
    public Integer call() {
        System.out.println(RepositoryInfo.of(this.config, this.name));

        return 0;
    }
}
