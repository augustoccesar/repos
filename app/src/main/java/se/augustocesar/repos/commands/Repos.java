package se.augustocesar.repos.commands;

import picocli.CommandLine.Command;
import se.augustocesar.repos.Config;

@Command(name = "repos", mixinStandardHelpOptions = true, subcommands = {ExpandCommand.class, ListCommand.class})
public record Repos(Config config) {
}
