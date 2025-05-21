package se.augustocesar.repos;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Optional;
import java.util.logging.Logger;

public class Git {
    private static final Logger logger = Logger.getLogger(Git.class.getName());

    public static boolean clone(final String url, final String target) {
        try {
            var processBuilder = new ProcessBuilder("git", "clone", url, target).inheritIO();
            var process = processBuilder.start();

            return process.waitFor() == 0;
        } catch (IOException e) {
            System.err.println("Failed to create process to clone repository.");

            return false;
        } catch (InterruptedException e) {
            System.err.println("Interrupted cloning.");

            return false;
        }
    }

    public static Optional<String> getRemote() {
        try {
            var processBuilder = new ProcessBuilder("git", "remote", "get-url", "origin");
            Process process = processBuilder.start();

            var output = new StringBuilder();
            try (BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()))) {
                String line;
                while ((line = reader.readLine()) != null) {
                    output.append(line);
                }
            }

            var outputStr = output.toString();
            if (process.waitFor() == 0 && !outputStr.isBlank()) {
                return Optional.of(output.toString());
            } else {
                return Optional.empty();
            }
        } catch (IOException | InterruptedException e) {
            logger.severe("Failed to resolve the repository remote: " + e);

            return Optional.empty();
        }
    }
}
