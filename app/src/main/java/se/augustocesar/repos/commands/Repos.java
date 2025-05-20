package se.augustocesar.repos.commands;

import picocli.CommandLine.Command;
import se.augustocesar.repos.Config;
import se.augustocesar.repos.GitHub;
import se.augustocesar.repos.Version;
import se.augustocesar.repos.VersionProvider;

@Command(
        name = "repos",
        mixinStandardHelpOptions = true,
        versionProvider = VersionProvider.class,
        subcommands = {
                ActivateCommand.class,
                ExpandCommand.class,
                ListCommand.class,
                UpdateCommand.class,
        }
)
public record Repos(Config config, Version version, GitHub github) {
}
