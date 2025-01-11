package se.augustocesar.repos;

import java.io.File;
import java.io.IOException;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.ObjectMapper;

public record Config(
        @JsonProperty("host") String host,
        @JsonProperty("username") String username) {

    public static Config buildDefault() {
        return new Config(Defaults.host(), Defaults.username());
    }

    static Config load(final ObjectMapper objectMapper) {
        try {
            var configFromFile = objectMapper.readValue(new File(Constants.CONFIG_FILE_PATH), Config.class);

            String host;
            if (configFromFile.host == null || configFromFile.host == "") {
                host = Defaults.host();
            } else {
                host = configFromFile.host;
            }

            String username;
            if (configFromFile.username == null || configFromFile.username == "") {
                username = Defaults.username();
            } else {
                username = configFromFile.username;
            }

            return new Config(host, username);
        } catch (IOException e) {
            // TODO: Log this error
            return Config.buildDefault();
        }
    }
}
