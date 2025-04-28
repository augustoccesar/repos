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
                .map(repo -> pathFromBase(repo).substring(1))
                .sorted()
                .toList();

        Node node = new Node(-1, "root");
        for (String repoPath : filteredRepos) {
            node.add(repoPath);
        }

        StringBuilder tree = new StringBuilder();
        node.asTree(tree, false, true);

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

        public void asTree(StringBuilder current, boolean isLast, boolean isParentLast) {
            char linePrefix = isLast ? '└' : '├';
            String parentPrefix = isParentLast ? "  " : "│ ";

            if (this.depth >= 0) {
                if (this.depth > 0) {
                    current.append(" ");
                    current.append(parentPrefix.repeat(this.depth - 1));
                    current.append(linePrefix).append(" ");
                }

                current.append(this.name).append("\n");
            }

            for (int i = 0; i < this.leaves.size(); i++) {
                boolean isLastLeaf = i == this.leaves.size() - 1;
                this.leaves.get(i).asTree(current, isLastLeaf, isLast);
            }
        }
    }
}
