package se.augustocesar.repos;

public class Defaults {
    public static String username() {
        return System.getProperty("user.name");
    }

    public static String host() {
        return "github.com";
    }

    public static String editor() {
        return "code";
    }
}
