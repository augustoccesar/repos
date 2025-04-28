package se.augustocesar.repos.commands;

import picocli.CommandLine.ParentCommand;
import picocli.CommandLine.Command;
import picocli.CommandLine.Option;
import se.augustocesar.repos.ReposDir;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.Callable;

import static se.augustocesar.repos.ReposDir.pathFromBase;

@Command(name = "list", mixinStandardHelpOptions = true, description = "List all the available repositories.")
public class ListCommand implements Callable<Integer> {
    @ParentCommand
    private Repos reposCommand;

    @Option(names = {"-f", "--filter"}, description = "Text to look for on repositories path.")
    String filter;

    @Override
    public Integer call() {
        System.out.println(this.displayTree(this.filter));

        return 0;
    }

    public String displayTree(String filter) {
        ReposDir reposDir = ReposDir.load();
        List<String> repoPaths = reposDir.getRepos();

        try {
            this.reposCommand.config().updateIndex(repoPaths);
        } catch (IOException e) {
            // TODO: Log
        }

        Node root = Node.root();
        for (int i = 0; i < repoPaths.size(); i++) {
            String pathFromBase = pathFromBase(repoPaths.get(i));
            if (filter == null || pathFromBase.toLowerCase().contains(filter.toLowerCase())) {
                root.add(
                        String.valueOf(i),
                        pathFromBase.startsWith("/") ? pathFromBase.substring(1) : pathFromBase
                );
            }
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
        private final String index;
        private final String name;
        private final boolean showIndex;
        private final List<Node> leaves;

        private Node(int depth, String index, String name, boolean showIndex) {
            this.depth = depth;
            this.index = index;
            this.name = name;
            this.showIndex = showIndex;
            this.leaves = new ArrayList<>();
        }

        public static Node root() {
            return new Node(-1, "-", "root", false);
        }

        public void add(String index, String repoPath) {
            String[] pathParts = repoPath.split("/");
            add(index, pathParts, 0);
        }

        private void add(String index, String[] repoPathParts, int pathPartIndex) {
            if (pathPartIndex >= repoPathParts.length) {
                return;
            }

            String part = repoPathParts[pathPartIndex];
            var node = this.leaves.stream().filter(leaf -> leaf.name.equals(part)).findFirst();

            if (node.isPresent()) {
                node.get().add(index, repoPathParts, pathPartIndex + 1);
            } else {
                var newNode = new Node(this.depth + 1, index, part, pathPartIndex == repoPathParts.length - 1);

                this.leaves.add(newNode);

                newNode.add(index, repoPathParts, pathPartIndex + 1);
            }
        }

        public void asTree(StringBuilder builder, String prefix, boolean isLast) {
            if (depth >= 0) {
                builder.append(prefix)
                        .append(isLast ? "└─ " : "├─ ");

                if (this.showIndex) {
                    builder.append("(").append(this.index).append(") ");
                }

                builder.append(name)
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
