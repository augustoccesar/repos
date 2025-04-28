package se.augustocesar.repos;

import org.tomlj.Toml;
import org.tomlj.TomlParseResult;

import java.io.IOException;
import java.nio.file.Path;

public record Config(String host, String username) {
    public static Config buildDefault() {
        return new Config(Defaults.host(), Defaults.username());
    }

    static Config load() {
        try {
            TomlParseResult result = Toml.parse(Path.of(Constants.CONFIG_FILE_PATH));
            if (result.hasErrors()) {
                // TODO: Logging
                return buildDefault();
            }

            String configHost = result.getString("host");
            String configUsername = result.getString("username");

            return new Config(
                    configHost == null ? Defaults.host() : configHost,
                    configUsername == null ? Defaults.username() : configUsername
            );
        } catch (IOException e) {
            // TODO: Logging
            return buildDefault();
        }
    }
}
