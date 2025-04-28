package se.augustocesar.repos;

import com.moandjiezana.toml.Toml;

import java.io.File;
import java.util.HashMap;

public class Config {
    private final String host;
    private final String username;
    private final HashMap<String, Object> index;

    public Config(String host, String username, HashMap<String, Object> index) {
        this.host = host;
        this.username = username;
        this.index = index;
    }

    public static Config buildDefault() {
        return new Config(Defaults.host(), Defaults.username(), new HashMap<>());
    }

    static Config load() {
        File configFile = new File(Constants.CONFIG_FILE_PATH);
        if (!configFile.exists()) {
            return buildDefault();
        }

        return new Toml().read(configFile).to(Config.class);
    }

    public String getHost() {
        return this.host != null ? this.host : Defaults.host();
    }

    public String getUsername() {
        return this.username != null ? this.username : Defaults.username();
    }

    public HashMap<String, Object> getIndex() {
        return index;
    }
}
