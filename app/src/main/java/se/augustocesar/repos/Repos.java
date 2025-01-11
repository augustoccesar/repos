package se.augustocesar.repos;

import picocli.CommandLine;
import picocli.CommandLine.Command;
import se.augustocesar.repos.commands.Expand;

@Command(name = "repos", mixinStandardHelpOptions = true)
public class Repos {
    public static void main(String[] args) {
        var commandLine = new CommandLine(new Repos());
        commandLine.addSubcommand("expand", new Expand());

        var exitCode = commandLine.execute(args);
        System.exit(exitCode);
    }
}
