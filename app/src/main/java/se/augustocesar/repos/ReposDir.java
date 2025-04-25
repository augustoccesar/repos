package se.augustocesar.repos;

import java.io.File;
import java.io.IOException;
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
                repo -> repo
                        .replace(Constants.REPOS_DIR_PATH, "")
                        .toLowerCase()
                        .contains(filter.toLowerCase())
        ).collect(Collectors.toList());
    }

    public String displayTree(String filter) {
        var filteredRepos = this.list(filter).stream()
                .map(repo -> repo.replace(Constants.REPOS_DIR_PATH, "").substring(1))
                .sorted()
                .toList();

        Node node = Node.init(-1, "root");
        for (String repoPath : filteredRepos) {
            node.add(repoPath);
        }

        StringBuilder tree = new StringBuilder();
        node.asTree(tree, false, true);

        return tree.toString();
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

    private record Node(int depth, String name, ArrayList<Node> leaves) {
        public static Node init(int depth, String name) {
            return new Node(depth, name, new ArrayList<>());
        }

        public void add(String repoPath) {
            ArrayDeque<String> pathParts = new ArrayDeque<>(Arrays.asList(repoPath.split("/")));

            var part = pathParts.pop();

            var node = this.leaves.stream().filter(leave -> leave.name.equals(part)).findFirst();
            if (node.isPresent()) {
                node.get().add(String.join("/", pathParts));
            } else {
                var newNode = Node.init(this.depth + 1, part);
                if (!pathParts.isEmpty()) {
                    newNode.add(String.join("/", pathParts));
                }

                this.leaves.add(newNode);
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
