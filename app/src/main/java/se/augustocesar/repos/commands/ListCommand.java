package se.augustocesar.repos.commands;

import picocli.CommandLine.Command;
import picocli.CommandLine.Option;
import se.augustocesar.repos.ReposDir;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.Callable;

@Command(name = "list", mixinStandardHelpOptions = true, description = "List all the available repositories.")
public class ListCommand implements Callable<Integer> {
    @Option(names = {"-f", "--filter"}, description = "Text to look for on repositories path.")
    String filter;

    private final Appendable output;

    public ListCommand(final Appendable output) {
        this.output = output;
    }

    @Override
    public Integer call() throws IOException {
        output.append(this.displayTree(this.filter));

        return 0;
    }

    public String displayTree(String filter) {
        ReposDir reposDir = ReposDir.load();

        var filteredRepos = reposDir.list(filter).stream()
                .map(repo -> {
                    String path = ReposDir.pathFromBase(repo);

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
