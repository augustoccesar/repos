package se.augustocesar.repos;

// Simple implementation. I guess if I need something more robust at some point
// can go with something like https://github.com/zafarkhaja/jsemver/.
// Might have pre-release at some point, but for now will stay with just major,
// minor and patch.
public record Version(int major, int minor, int patch) implements Comparable<Version> {
    public static class InvalidFormat extends Exception {
        public InvalidFormat() {
            super("Invalid format. Expected X.Y.Z where X, Y and Z are positive integers.");
        }
    }

    public static Version parse(final String value) throws InvalidFormat {
        if (value == null) {
            return null;
        }

        String[] parts = value.split("\\.");
        if (parts.length != 3) {
            throw new InvalidFormat();
        }

        try {
            int major = Integer.parseInt(parts[0]);
            int minor = Integer.parseInt(parts[1]);
            int patch = Integer.parseInt(parts[2]);

            if (major < 0 || minor < 0 || patch < 0) {
                throw new InvalidFormat();
            }

            return new Version(major, minor, patch);
        } catch (NumberFormatException e) {
            throw new InvalidFormat();
        }
    }

    public boolean isNewerThan(final Version other) {
        return this.compareTo(other) > 0;
    }

    public boolean isOlderThan(final Version other) {
        return this.compareTo(other) < 0;
    }

    @Override
    public int compareTo(Version other) {
        int majorComparison = Integer.compare(this.major, other.major);
        if (majorComparison != 0) {
            return majorComparison;
        }

        int minorComparison = Integer.compare(this.minor, other.minor);
        if (minorComparison != 0) {
            return minorComparison;
        }

        return Integer.compare(this.patch, other.patch);
    }

    @Override
    public String toString() {
        return String.format("%d.%d.%d", this.major, this.minor, this.patch);
    }
}
