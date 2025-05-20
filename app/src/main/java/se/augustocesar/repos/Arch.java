package se.augustocesar.repos;

import java.util.Arrays;
import java.util.List;

public enum Arch {
    arm64;

    public static Arch fromSystem() {
        String osArch = System.getProperty("os.arch");

        for (var arch : Arch.values()) {
            if (arch.possibleNames().contains(osArch)) {
                return arch;
            }
        }

        return null;
    }


    public List<String> possibleNames() {
        return switch (this) {
            case arm64 -> Arrays.asList("arm64", "aarch64");
        };
    }

    public String releaseName() {
        return switch (this) {
            case arm64 -> "aarch64";
        };
    }
}
