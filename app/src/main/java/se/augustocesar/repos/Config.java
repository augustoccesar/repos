package se.augustocesar.repos;

import com.moandjiezana.toml.Toml;
import com.moandjiezana.toml.TomlWriter;

import java.io.File;
import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Config {
    private final String host;
    private final String username;
    private final String editor;
    private Map<String, Object> index;
    private final Map<String, Object> aliases;

    public Config(
            String host,
            String username,
            String editor,
            Map<String, Object> index,
            Map<String, Object> aliases
    ) {
        this.host = host;
        this.username = username;
        this.editor = editor;
        this.index = index;
        this.aliases = aliases;
    }

    public static Config buildDefault() {
        return new Config(
                Defaults.host(),
                Defaults.username(),
                Defaults.editor(),
                new HashMap<>(),
                new HashMap<>()
        );
    }

    static Config load() {
        File configFile = new File(Constants.CONFIG_FILE_PATH);
        if (!configFile.exists()) {
            return buildDefault();
        }

        Toml toml = new Toml().read(configFile);

        return new Config(
                toml.getString("host"),
                toml.getString("username"),
                toml.getString("editor"),
                getTableAsMap(toml, "index"),
                getTableAsMap(toml, "aliases")
        );
    }

    public void updateIndex(List<String> repoPaths) throws IOException {
        this.index = new HashMap<>();

        for (int i = 0; i < repoPaths.size(); i++) {
            this.index.put(
                    String.valueOf(i),
                    repoPaths.get(i).replace(Constants.REPOS_DIR_PATH + "/", "")
            );
        }

        var writer = new TomlWriter();
        writer.write(this.toMap(), new File(Constants.CONFIG_FILE_PATH));
    }

    // Needs to manually serialize since the Object -> TOML requires reflection, which does not work
    // well on the native and I don't want to dig deep into it rn.
    private Map<String, Object> toMap() {
        var map = new HashMap<String, Object>();

        map.put("host", this.host);
        map.put("username", this.username);
        map.put("editor", this.editor);
        map.put("index", this.index);
        map.put("aliases", this.aliases);

        return map;
    }

    private static Map<String, Object> getTableAsMap(final Toml toml, String key) {
        var table = toml.getTable(key);
        if (table == null) return null;

        return table.toMap();
    }

    public String getHost() {
        return this.host != null ? this.host : Defaults.host();
    }

    public String getUsername() {
        return this.username != null ? this.username : Defaults.username();
    }

    public String getEditor() {
        return this.editor != null ? this.editor : Defaults.editor();
    }

    public Map<String, Object> getIndex() {
        return index;
    }

    public Map<String, Object> getAliases() {
        return aliases;
    }
}
