package se.augustocesar.repos;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

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

    private void scanDir(String dirPath) {
        var dir = new File(dirPath);
        File[] items = dir.listFiles();

        if (items == null) {
            return;
        }

        var itemsList = Arrays.asList(items);
        if (itemsList.stream().anyMatch(file -> file.getName().equals(".git"))) {
            try {
                repos.add(dir.getCanonicalPath());
            } catch (IOException e) {
                throw new RuntimeException(e);
            }
        } else {
            for (File item : itemsList) {
                if (item.isDirectory()) {
                    scanDir(item.getPath());
                }
            }
        }
    }

    public List<String> getRepos() {
        return repos;
    }
}
