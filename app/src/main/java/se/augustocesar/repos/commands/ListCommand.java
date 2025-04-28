package se.augustocesar.repos.commands;

import se.augustocesar.repos.ReposDir;

import java.io.IOException;
import java.util.*;

public class ListCommand implements Command {
    public record Args(String filter) {
        public static Args parse(ArrayDeque<String> rawArgs) throws InvalidCommandArg {
            String filter = null;
            if (!rawArgs.isEmpty()) {
                var arg = rawArgs.pop();
                switch (arg) {
                    case "-f", "--filter":
                        try {
                            filter = rawArgs.pop();

                            break;
                        } catch (NoSuchElementException e) {
                            throw new InvalidCommandArg("Missing value for filter");
                        }
                    default:
                        throw new InvalidCommandArg("Invalid arg: " + arg);
                }
            }

            return new Args(filter);
        }
    }

    private final Args args;

    public ListCommand(Args args) {
        this.args = args;
    }

    @Override
    public Integer run(Appendable output) throws IOException {
        output.append(this.displayTree(this.args.filter));

        return 0;
    }

    @Override
    public Optional<String> help() {
        var help = """
                List all the available repositories.
                
                Args:
                  - filter: Text to look for on repositories path.
                """;

        return Optional.of(help);
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
