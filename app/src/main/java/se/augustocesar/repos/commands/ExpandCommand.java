package se.augustocesar.repos.commands;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.util.ArrayDeque;
import java.util.Optional;

import se.augustocesar.repos.Config;
import se.augustocesar.repos.Git;
import se.augustocesar.repos.RepositoryInfo;

public class ExpandCommand implements Command {
    public record Args(String name, boolean shouldClone) {
        public static Args parse(ArrayDeque<String> rawArgs) throws InvalidCommandArg {
            if (rawArgs.isEmpty()) {
                throw new InvalidCommandArg("Missing repository name.");
            }

            var name = rawArgs.pop();
            var clone = false;

            if (!rawArgs.isEmpty()) {
                var arg = rawArgs.pop();

                switch (arg) {
                    case "-c", "--clone":
                        clone = true;

                        break;
                    default:
                        throw new InvalidCommandArg("Invalid arg: " + arg);
                }
            }

            return new Args(name, clone);
        }
    }

    private final Args args;
    private final Config config;

    public ExpandCommand(Args args, final Config config) {
        this.args = args;
        this.config = config;
    }

    @Override
    public Integer run(Appendable output) throws IOException {
        var info = RepositoryInfo.of(this.config, this.args.name);
        if (Files.exists(info.localPath())) {
            output.append(info.localPath().toString()).append('\n');

            return 0;
        }

        if (this.args.shouldClone) {
            output.append("Repository not found locally.").append('\n');
            output.append("Local path: ").append(info.localPath().toString()).append('\n');
            output.append("Git repo: ").append(info.cloneUri()).append('\n');
            output.append("Do you want to clone it? (y, N)").append('\n');

            try {
                var reader = new BufferedReader(new InputStreamReader(System.in));
                var response = reader.readLine();

                if (response != null && response.equalsIgnoreCase("y")) {
                    if (Git.clone(info.cloneUri(), info.localPath().toString())) {
                        output.append(info.localPath().toString()).append('\n');

                        return 0;
                    } else {
                        return 1;
                    }
                } else {
                    output.append("Aborted!").append('\n');

                    return 1;
                }

            } catch (IOException e) {
                output.append("Failed to read input.").append('\n');

                return 1;
            }
        } else {
            output.append("Repo not found locally.").append('\n');

            return 1;
        }
    }

    @Override
    public Optional<String> help() {
        var text = """
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
                """;

        return Optional.of(text);
    }
}
