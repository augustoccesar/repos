package se.augustocesar.repos;

public class Constants {
    public static String HIDDEN_REPOS_DIR_PATH = System.getenv("HOME") + "/.repos";
    public static String REPOS_DIR_PATH = System.getenv("HOME") + "/repos";

    public static String CONFIG_FILE_PATH = REPOS_DIR_PATH + "/config.toml";
    public static String UPDATE_CACHE_FILE_PATH = HIDDEN_REPOS_DIR_PATH + "/latest_release.json";
}
