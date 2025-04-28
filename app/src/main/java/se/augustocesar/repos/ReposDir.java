package se.augustocesar.repos;

import java.io.File;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.util.*;
import java.util.stream.Collectors;

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

    public List<String> list(String filter) {
        if (filter == null || filter.isBlank()) {
            return repos;
        }

        return repos.stream().filter(
                repo -> pathFromBase(repo)
                        .toLowerCase()
                        .contains(filter.toLowerCase())
        ).collect(Collectors.toList());
    }

    public static String pathFromBase(String repoFullPath) {
        return repoFullPath.startsWith(Constants.REPOS_DIR_PATH)
                ? repoFullPath.substring(Constants.REPOS_DIR_PATH.length())
                : repoFullPath;
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
