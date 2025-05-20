package se.augustocesar.repos;

import java.util.Arrays;
import java.util.List;

public enum OS {
    macOS;

    public static OS fromSystem() {
        String osName = System.getProperty("os.name");

        for (var os : OS.values()) {
            if (os.possibleNames().contains(osName)) {
                return os;
            }
        }

        return null;
    }

    public List<String> possibleNames() {
        return switch (this) {
            case macOS -> Arrays.asList("Mac OS X", "apple-darwin", "darwin");
        };
    }

    public String releaseName() {
        return switch (this) {
            case macOS -> "apple-darwin";
        };
    }
}
