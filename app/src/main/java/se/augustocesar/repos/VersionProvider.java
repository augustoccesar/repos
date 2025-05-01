package se.augustocesar.repos;

import picocli.CommandLine;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

public class VersionProvider implements CommandLine.IVersionProvider {
    @Override
    public String[] getVersion() {
        try (InputStream stream = getClass().getResourceAsStream("/version.properties")) {
            Properties props = new Properties();
            props.load(stream);

            String version = props.getProperty("version", "unknown");

            return new String[]{version};
        } catch (IOException e) {
            return new String[]{"unknown"};
        }
    }
}
