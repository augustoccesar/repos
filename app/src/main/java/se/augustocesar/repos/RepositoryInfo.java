package se.augustocesar.repos;

import java.nio.file.Path;
import java.util.regex.Pattern;

public record RepositoryInfo(String host, String username, String name) {

    static final Pattern FULL_GIT = Pattern.compile("^git@(.+):(.+)/(.+).git$");
    static final Pattern INDEX = Pattern.compile("^@(\\d+)$");

    public static RepositoryInfo of(Config config, String input) {
        if (config.getAliases() != null) {
            var aliased = config.getAliases().get(input);
            if (aliased != null) {
                input = (String) aliased;
            }
        }

        var indexMatcher = INDEX.matcher(input);
        if (indexMatcher.matches()) {
            var indexInput = indexMatcher.group(1);
            var index = config.getIndex();
            if (index != null) {
                String path = (String) index.get(indexInput);
                if (path != null) {
                    input = path;
                }
            }
        }

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
            return new RepositoryInfo(config.getHost(), tokens[0], tokens[1]);
        } else {
            return new RepositoryInfo(config.getHost(), config.getUsername(), input);
        }
    }

    public Path localPath() {
        return Path.of(Constants.REPOS_DIR_PATH, this.host, this.username, this.name);
    }

    public String cloneUri() {
        return "git@" + this.host + ":" + this.username + "/" + this.name;
    }

    public String remoteUri() {
        return "https://" + this.host + "/" + this.username + "/" + this.name;
    }
}
