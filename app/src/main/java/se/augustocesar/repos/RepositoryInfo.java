package se.augustocesar.repos;

import java.util.regex.Pattern;

public record RepositoryInfo(String host, String username, String name) {

    static final Pattern FULL_GIT = Pattern.compile("^git@(.+):(.+)\\/(.+).git$");

    public static RepositoryInfo of(Config config, String input) {
        var fullGitMatcher = FULL_GIT.matcher(input);
        if (fullGitMatcher.matches()) {
            var host = fullGitMatcher.group(1);
            var username = fullGitMatcher.group(2);
            var name = fullGitMatcher.group(3);

            return new RepositoryInfo(host, username, name);
        }

        var tokens = input.split("/");
        if (tokens.length == 3) {
            return new RepositoryInfo(tokens[0], tokens[1], tokens[2]);
        } else if (tokens.length == 2) {
            return new RepositoryInfo(config.host(), tokens[0], tokens[1]);
        } else {
            return new RepositoryInfo(config.host(), config.username(), input);
        }
    }
}
