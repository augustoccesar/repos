package se.augustocesar.repos;

import java.io.File;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.util.*;

public class ReposDir {
    private final List<String> repos;

    private ReposDir() {
        this.repos = new ArrayList<>();
    }

    public static ReposDir load() {
        var reposDir = new ReposDir();
        reposDir.scanDir(Constants.REPOS_DIR_PATH);

        return reposDir;
    }

    public static String pathFromBase(String repoFullPath) {
        return repoFullPath.startsWith(Constants.REPOS_DIR_PATH)
                ? repoFullPath.substring(Constants.REPOS_DIR_PATH.length())
                : repoFullPath;
    }

    public List<String> getRepos() {
        return repos;
    }

    private void scanDir(String dirPath) {
        var dir = new File(dirPath);

        if (hasGitFolder(dir)) {
            try {
                repos.add(dir.getCanonicalPath());
            } catch (IOException e) {
                throw new UncheckedIOException(e);
            }
            return;
        }

        File[] items = dir.listFiles();
        if (items == null) {
            return;
        }

        for (File item : items) {
            if (item.isDirectory()) {
                scanDir(item.getPath());
            }
        }
    }

    private boolean hasGitFolder(File dir) {
        File[] items = dir.listFiles();
        if (items == null) {
            return false;
        }

        return Arrays.stream(items).anyMatch(file -> file.getName().equals(".git"));
    }
}
