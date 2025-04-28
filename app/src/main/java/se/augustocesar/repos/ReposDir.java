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

    public String displayTree(String filter) {
        var filteredRepos = this.list(filter).stream()
                .map(repo -> {
                    String path = pathFromBase(repo);

                    return path.startsWith("/") ? path.substring(1) : path;
                })
                .sorted()
                .toList();

        Node root = new Node(-1, "root");
        for (String repoPath : filteredRepos) {
            root.add(repoPath);
        }

        StringBuilder tree = new StringBuilder();
        for (int i = 0; i < root.leaves.size(); i++) {
            boolean isLast = i == root.leaves.size() - 1;
            root.leaves.get(i).asTree(tree, "", isLast);
        }

        return tree.toString();
    }

    private String pathFromBase(String repoFullPath) {
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

    private static class Node {
        private final int depth;
        private final String name;
        private final List<Node> leaves;

        private Node(int depth, String name) {
            this.depth = depth;
            this.name = name;
            this.leaves = new ArrayList<>();
        }

        public void add(String repoPath) {
            String[] pathParts = repoPath.split("/");
            add(pathParts, 0);
        }

        private void add(String[] repoPathParts, int index) {
            if (index >= repoPathParts.length) {
                return;
            }

            String part = repoPathParts[index];
            var node = this.leaves.stream().filter(leaf -> leaf.name.equals(part)).findFirst();

            if (node.isPresent()) {
                node.get().add(repoPathParts, index + 1);
            } else {
                var newNode = new Node(this.depth + 1, part);

                this.leaves.add(newNode);

                newNode.add(repoPathParts, index + 1);
            }
        }

        public void asTree(StringBuilder builder, String prefix, boolean isLast) {
            if (depth >= 0) {
                builder.append(prefix)
                        .append(isLast ? "└─ " : "├─ ")
                        .append(name)
                        .append("\n");
            }

            for (int i = 0; i < leaves.size(); i++) {
                boolean lastChild = i == leaves.size() - 1;
                leaves.get(i).asTree(
                        builder,
                        prefix + (isLast ? "   " : "│  "),
                        lastChild
                );
            }
        }
    }
}
